use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashMap;

use xmlparser::Token as XmlToken;
use xmlparser::{TextUnescape, XmlSpace};

use parser::*;
use names::*;
use support::Facets;
use primitives::{QName,NcName,AnyUri,NonNegativeInteger};

pub const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

fn parse_min_occurs(x: &Option<NonNegativeInteger>) -> usize {
    match x {
        None => 1,
        Some(n) => n.0 as usize,
    }
}
fn parse_max_occurs(x: &Option<unions::UnionNonNegativeIntegerNmtoken>) -> usize {
    match x {
        None => 1,
        Some(unions::UnionNonNegativeIntegerNmtoken::NonNegativeInteger(n)) => n.0 as usize,
        Some(unions::UnionNonNegativeIntegerNmtoken::Nmtoken(restrictions::Unbounded(_))) => usize::max_value(),
    }
}

fn vec_concat_opt<T: Clone>(vector: &Vec<T>, value: Option<T>) -> Vec<T>{
    let mut vector2: Vec<T> = vector.clone();
    if let Some(v) = value {
        vector2.push(v);
    }
    vector2
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[must_use]
pub struct Documentation<'input>(Vec<&'input str>);
impl<'input> Documentation<'input> {
    pub fn new() -> Documentation<'input> {
        Documentation(Vec::new())
    }
    pub fn extend(&mut self, v: &Documentation<'input>) {
        self.0.extend(v.0.iter());
    }
}

impl<'input> ToString for Documentation<'input> {
    fn to_string(&self) -> String {
        self.0.iter().map(|doc| TextUnescape::unescape(doc, XmlSpace::Default)).collect::<Vec<_>>().join("\n")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttrUse {
    Prohibited,
    Required,
    Optional,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attrs<'input> {
    pub named: Vec<(FullName<'input>, AttrUse, Option<SimpleType<'input>>)>,
    pub refs: Vec<(Option<FullName<'input>>, AttrUse, FullName<'input>)>,
    pub group_refs: Vec<FullName<'input>>,
    pub any_attributes: bool,
}
impl<'input> Attrs<'input> {
    pub fn new() -> Attrs<'input> {
        Attrs { named: Vec::new(), refs: Vec::new(), group_refs: Vec::new(), any_attributes: false }
    }
    fn extend(&mut self, other: Attrs<'input>) {
        let Attrs { named, refs, group_refs, any_attributes } = other;
        self.named.extend(named);
        self.refs.extend(refs);
        self.group_refs.extend(group_refs);
        self.any_attributes |= any_attributes;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RichAstNode<'input, T: Debug + Hash + PartialEq + Eq + PartialOrd + Ord, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> {
    pub attrs: Attrs<'input>,
    pub type_: T,
    pub doc: Documentation<'input>,
    pub data: D,
}
impl<'input, T: Debug + Hash + PartialEq + Eq + PartialOrd + Ord, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> RichAstNode<'input, T, D> {
    pub fn new(type_: T, doc: Documentation<'input>) -> RichAstNode<'input, T, D> {
        RichAstNode { attrs: Attrs::new(), type_, doc, data: D::default() }
    }
}
impl<'input, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> RichAstNode<'input, Type<'input>, D> {
    fn add_attrs(mut self, new_attrs: Attrs<'input>) -> RichAstNode<'input, Type<'input>, D> {
        self.attrs.extend(new_attrs);
        self
    }
}

impl<'input, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> RichAstNode<'input, SimpleType<'input>, D> {
    pub fn into_complex(self) -> RichAstNode<'input, Type<'input>, D> {
        let RichAstNode { attrs, type_, doc, data } = self;
        RichAstNode { attrs, type_: Type::Simple(type_), doc, data }
    }
}

/// A reference to a type, that can be part of another type/element/...
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InlineComplexType<'input, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> {
    min_occurs: usize,
    max_occurs: usize,
    type_: RichAstNode<'input, Type<'input>, D>,
}
impl<'input, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> InlineComplexType<'input, D> {
    fn new(min_occurs: usize, max_occurs: usize, type_: RichAstNode<'input, Type<'input>, D>) -> InlineComplexType<'input, D> {
        InlineComplexType { min_occurs, max_occurs, type_ }
    }
}

/// A reference to a type, that can be part of another type/element/...
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InlineSimpleType<'input, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> {
    type_: RichAstNode<'input, SimpleType<'input>, D>,
}
impl<'input, D: Debug + Hash + PartialEq + Eq + PartialOrd + Ord + Default> InlineSimpleType<'input, D> {
    fn new(type_: RichAstNode<'input, SimpleType<'input>, D>) -> InlineSimpleType<'input, D> {
        InlineSimpleType { type_ }
    }

    fn into_complex(self) -> InlineComplexType<'input, D> {
        InlineComplexType { min_occurs: 0, max_occurs: 0, type_: self.type_.into_complex() }
    }
}


#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type<'input> {
    Any,
    Empty,
    Alias(FullName<'input>),
    Extension(FullName<'input>, Box<InlineComplexType<'input, ()>>),
    Restriction(FullName<'input>, Box<InlineComplexType<'input, ()>>),
    ElementRef(FullName<'input>),
    Element(FullName<'input>),
    Group(FullName<'input>),
    Choice(Vec<Vec<InlineComplexType<'input, ()>>>),
    //             ^- a field of a variant
    //         ^- a variant (anonymous struct)
    //     ^- list of variants
    Sequence(Vec<InlineComplexType<'input, ()>>),
    Simple(SimpleType<'input>),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleType<'input> {
    Alias(FullName<'input>),
    Restriction(FullName<'input>, Facets<'input>),
    List(Box<InlineSimpleType<'input, ()>>),
    Union(Vec<InlineSimpleType<'input, ()>>),
    Empty,
}

#[derive(Debug)]
pub struct Processor<'ast, 'input: 'ast> {
    pub target_namespace: Option<&'input str>,
    pub element_form_default_qualified: bool,
    pub attribute_form_default_qualified: bool,
    pub elements: HashMap<FullName<'input>, InlineComplexType<'input, ()>>,
    pub simple_types: HashMap<FullName<'input>, (InlineSimpleType<'input, ()>, Documentation<'input>)>,
    pub groups: HashMap<FullName<'input>, InlineComplexType<'input, ()>>,
    pub attribute_groups: HashMap<FullName<'input>, Attrs<'input>>,

    pub substitution_groups: HashMap<FullName<'input>, Vec<FullName<'input>>>,
    _phantom: PhantomData<&'ast ()>, // Sometimes I need 'ast when prototyping
}

impl<'ast, 'input: 'ast> Processor<'ast, 'input> {
    pub fn new(ast: &'ast xs::Schema<'input>) -> Processor<'ast, 'input> {
        let target_namespace = ast.attr_target_namespace.as_ref().map(|t| t.0);
        let element_form_default_qualified = match ast.attr_element_form_default.as_ref().map(|x| ((x.0).0).0) {
            Some("qualified") => true,
            Some("unqualified") | None => false,
            _ => unreachable!(),
        };
        let attribute_form_default_qualified = match ast.attr_attribute_form_default.as_ref().map(|x| ((x.0).0).0) {
            Some("qualified") => true,
            Some("unqualified") | None => false,
            _ => unreachable!(),
        };
        Processor {
            target_namespace,
            element_form_default_qualified,
            attribute_form_default_qualified,
            elements: HashMap::new(),
            groups: HashMap::new(),
            attribute_groups: HashMap::new(),
            simple_types: HashMap::new(),
            substitution_groups: HashMap::new(),
            _phantom: PhantomData::default(),
        }
    }

    pub fn process_ast(&mut self, ast: &'ast xs::Schema<'input>) {
        for top_level_item in ast.sequence_schema_top_annotation.iter() {
            match top_level_item.schema_top {
                xs::SchemaTop::Redefinable(ref r) => self.process_redefinable(r),
                xs::SchemaTop::Element(ref e) => { self.process_toplevel_element(e); },
                xs::SchemaTop::Attribute(_) => unimplemented!("top-level attribute"),
                xs::SchemaTop::Notation(ref e) => self.process_notation(e),
            }
        }
    }

    fn process_notation(&mut self, notation: &'ast xs::Notation<'input>) {
        // TODO
    }

    fn process_redefinable(&mut self, r: &'ast xs::Redefinable<'input>) {
        match r {
            xs::Redefinable::SimpleType(ref e) => { self.process_simple_type(e); },
            xs::Redefinable::ComplexType(e) => { self.process_complex_type(e); },
            xs::Redefinable::Group(e) => { self.process_named_group(e); },
            xs::Redefinable::AttributeGroup(e) => self.process_attribute_group(e),
        }
    }

    fn process_annotation(&self, annotation: &Vec<&'ast xs::Annotation<'input>>) -> Documentation<'input> {
        let strings = annotation.iter().flat_map(|xs::Annotation { ref attrs, ref attr_id, ref annotation_content }| {
            annotation_content.iter().filter_map(|c| {
                match c {
                    enums::AnnotationContent::Appinfo(_) => None,
                    enums::AnnotationContent::Documentation(e) => {
                        let xs::Documentation { ref attrs, ref attr_source, ref sequence_any } = **e;
                        Some(sequence_any.iter().flat_map(|sequences::SequenceAny { any }| {
                            any.0.iter().filter_map(|tok| {
                                match tok {
                                    XmlToken::Text(s) => Some(s.to_str()),
                                    _ => None,
                                }
                            })
                        }))
                    },
                }
            })
        }).flat_map(|v| v).collect();
        Documentation(strings)
    }

    fn process_group_ref(&mut self, 
            group_ref: &'ast inline_elements::GroupRef<'input>,
            ) -> InlineComplexType<'input, ()> {
        let inline_elements::GroupRef { ref attrs, ref attr_id, ref attr_ref, ref attr_min_occurs, ref attr_max_occurs, ref annotation } = group_ref;
        let ref_ = attr_ref;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);

        let ref_ = FullName::from_qname(ref_, self.target_namespace);

        InlineComplexType::new(
            min_occurs, max_occurs,
            RichAstNode::new(
                Type::Group(ref_),
                self.process_annotation(&annotation.iter().collect()),
                ))
    }

    fn process_named_group(&mut self, 
            group: &'ast xs::Group<'input>,
            ) {
        let xs::Group { ref attrs, ref attr_id, ref attr_name, ref annotation, choice_all_choice_sequence: ref content } = group;
        let name = attr_name;

        let mut type_ = match content {
            enums::ChoiceAllChoiceSequence::All(_) => unimplemented!("all"),
            enums::ChoiceAllChoiceSequence::Choice(e) => self.process_choice(e),
            enums::ChoiceAllChoiceSequence::Sequence(e) => self.process_sequence(e),
        };

        type_.type_.doc.extend(&self.process_annotation(&annotation.iter().collect()));

        let name = FullName::new(self.target_namespace, name.0);
        self.groups.insert(name, type_);
    }

    fn process_attribute_group(&mut self, group: &'ast xs::AttributeGroup<'input>) {
        let name = &group.attr_name;
        let attrs = self.process_attr_decls(&group.attr_decls);
        let name = FullName::new(self.target_namespace, name.0);
        self.attribute_groups.insert(name, attrs);
    }

    fn process_local_simple_type(&mut self,
            simple_type: &'ast inline_elements::LocalSimpleType<'input>,
            ) -> InlineSimpleType<'input, ()> {
        let inline_elements::LocalSimpleType { ref attrs, ref attr_id, ref annotation, ref simple_derivation } = simple_type;
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let annotation: Vec<_> = annotation.iter().collect();
        match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => self.process_simple_restriction(e, annotation.clone()),
            xs::SimpleDerivation::List(ref e) => self.process_list(e, annotation.clone()),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e, annotation.clone()),
        }
    }

    fn process_simple_type(&mut self,
            simple_type: &'ast xs::SimpleType<'input>,
            ) -> InlineSimpleType<'input, ()> {
        let xs::SimpleType { ref attrs, ref attr_id, ref attr_name, ref attr_final, ref annotation, ref simple_derivation } = simple_type;
        let annotation: Vec<_> = annotation.iter().collect();
        let name = attr_name;
        let name = FullName::new(self.target_namespace, name.0);
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let ty = match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => 
                self.process_simple_restriction(e, annotation.clone()),
            xs::SimpleDerivation::List(ref e) => self.process_list(e, annotation.clone()),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e, annotation.clone()),
        };

        let doc = self.process_annotation(&annotation);
        self.simple_types.insert(name, (ty, doc.clone()));
        InlineSimpleType::new(
            RichAstNode::new(
                SimpleType::Alias(name),
                doc,
                ))
    }

    fn process_list(&mut self,
            list: &'ast xs::List<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineSimpleType<'input, ()> {
        let item_type = list.attr_item_type;
        let item_type = item_type.as_ref().map(|n| FullName::from_qname(n, self.target_namespace));
        
        let item_type = match (item_type, &list.local_simple_type) {
            (None, Some(st)) => {
                let mut ty = self.process_local_simple_type(st);
                ty.type_.doc.extend(&self.process_annotation(&annotation));
                ty
            },
            (Some(n), None) => {
                InlineSimpleType::new(
                    RichAstNode::new(
                        SimpleType::Alias(n),
                        self.process_annotation(&annotation),
                        ))
            },
            (None, None) => panic!("<list> with no itemType or child type."),
            (Some(ref t1), Some(ref t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        let doc = self.process_annotation(&annotation);
        InlineSimpleType::new(
            RichAstNode::new(
                SimpleType::List(Box::new(item_type)),
                doc,
                ))
    }

    fn process_union(&mut self,
            union: &'ast xs::Union<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineSimpleType<'input, ()> {
        let default_vec = Vec::new();
        let member_types = union.attr_member_types.as_ref().map(|l| &l.0).unwrap_or(&default_vec);
        let mut member_types: Vec<_> = member_types.iter().map(|name| {
            let name = FullName::from_qname(name, self.target_namespace);
            InlineSimpleType::new(
                RichAstNode::new(
                    SimpleType::Alias(name),
                    self.process_annotation(&annotation),
                    ))
        }).collect();

        for t in union.local_simple_type.iter() {
            let ty = self.process_local_simple_type(t);
            member_types.push(ty)
        }

        let doc = self.process_annotation(&annotation);
        InlineSimpleType::new(
            RichAstNode::new(
                SimpleType::Union(member_types),
                doc,
                ))
    }

    fn process_complex_type(&mut self,
            complex_type: &'ast xs::ComplexType<'input>,
            ) -> InlineComplexType<'input, ()> {
        let xs::ComplexType { ref attrs, ref attr_id, ref attr_name, ref attr_mixed, ref attr_abstract, ref attr_final, ref attr_block, ref attr_default_attributes_apply, ref annotation, ref complex_type_model } = complex_type;
        let name = attr_name;
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let mut ty = match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(_) => unimplemented!("simpleContent"),
            xs::ComplexTypeModel::ComplexContent(ref model) =>
                self.process_complex_content(model),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } =>
                self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions),
        };
        ty.type_.doc.extend(&self.process_annotation(&annotation.iter().collect()));
        ty
    }

    fn process_local_complex_type(&mut self,
            complex_type: &'ast inline_elements::LocalComplexType<'input>,
            attr_name: Option<&'ast NcName<'input>>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineComplexType<'input, ()> {
        let inline_elements::LocalComplexType { ref attrs, ref attr_id, ref attr_mixed, ref attr_default_attributes_apply, annotation: ref annotation2, ref complex_type_model } = complex_type;
        let name = attr_name;
        let mut ty = match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(_) => unimplemented!("simpleContent"),
            xs::ComplexTypeModel::ComplexContent(ref model) =>
                self.process_complex_content(model),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } =>
                self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions),
        };
        ty.type_.doc.extend(&self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())));
        ty
    }

    fn process_complete_content_model(&mut self,
            open_content: &'ast Option<Box<xs::OpenContent<'input>>>,
            type_def_particle: &'ast Option<Box<xs::TypeDefParticle<'input>>>,
            attr_decls: &'ast xs::AttrDecls<'input>,
            assertions: &'ast xs::Assertions<'input>,
            ) -> InlineComplexType<'input, ()> {
        let ty = match type_def_particle.as_ref() {
            Some(type_def_particle) => self.process_type_def_particle(type_def_particle),
            None => InlineComplexType::new(1, 1, RichAstNode::new(
                Type::Empty,
                Documentation::new()
            )),
        };
        ty.type_.add_attrs(self.process_attr_decls(attr_decls));
        ty
    }

    fn process_complex_content(&mut self, model: &'ast xs::ComplexContent<'input>) -> InlineComplexType<'input, ()> {
        let xs::ComplexContent { ref attrs, ref attr_id, ref attr_mixed, ref annotation, ref choice_restriction_extension } = model;
        let annotation = annotation.iter().collect();
        match choice_restriction_extension {
            enums::ChoiceRestrictionExtension::Restriction(ref r) => {
                let inline_elements::ComplexRestrictionType {
                    ref attrs, ref attr_id, ref attr_base, annotation: ref annotation2,
                    ref sequence_open_content_type_def_particle,
                    ref attr_decls, ref assertions
                } = **r;
                let ty = match sequence_open_content_type_def_particle {
                    Some(sequences::SequenceOpenContentTypeDefParticle { open_content, type_def_particle }) =>
                        self.process_complex_restriction(attr_base, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref())),
                    None => {
                        InlineComplexType::new(
                            1, 1,
                            RichAstNode::new(
                                Type::Empty,
                                self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                                ))
                    },
                };
                ty.type_.add_attrs(self.process_attr_decls(attr_decls));
                ty
            },
            enums::ChoiceRestrictionExtension::Extension(ref e) => {
                let inline_elements::ExtensionType {
                    ref attrs, ref attr_base, ref attr_id, annotation: ref annotation2, ref open_content,
                    ref type_def_particle, ref attr_decls, ref assertions
                } = **e;
                let ty = match type_def_particle {
                    Some(type_def_particle) =>
                        self.process_extension(attrs, attr_base, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref())),
                    None => self.process_trivial_extension(attrs, attr_base, vec_concat_opt(&annotation, annotation2.as_ref())),
                };
                ty.type_.add_attrs(self.process_attr_decls(attr_decls));
                ty
            },
        }
    }

    fn process_complex_restriction(&mut self, 
            attr_base: &'ast QName<'input>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineComplexType<'input, ()> {
        // TODO: use the base
        let ty = self.process_type_def_particle(type_def_particle);
        let base = FullName::from_qname(attr_base, self.target_namespace);
        InlineComplexType::new(
            1, 1,
            RichAstNode::new(
                Type::Restriction(base, Box::new(ty)),
                self.process_annotation(&annotation),
                ))
    }
    
    fn process_facets(&mut self, facet_list: &Vec<enums::ChoiceFacetAny<'input>>) -> Facets<'input> {
        let mut facets = Facets::default();
        use parser::xs::Facet::*;
        for facet_or_any in facet_list {
            match facet_or_any {
                enums::ChoiceFacetAny::Facet(e) => {
                    match **e {
                        FacetHead(_) => panic!("abstract element"),
                        MinExclusive(ref e) => facets.min_exclusive = Some(e.attr_value.0.parse().expect("invalid minexclusive")),
                        MinInclusive(ref e) => facets.min_inclusive = Some(e.attr_value.0.parse().expect("invalid mininclusive")),
                        MaxExclusive(ref e) => facets.max_exclusive = Some(e.attr_value.0.parse().expect("invalid maxexclusive")),
                        MaxInclusive(ref e) => facets.max_inclusive = Some(e.attr_value.0.parse().expect("invalid maxinclusive")),
                        TotalDigits(ref e) => facets.total_digits = Some(e.attr_value.0),
                        FractionDigits(ref e) => facets.fraction_digits = Some(e.attr_value.0),
                        Length(ref e) => facets.length = Some(e.attr_value.0 as usize),
                        MinLength(ref e) => facets.min_length = Some(e.attr_value.0 as usize),
                        MaxLength(ref e) => facets.max_length = Some(e.attr_value.0 as usize),
                        Enumeration(ref e) => facets.enumeration.get_or_insert(Vec::new()).push(e.attr_value.0),
                        WhiteSpace(ref e) => facets.white_space = Some(((e.attr_value.0).0).0),
                        Pattern(ref e) => facets.pattern = Some(e.attr_value.0),
                        Assertion(ref e) => unimplemented!("assertion facet"),
                        ExplicitTimezone(ref e) => facets.explicit_timezone = Some(((e.attr_value.0).0).0),
                    };
                },
                enums::ChoiceFacetAny::Any(_) => (), // TODO (probably just whitespaces)
            }
        }
        facets
    }

    fn process_simple_restriction(&mut self, 
            restriction: &'ast xs::Restriction<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineSimpleType<'input, ()> {
        let xs::Restriction { ref attrs, ref attr_id, ref attr_base, annotation: ref annotation2, ref simple_restriction_model } = restriction;
        let base = attr_base;
        let base = base.unwrap_or(QName { namespace: Some(SCHEMA_URI), local_name: "anySimpleType" });
        let xs::SimpleRestrictionModel { ref local_simple_type, ref choice_facet_any } = simple_restriction_model;
        let facets = self.process_facets(choice_facet_any);

        let base = FullName::from_qname(&base, self.target_namespace);

        match local_simple_type {
            Some(inline_elements::LocalSimpleType { ref attrs, ref attr_id, annotation: ref annotation2, ref simple_derivation }) => {
                InlineSimpleType::new(
                    RichAstNode::new(
                        SimpleType::Restriction(base, facets),
                        self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                        ))
            },
            None => {
                InlineSimpleType::new(
                    RichAstNode::new(
                        SimpleType::Restriction(base, facets),
                        self.process_annotation(&annotation),
                        ))
            },
        }
    }

    fn process_type_def_particle(&mut self, particle: &'ast xs::TypeDefParticle<'input>) -> InlineComplexType<'input, ()> {
        match particle {
            xs::TypeDefParticle::Group(e) => self.process_group_ref(e),
            xs::TypeDefParticle::All(_) => unimplemented!("all"),
            xs::TypeDefParticle::Choice(e) => self.process_choice(e),
            xs::TypeDefParticle::Sequence(e) => self.process_sequence(e),
        }
    }

    fn process_nested_particle(&mut self,
            particle: &'ast xs::NestedParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineComplexType<'input, ()> {
        let mut ty = match particle {
            xs::NestedParticle::Element(e) => self.process_local_element(e),
            xs::NestedParticle::Group(e) => self.process_group_ref(e),
            xs::NestedParticle::Choice(e) => self.process_choice(e),
            xs::NestedParticle::Sequence(e) => self.process_sequence(e),
            xs::NestedParticle::Any(e) => self.process_any(e, Vec::new()),
        };

        ty.type_.doc.extend(&self.process_annotation(&annotation));
        ty
    }

    fn process_any(&mut self,
            any: &'ast xs::Any<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineComplexType<'input, ()> {
        InlineComplexType::new(
            1, 1,
            RichAstNode::new(
                Type::Any,
                self.process_annotation(&annotation),
                ))
    }

    fn process_sequence(&mut self,
            seq: &'ast xs::Sequence<'input>,
            ) -> InlineComplexType<'input, ()> {
        let xs::Sequence { ref attrs, ref attr_id, ref attr_min_occurs, ref attr_max_occurs, ref annotation, ref nested_particle } = seq;
        let particles = nested_particle;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);
        if min_occurs == 1 && max_occurs == 1 && particles.len() == 1 {
            self.process_nested_particle(particles.get(0).unwrap(), annotation.iter().collect())
        }
        else {
            let items = particles.iter().map(|particle|
                self.process_nested_particle(particle, vec![])
            ).collect();
            let doc = self.process_annotation(&annotation.iter().collect());
            InlineComplexType::new(
                min_occurs, max_occurs,
                RichAstNode::new(
                    Type::Sequence(items),
                    doc,
                    ))
        }
    }

    fn process_choice(&mut self,
            choice: &'ast xs::Choice<'input>,
            ) -> InlineComplexType<'input, ()> {
        let xs::Choice { ref attrs, ref attr_id, ref attr_min_occurs, ref attr_max_occurs, ref annotation, ref nested_particle } = choice;
        let particles = nested_particle;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);
        let mut items = particles.iter().map(|particle|
            vec![self.process_nested_particle(particle, vec![])]
        ).collect();
        let doc = self.process_annotation(&annotation.iter().collect());
        InlineComplexType::new(
            min_occurs, max_occurs,
            RichAstNode::new(
                Type::Choice(items),
                doc,
                ))
    }

    fn process_trivial_extension(&mut self,
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineComplexType<'input, ()> {
        let base = FullName::from_qname(&attr_base, self.target_namespace);
        InlineComplexType::new(
            1, 1,
            RichAstNode::new(
                Type::Alias(base),
                self.process_annotation(&annotation),
                ))
    }

    fn process_extension(&mut self,
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> InlineComplexType<'input, ()> {
        let base = FullName::from_qname(attr_base, self.target_namespace);
        InlineComplexType::new(
            1, 1,
            RichAstNode::new(
                Type::Extension(base, Box::new(self.process_type_def_particle(type_def_particle))),
                self.process_annotation(&annotation),
                ))
    }

    fn process_toplevel_element(&mut self, element: &'ast xs::Element<'input>) {
        let name = FullName::new(self.target_namespace, element.attr_name.0);
        let type_attr: Option<QName<'input>> = element.attr_type;
        let substitution_group = &element.attr_substitution_group;
        let xs::Element { ref attrs, ref attr_id, ref attr_name, ref attr_type, ref attr_substitution_group, ref attr_default, ref attr_fixed, ref attr_nillable, ref attr_abstract, ref attr_final, ref attr_block, ref annotation, type_: ref child_type, ref alternative_alt_type, ref identity_constraint } = element;
        let annotation = annotation.iter().collect();
        if let Some(heads) = attr_substitution_group {
            for head in &heads.0 {
                let head = FullName::from_qname(head, self.target_namespace);
                self.substitution_groups.entry(head)
                    .or_insert(Vec::new())
                    .push(name.clone());
            }
        }
        let type_ = match (type_attr, &child_type) {
            (None, Some(ref c)) => match c {
                enums::Type::SimpleType(ref e) => {
                    let mut ty = self.process_local_simple_type(e);
                    ty.type_.doc.extend(&self.process_annotation(&annotation));
                    ty.into_complex()
                },
                enums::Type::ComplexType(ref e) => {
                    self.process_local_complex_type(e, Some(attr_name), annotation)
                },
            },
            (Some(t), None) => {
                let t = FullName::from_qname(&t, self.target_namespace);
                InlineComplexType::new(
                    1, 1,
                    RichAstNode::new(
                        Type::Alias(t.into()),
                        self.process_annotation(&annotation),
                        ))
            },
            (None, None) => {
                InlineComplexType::new(
                    1, 1,
                    RichAstNode::new(
                        Type::Empty,
                        self.process_annotation(&annotation),
                        ))
            },
            (Some(ref t1), Some(ref t2)) => panic!("Toplevel element '{:?}' has both a type attribute ({:?}) and a child type ({:?}).", name, t1, t2),
        };

        self.elements.insert(name, type_);
    }

    fn process_local_element(&mut self,
            element: &'ast inline_elements::LocalElement<'input>,
            ) -> InlineComplexType<'input, ()> {
        let inline_elements::LocalElement { ref attrs, ref attr_id, ref attr_name, ref attr_ref, ref attr_min_occurs, ref attr_max_occurs, ref attr_type, ref attr_default, ref attr_fixed, ref attr_nillable, ref attr_block, ref attr_form, ref attr_target_namespace, ref annotation, ref type_, ref alternative_alt_type, ref identity_constraint } = element;
        let annotation = annotation.iter().collect();
        let name = attr_name;
        let type_attr = attr_type;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);

        if let Some(ref_) = attr_ref {
            if let Some(name) = name {
                panic!("<element> has both ref={:?} and name={:?}", ref_, name);
            }
            let ref_ = FullName::from_qname(ref_, self.target_namespace);
            InlineComplexType::new(
                min_occurs, max_occurs,
                RichAstNode::new(
                    Type::ElementRef(ref_),
                    self.process_annotation(&annotation),
                    ))
        }
        else {
            let name = name.as_ref().expect("<element> has no name.").0;

            // https://www.w3.org/TR/xmlschema11-1/#dcl.elt.local
            let qualified_form = match attr_form.as_ref().map(|x| ((x.0).0).0) {
                Some("qualified") => true,
                Some("unqualified") => false,
                None => self.element_form_default_qualified,
                _ => unreachable!(),
            };
            let namespace = match (attr_target_namespace, qualified_form) {
                (Some(AnyUri(target_namespace)), _) => Some(*target_namespace),
                (None, true) => self.target_namespace,
                (None, false) => None,
            };

            let mut doc = self.process_annotation(&annotation);

            match (type_attr, &type_) {
                (None, Some(ref c)) => {
                    let mut t = match c {
                        enums::Type::SimpleType(ref e) => {
                            let mut ty = self.process_local_simple_type(e);
                            ty.type_.doc.extend(&self.process_annotation(&annotation));
                            ty.into_complex()
                        },
                        enums::Type::ComplexType(ref e) => {
                            self.process_local_complex_type(e, None, annotation)
                        },
                    };
                    t.type_.doc.extend(&doc);
                    InlineComplexType::new(
                        min_occurs, max_occurs,
                        RichAstNode::new(
                            Type::Element(FullName::new(namespace, name)),
                            t.type_.doc.clone(),
                            ))
                },
                (Some(t), None) => {
                    let name = FullName::from_qname(t, self.target_namespace);
                    InlineComplexType::new(
                        min_occurs, max_occurs,
                        RichAstNode::new(
                            Type::Element(name),
                            doc,
                            ))
                },
                (None, None) => {
                    InlineComplexType::new(
                        min_occurs, max_occurs,
                        RichAstNode::new(
                            Type::Empty,
                            self.process_annotation(&annotation),
                            ))
                },
                (Some(ref t1), Some(ref t2)) => panic!("Element '{:?}' has both a type attribute ({:?}) and a child type ({:?}).", name, t1, t2),
            }
        }
    }

    fn process_attr_decls(&mut self, attr_decls: &'ast xs::AttrDecls<'input>) -> Attrs<'input> {
        let mut attrs = Attrs::new();
        for attr_decl in &attr_decls.attribute {
            match attr_decl {
                enums::AttrOrAttrGroup::Attribute(e) => {
                    let name = e.attr_name.as_ref().map(|ncn| FullName::new(self.target_namespace, ncn.0));
                    let mut type_attr: Option<QName<'input>> = e.attr_type;
                    let use_ = match e.attr_use.as_ref().map(|x| ((x.0).0).0) {
                        Some("prohibited") => AttrUse::Prohibited,
                        Some("required") => AttrUse::Required,
                        Some("optional") => AttrUse::Optional,
                        None => AttrUse::Optional, // TODO
                        Some(s) => panic!("Unknown attribute value use={:?}", s),
                    };
                    match (name, e.attr_ref, type_attr, &e.local_simple_type) {
                        (Some(name), None, Some(t), None) => {
                            let t = FullName::from_qname(&t, self.target_namespace);
                            attrs.named.push((name, use_, Some(SimpleType::Alias(t))));
                        },
                        (Some(name), None, None, Some(t)) => {
                            let t = self.process_local_simple_type(t);
                            attrs.named.push((name, use_, Some(t.type_.type_)));
                        },
                        (Some(name), None, None, None) =>
                            attrs.named.push((name, use_, None)),
                        (None, None, None, None) =>
                            panic!("no attribute on <attribute>."),
                        (_, _, Some(ref t1), Some(ref t2)) =>
                            panic!("<attribute> has both a type attribute ({:?}) and a child type ({:?}).", t1, t2),
                        (None, None, Some(_), None) | (None, None, None, Some(_)) =>
                            panic!("<attribute> has a type but no name."),
                        (_, Some(_), Some(_), None) | (_, Some(_), None, Some(_)) =>
                            panic!("<attribute> has a type and a ref."),
                        (_, Some(ref_), None, None) => (), // TODO
                    }
                },
                enums::AttrOrAttrGroup::AttributeGroup(e) => {
                    attrs.group_refs.push(FullName::from_qname(&e.attr_ref, self.target_namespace));
                },
            }
        }
        if attr_decls.any_attribute.is_some() {
            attrs.any_attributes = true;
        }
        attrs
    }


}
