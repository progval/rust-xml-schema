use std::fmt::Debug;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

use xmlparser::Token as XmlToken;
use xmlparser::{TextUnescape, XmlSpace};

use parser::*;
use names::*;
use support::Facets;
use primitives::{QName,NcName,AnyUri};

pub const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

fn parse_max_occurs(s: &str) -> Result<usize, ParseIntError> {
    if s == "unbounded" {
        Ok(usize::max_value())
    }
    else {
        s.parse()
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
    fn new() -> Attrs<'input> {
        Attrs { named: Vec::new(), refs: Vec::new(), group_refs: Vec::new(), any_attributes: false }
    }
    fn extend(&mut self, other: Attrs<'input>) {
        let Attrs { named, refs, group_refs, any_attributes } = other;
        self.named.extend(named);
        self.refs.extend(refs);
        self.group_refs.extend(group_refs);
        self.any_attributes |= any_attributes;
    }
    pub fn restrict(&self, other: &Attrs<'input>) -> Attrs<'input> {
        let mut other_named = HashMap::new();
        for (name, attr_use, type_) in other.named.iter() {
            other_named.insert(name.clone(), (*attr_use, type_));
        }
        let mut seen = HashSet::new();
        let mut named: Vec<_> = self.named.iter().map(|(name, attr_use, type_)| {
            seen.insert(name);
            match other_named.get(name) {
                None => (name.clone(), *attr_use, (*type_).clone()),
                Some((attr_use, type_)) => (name.clone(), *attr_use, (*type_).clone()),
            }
        }).collect();

        let mut other_refs = HashMap::new();
        for (name, attr_use, ref_) in other.refs.iter() {
            other_refs.insert((name.clone(), ref_.clone()), *attr_use);
        }
        let mut refs: Vec<_> = self.refs.iter().map(|(name, attr_use, ref_)| {
            if let Some(name) = name {
                seen.insert(name);
            }
            match other_refs.get(&(*name, *ref_)) {
                None => (name.clone(), *attr_use, (*ref_).clone()),
                Some(attr_use) => (name.clone(), *attr_use, (*ref_).clone()),
            }
        }).collect();

        if other.any_attributes {
            for (name, attr_use, type_) in self.named.iter() {
                if !seen.contains(name) {
                    named.push((name.clone(), *attr_use, type_.clone()));
                }
            }
            for (name, attr_use, ref_) in other.refs.iter() {
                match name {
                    Some(name) => {
                        if !seen.contains(name) {
                            refs.push((Some(name.clone()), *attr_use, ref_.clone()));
                        }
                    },
                    None => (), // TODO
                }
            }
        }

        Attrs { named, refs, group_refs: self.group_refs.clone(), any_attributes: other.any_attributes }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RichType<'input, T: Debug + Hash + PartialEq + Eq + PartialOrd + Ord> {
    pub name_hint: NameHint<'input>,
    pub attrs: Attrs<'input>,
    pub type_: T,
    pub doc: Documentation<'input>,
}
impl<'input, T: Debug + Hash + PartialEq + Eq + PartialOrd + Ord> RichType<'input, T> {
    pub fn new(name_hint: NameHint<'input>, type_: T, doc: Documentation<'input>) -> RichType<'input, T> {
        RichType { name_hint, attrs: Attrs::new(), type_, doc }
    }
}
impl<'input> RichType<'input, Type<'input>> {
    fn add_attrs(mut self, new_attrs: Attrs<'input>) -> RichType<'input, Type<'input>> {
        self.attrs.extend(new_attrs);
        self
    }
}

impl<'input> RichType<'input, SimpleType<'input>> {
    pub fn into_complex(self) -> RichType<'input, Type<'input>> {
        let RichType { name_hint, attrs, type_, doc } = self;
        RichType { name_hint, attrs, type_: Type::Simple(type_), doc }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type<'input> {
    Any,
    Empty,
    Alias(FullName<'input>),
    Extension(FullName<'input>, Box<RichType<'input, Type<'input>>>),
    Restriction(FullName<'input>, Box<RichType<'input, Type<'input>>>),
    ElementRef(usize, usize, FullName<'input>),
    Element(usize, usize, String),
    Group(usize, usize, FullName<'input>),
    Choice(usize, usize, String),
    InlineChoice(Vec<RichType<'input, Type<'input>>>),
    Sequence(usize, usize, String),
    InlineSequence(Vec<RichType<'input, Type<'input>>>),
    Simple(SimpleType<'input>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleType<'input> {
    Primitive(&'static str, &'static str),
    Alias(FullName<'input>),
    Restriction(FullName<'input>, Facets<'input>),
    List(String),
    Union(String),
    Empty,
}

#[derive(Debug)]
pub struct Processor<'ast, 'input: 'ast> {
    pub target_namespace: Option<&'input str>,
    pub element_form_default_qualified: bool,
    pub attribute_form_default_qualified: bool,
    pub elements: HashMap<FullName<'input>, RichType<'input, Type<'input>>>,
    pub types: HashMap<FullName<'input>, RichType<'input, Type<'input>>>,
    pub simple_types: HashMap<FullName<'input>, (RichType<'input, SimpleType<'input>>, Documentation<'input>)>,
    pub choices: HashMap<Vec<RichType<'input, Type<'input>>>, HashSet<String>>,
    pub sequences: HashMap<Vec<RichType<'input, Type<'input>>>, (HashSet<String>, Documentation<'input>)>,
    pub groups: HashMap<FullName<'input>, RichType<'input, Type<'input>>>,
    pub attribute_groups: HashMap<FullName<'input>, Attrs<'input>>,
    pub inline_elements: HashMap<(Option<&'input str>, &'input str, Attrs<'input>, Type<'input>), (HashSet<String>, Documentation<'input>)>,

    pub lists: HashMap<RichType<'input, SimpleType<'input>>, HashSet<String>>,
    pub unions: HashMap<Vec<RichType<'input, SimpleType<'input>>>, HashSet<String>>,
    pub simple_restrictions: HashSet<(FullName<'input>, Facets<'input>)>,
    pub substitution_groups: HashMap<FullName<'input>, Vec<FullName<'input>>>,
    _phantom: PhantomData<&'ast ()>, // Sometimes I need 'ast when prototyping
}

impl<'ast, 'input: 'ast> Processor<'ast, 'input> {
    pub fn new(ast: &'ast xs::Schema<'input>) -> Processor<'ast, 'input> {
        let mut target_namespace = None;
        for (key, &value) in ast.attrs.iter() {
            match key.as_tuple() {
                ("xml", "lang") => (),
                ("xmlns", ns) => (),
                (SCHEMA_URI, "targetNamespace") => target_namespace = Some(value),
                (SCHEMA_URI, "elementFormDefault") => (),
                (SCHEMA_URI, "attributeFormDefault") => (),
                (SCHEMA_URI, "version") => (),
                _ => panic!("Unknown attribute {} on <schema>.", key),
            }
        }
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
            types: HashMap::new(),
            groups: HashMap::new(),
            choices: HashMap::new(),
            lists: HashMap::new(),
            unions: HashMap::new(),
            sequences: HashMap::new(),
            attribute_groups: HashMap::new(),
            inline_elements: HashMap::new(),
            simple_types: HashMap::new(),
            simple_restrictions: HashSet::new(),
            substitution_groups: HashMap::new(),
            _phantom: PhantomData::default(),
        }
    }

    pub fn process_ast(&mut self, ast: &'ast xs::Schema<'input>) {
        for top_level_item in ast.sequence_schema_top_annotation.iter() {
            match top_level_item.schema_top {
                xs::SchemaTop::Redefinable(ref r) => self.process_redefinable(r, false),
                xs::SchemaTop::Element(ref e) => { self.process_toplevel_element(e); },
                xs::SchemaTop::Attribute(_) => unimplemented!("top-level attribute"),
                xs::SchemaTop::Notation(ref e) => self.process_notation(e),
            }
        }
    }

    fn process_notation(&mut self, notation: &'ast xs::Notation<'input>) {
        // TODO
    }

    fn process_redefinable(&mut self, r: &'ast xs::Redefinable<'input>, inlinable: bool) {
        match r {
            xs::Redefinable::SimpleType(ref e) => { self.process_simple_type(e); },
            xs::Redefinable::ComplexType(e) => { self.process_complex_type(e, inlinable); },
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
            ) -> RichType<'input, Type<'input>> {
        let inline_elements::GroupRef { ref attrs, ref attr_ref, ref annotation } = group_ref;
        let ref_ = attr_ref;
        let mut max_occurs = 1;
        let mut min_occurs = 1;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "ref") => (),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        let ref_ = FullName::new(ref_.0, ref_.1);
        let (_, field_name) = ref_.as_tuple();
        RichType::new(
            NameHint::new(field_name),
            Type::Group(min_occurs, max_occurs, ref_),
            self.process_annotation(&annotation.iter().collect()),
            )
    }

    fn process_named_group(&mut self, 
            group: &'ast xs::Group<'input>,
            ) -> RichType<'input, Type<'input>> {
        let xs::Group { ref attrs, ref attr_name, ref annotation, choice_all_choice_sequence: ref content } = group;
        let name = attr_name;
        let mut max_occurs = 1;
        let mut min_occurs = 1;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        let mut type_ = match content {
            enums::ChoiceAllChoiceSequence::All(_) => unimplemented!("all"),
            enums::ChoiceAllChoiceSequence::Choice(e) => self.process_choice(e, true),
            enums::ChoiceAllChoiceSequence::Sequence(e) => self.process_sequence(e, true),
        };

        type_.doc.extend(&self.process_annotation(&annotation.iter().collect()));
        let doc = type_.doc.clone();

        let name = FullName::new(self.target_namespace, name.0);
        self.groups.insert(name, type_);
        RichType::new(
            NameHint::from_fullname(&name),
            Type::Group(min_occurs, max_occurs, name),
            doc,
            )
    }

    fn process_attribute_group(&mut self, group: &'ast xs::AttributeGroup<'input>) {
        let mut name = &group.attr_name;
        for (key, &value) in group.attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                _ => panic!("Unknown attribute {} in <attributeGroup>", key),
            }
        }
        let attrs = self.process_attr_decls(&group.attr_decls);
        let name = FullName::new(self.target_namespace, name.0);
        self.attribute_groups.insert(name, attrs);
    }

    fn process_local_simple_type(&mut self,
            simple_type: &'ast inline_elements::LocalSimpleType<'input>,
            ) -> RichType<'input, SimpleType<'input>> {
        let inline_elements::LocalSimpleType { ref attrs, ref annotation, ref simple_derivation } = simple_type;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                //(SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "id") => (), // TODO
                _ => panic!("Unknown attribute {} in <simpleType>", key),
            }
        }
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
            ) -> RichType<'input, SimpleType<'input>> {
        let xs::SimpleType { ref attrs, ref attr_name, ref attr_final, ref annotation, ref simple_derivation } = simple_type;
        let annotation: Vec<_> = annotation.iter().collect();
        let name = attr_name;
        let name = FullName::new(self.target_namespace, name.0);
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "id") => (), // TODO
                _ => panic!("Unknown attribute {} in <simpleType>", key),
            }
        }
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let ty = match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => 
                self.process_simple_restriction(e, annotation.clone()),
            xs::SimpleDerivation::List(ref e) => self.process_list(e, annotation.clone()),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e, annotation.clone()),
        };

        let doc = self.process_annotation(&annotation);
        self.simple_types.insert(name, (ty, doc.clone()));
        RichType::new(
            NameHint::from_fullname(&name),
            SimpleType::Alias(name),
            doc,
            )
    }

    fn process_list(&mut self,
            list: &'ast xs::List<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input, SimpleType<'input>> {
        let item_type = list.attr_item_type;
        let item_type = item_type.map(|n| FullName::new(n.0, n.1));
        for (key, &value) in list.attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "itemType") => (),
                _ => panic!("Unknown attribute {} in <list>", key),
            }
        }
        
        let item_type = match (item_type, &list.local_simple_type) {
            (None, Some(st)) => {
                let mut ty = self.process_local_simple_type(st);
                ty.doc.extend(&self.process_annotation(&annotation));
                ty
            },
            (Some(n), None) => {
                RichType::new(
                    NameHint::new(n.as_tuple().1),
                    SimpleType::Alias(n),
                    self.process_annotation(&annotation),
                    )
            },
            (None, None) => panic!("<list> with no itemType or child type."),
            (Some(ref t1), Some(ref t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        let mut name_hint = item_type.name_hint.clone();
        name_hint.push("list");
        let doc = self.process_annotation(&annotation);
        let name = name_from_hint(&name_hint).unwrap();
        self.lists.entry(item_type)
                .or_insert(HashSet::new())
                .insert(name.clone());
        RichType::new(
            name_hint,
            SimpleType::List(name),
            doc,
            )
    }

    fn process_union(&mut self,
            union: &'ast xs::Union<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input, SimpleType<'input>> {
        let default_vec = Vec::new();
        let mut name_hint = NameHint::new("union");
        let member_types = union.attr_member_types.as_ref().map(|l| &l.0).unwrap_or(&default_vec);
        let mut member_types: Vec<_> = member_types.iter().map(|name| {
            let name = FullName::new(name.0.or(self.target_namespace), name.1);
            name_hint.push(name.local_name());
            RichType::new(
                NameHint::new(name.local_name()),
                SimpleType::Alias(name),
                self.process_annotation(&annotation),
                )
        }).collect();
        for (key, &value) in union.attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "memberTypes") => (),
                _ => panic!("Unknown attribute {} in <union>", key),
            }
        }

        for t in union.local_simple_type.iter() {
            let ty = self.process_local_simple_type(t);
            name_hint.extend(&ty.name_hint);
            member_types.push(ty)
        }

        let doc = self.process_annotation(&annotation);
        let name = name_from_hint(&name_hint).unwrap();
        self.unions.entry(member_types)
                .or_insert(HashSet::new())
                .insert(name.clone());
        RichType::new(
            name_hint,
            SimpleType::Union(name),
            doc,
            )
    }

    fn process_complex_type(&mut self,
            complex_type: &'ast xs::ComplexType<'input>,
            inlinable: bool,
            ) -> RichType<'input, Type<'input>> {
        let xs::ComplexType { ref attrs, ref attr_name, ref attr_mixed, ref attr_abstract, ref attr_final, ref attr_block, ref attr_default_attributes_apply, ref annotation, ref complex_type_model } = complex_type;
        let name = attr_name;
        let mut abstract_ = false;
        let mut mixed = false;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "abstract") => {
                    match value {
                        "true" => abstract_ = true,
                        "false" => abstract_ = false,
                        _ => panic!("Invalid value for abstract attribute: {}", value),
                    }
                },
                (SCHEMA_URI, "mixed") => {
                    match value {
                        "true" => mixed = true,
                        "false" => mixed = false,
                        _ => panic!("Invalid value for mixed attribute: {}", value),
                    }
                },
                _ => panic!("Unknown attribute {} in <complexType>", key),
            }
        }
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let mut ty = match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(_) => unimplemented!("simpleContent"),
            xs::ComplexTypeModel::ComplexContent(ref model) =>
                self.process_complex_content(model, false),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } =>
                self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions, inlinable),
        };
        ty.doc.extend(&self.process_annotation(&annotation.iter().collect()));

        let doc = ty.doc.clone();
        let name = FullName::new(self.target_namespace, name.0);
        self.types.insert(name, ty);
        RichType::new(
            NameHint::from_fullname(&name),
            Type::Alias(name),
            doc,
            )
    }

    fn process_local_complex_type(&mut self,
            complex_type: &'ast inline_elements::LocalComplexType<'input>,
            attr_name: Option<&'ast NcName<'input>>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> RichType<'input, Type<'input>> {
        let inline_elements::LocalComplexType { ref attrs, ref attr_mixed, ref attr_default_attributes_apply, annotation: ref annotation2, ref complex_type_model } = complex_type;
        let name = attr_name;
        let mut abstract_ = false;
        let mut mixed = false;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "abstract") => {
                    match value {
                        "true" => abstract_ = true,
                        "false" => abstract_ = false,
                        _ => panic!("Invalid value for abstract attribute: {}", value),
                    }
                },
                (SCHEMA_URI, "mixed") => {
                    match value {
                        "true" => mixed = true,
                        "false" => mixed = false,
                        _ => panic!("Invalid value for mixed attribute: {}", value),
                    }
                },
                _ => panic!("Unknown attribute {} in <complexType>", key),
            }
        }
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let mut ty = match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(_) => unimplemented!("simpleContent"),
            xs::ComplexTypeModel::ComplexContent(ref model) =>
                self.process_complex_content(model, false),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } =>
                self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions, inlinable),
        };
        ty.doc.extend(&self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())));

        if let Some(name) = name {
            let doc = ty.doc.clone();
            let name = FullName::new(self.target_namespace, name.0);
            self.types.insert(name, ty);
            RichType::new(
                NameHint::from_fullname(&name),
                Type::Alias(name),
                doc,
                )
        }
        else {
            ty
        }
    }

    fn process_complete_content_model(&mut self,
            open_content: &'ast Option<Box<xs::OpenContent<'input>>>,
            type_def_particle: &'ast Option<Box<xs::TypeDefParticle<'input>>>,
            attr_decls: &'ast xs::AttrDecls<'input>,
            assertions: &'ast xs::Assertions<'input>,
            inlinable: bool,
            ) -> RichType<'input, Type<'input>> {
        self.process_type_def_particle(type_def_particle.as_ref().unwrap(), inlinable)
            .add_attrs(self.process_attr_decls(attr_decls))
    }

    fn process_complex_content(&mut self, model: &'ast xs::ComplexContent<'input>, inlinable: bool) -> RichType<'input, Type<'input>> {
        let xs::ComplexContent { ref attrs, ref attr_id, ref attr_mixed, ref annotation, ref choice_restriction_extension } = model;
        let annotation = annotation.iter().collect();
        match choice_restriction_extension {
            enums::ChoiceRestrictionExtension::Restriction(ref r) => {
                let inline_elements::ComplexRestrictionType {
                    ref attrs, ref attr_base, annotation: ref annotation2,
                    ref sequence_open_content_type_def_particle,
                    ref attr_decls, ref assertions
                } = **r;
                let ty = match sequence_open_content_type_def_particle {
                    Some(sequences::SequenceOpenContentTypeDefParticle { open_content, type_def_particle }) =>
                        self.process_complex_restriction(attrs, attr_base, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref())),
                    None => {
                        RichType::new(
                            NameHint::new("empty_extension"),
                            Type::Empty,
                            self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                            )
                    },
                };
                ty.add_attrs(self.process_attr_decls(attr_decls))
            },
            enums::ChoiceRestrictionExtension::Extension(ref e) => {
                let inline_elements::ExtensionType {
                    ref attrs, ref attr_base, ref attr_id, annotation: ref annotation2, ref open_content,
                    ref type_def_particle, ref attr_decls, ref assertions
                } = **e;
                let ty = match type_def_particle {
                    Some(type_def_particle) =>
                        self.process_extension(attrs, attr_base, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref()), inlinable),
                    None => self.process_trivial_extension(attrs, attr_base, vec_concat_opt(&annotation, annotation2.as_ref())),
                };
                ty.add_attrs(self.process_attr_decls(attr_decls))
            },
        }
    }

    fn process_complex_restriction(&mut self, 
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input, Type<'input>> {
        let base = attr_base;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "base") => (),
                _ => panic!("Unknown attribute {} in <restriction>", key),
            }
        }
        let base = FullName::new(base.0, base.1);
        // TODO: use the base
        let ty = self.process_type_def_particle(type_def_particle, false);
        RichType::new(
            NameHint::new_empty(),
            Type::Restriction(base, Box::new(ty)),
            self.process_annotation(&annotation),
            )
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
                        _ => unimplemented!("{:?}", e),// TODO
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
            ) -> RichType<'input, SimpleType<'input>> {
        let xs::Restriction { ref attrs, ref attr_id, ref attr_base, annotation: ref annotation2, ref simple_restriction_model } = restriction;
        let base = attr_base;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "base") => (),
                _ => panic!("Unknown attribute {} in <restriction>", key),
            }
        }
        let base = base.unwrap_or(QName(Some(SCHEMA_URI), "anySimpleType"));
        let base = FullName::new(base.0, base.1);
        let xs::SimpleRestrictionModel { ref local_simple_type, ref choice_facet_any } = simple_restriction_model;
        let facets = self.process_facets(choice_facet_any);

        self.simple_restrictions.insert((base.clone(), facets.clone()));

        match local_simple_type {
            Some(inline_elements::LocalSimpleType { ref attrs, annotation: ref annotation2, ref simple_derivation }) => {
                RichType::new(
                    NameHint::new(base.as_tuple().1),
                    SimpleType::Restriction(base, facets),
                    self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                    )
            },
            None => {
                RichType::new(
                    NameHint::new(base.as_tuple().1),
                    SimpleType::Restriction(base, facets),
                    self.process_annotation(&annotation),
                    )
            },
        }
    }

    fn process_type_def_particle(&mut self, particle: &'ast xs::TypeDefParticle<'input>, inlinable: bool) -> RichType<'input, Type<'input>> {
        match particle {
            xs::TypeDefParticle::Group(e) => self.process_group_ref(e),
            xs::TypeDefParticle::All(_) => unimplemented!("all"),
            xs::TypeDefParticle::Choice(e) => self.process_choice(e, inlinable),
            xs::TypeDefParticle::Sequence(e) => self.process_sequence(e, inlinable),
        }
    }

    fn process_nested_particle(&mut self,
            particle: &'ast xs::NestedParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool
            ) -> RichType<'input, Type<'input>> {
        let mut ty = match particle {
            xs::NestedParticle::Element(e) => self.process_local_element(e),
            xs::NestedParticle::Group(e) => self.process_group_ref(e),
            xs::NestedParticle::Choice(e) => self.process_choice(e, inlinable),
            xs::NestedParticle::Sequence(e) => self.process_sequence(e, inlinable),
            xs::NestedParticle::Any(e) => self.process_any(e, Vec::new()),
        };

        ty.doc.extend(&self.process_annotation(&annotation));
        ty
    }

    fn process_any(&mut self,
            any: &'ast xs::Any<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input, Type<'input>> {
        RichType::new(
            NameHint::new("any"),
            Type::Any,
            self.process_annotation(&annotation),
            )
    }

    fn process_sequence(&mut self,
            seq: &'ast xs::Sequence<'input>,
            inlinable: bool,
            ) -> RichType<'input, Type<'input>> {
        let xs::Sequence { ref attrs, ref attr_min_occurs, ref attr_max_occurs, ref annotation, ref nested_particle } = seq;
        let particles = nested_particle;
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <sequence>", key),
            }
        }
        let mut items = Vec::new();
        let mut name_hint = NameHint::new("sequence");
        if min_occurs == 1 && max_occurs == 1 && inlinable && particles.len() == 1 {
            self.process_nested_particle(particles.get(0).unwrap(), annotation.iter().collect(), inlinable)
        }
        else {
            for particle in particles.iter() {
                let ty = self.process_nested_particle(particle, vec![], false);
                name_hint.extend(&ty.name_hint);
                items.push(ty);
            }
            let doc = self.process_annotation(&annotation.iter().collect());
            if min_occurs == 1 && max_occurs == 1 {
                RichType::new(
                    name_hint,
                    Type::InlineSequence(items),
                    doc,
                    )
            }
            else {
                let name = name_from_hint(&name_hint).unwrap();
                let (names, docs) = self.sequences.entry(items)
                    .or_insert((HashSet::new(), Documentation::new()));
                names.insert(name.clone());
                docs.extend(&doc);
                RichType::new(
                    name_hint,
                    Type::Sequence(min_occurs, max_occurs, name),
                    doc,
                    )
            }
        }
    }

    fn process_choice(&mut self,
            choice: &'ast xs::Choice<'input>,
            inlinable: bool
            ) -> RichType<'input, Type<'input>> {
        let xs::Choice { ref attrs, ref attr_min_occurs, ref attr_max_occurs, ref annotation, ref nested_particle } = choice;
        let particles = nested_particle;
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <choice>", key),
            }
        }
        let mut items = Vec::new();
        let mut name_hint = NameHint::new("choice");
        if particles.len() == 1 {
            let particle = particles.get(0).unwrap();
            let RichType { name_hint, attrs, type_, doc } =
                self.process_nested_particle(particle, annotation.iter().collect(), inlinable);
            match (min_occurs, max_occurs, type_) {
                (_, _, Type::Element(1, 1, e)) => return RichType {
                    name_hint, attrs, type_: Type::Element(min_occurs, max_occurs, e), doc },
                (_, _, Type::Group(1, 1, e)) => return RichType {
                    name_hint, attrs, type_: Type::Group(min_occurs, max_occurs, e), doc },
                (_, _, Type::Choice(1, 1, e)) => return RichType {
                    name_hint, attrs, type_: Type::Choice(min_occurs, max_occurs, e), doc },
                (_, _, Type::Sequence(1, 1, e)) => return RichType {
                    name_hint, attrs, type_: Type::Sequence(min_occurs, max_occurs, e), doc },
                (1, 1, type_) => return RichType { name_hint, attrs, type_, doc },
                (_, _, type_) => {
                    let name = name_from_hint(&name_hint).unwrap();
                    let items = vec![RichType { name_hint: name_hint.clone(), attrs: Attrs::new(), type_, doc: doc.clone() }];
                    let (names, docs) = self.sequences.entry(items)
                        .or_insert((HashSet::new(), Documentation::new()));
                    names.insert(name.clone());
                    docs.extend(&doc);
                    let type_ = Type::Sequence(min_occurs, max_occurs, name);
                    return RichType { name_hint, attrs, type_, doc }
                },
            }
        }
        else {
            for particle in particles.iter() {
                let ty = self.process_nested_particle(particle, vec![], false);
                name_hint.extend(&ty.name_hint);
                items.push(ty);
            }
        }
        let doc = self.process_annotation(&annotation.iter().collect());
        match (min_occurs, max_occurs, inlinable) {
            (1, 1, true) => {
                RichType::new(
                    name_hint,
                    Type::InlineChoice(items),
                    doc,
                    )
            },
            (_, _, _) => {
                let name = name_from_hint(&name_hint).unwrap();
                self.choices.entry(items)
                        .or_insert(HashSet::new())
                        .insert(name.clone());
                RichType::new(
                    name_hint,
                    Type::Choice(min_occurs, max_occurs, name),
                    doc,
                    )
            }
        }
    }

    fn process_trivial_extension(&mut self,
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input, Type<'input>> {
        let base = attr_base;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "base") => (),
                _ => panic!("Unknown attribute {} in <extension>", key),
            }
        }
        let base = FullName::new(base.0, base.1);
        RichType::new(
            NameHint::new_empty(),
            Type::Alias(base),
            self.process_annotation(&annotation),
            )
    }

    fn process_extension(&mut self,
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> RichType<'input, Type<'input>> {
        let base = attr_base;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "base") => (),
                _ => panic!("Unknown attribute {} in <extension>", key),
            }
        }
        let base = FullName::new(base.0, base.1);
        RichType::new(
            NameHint::new_empty(),
            Type::Extension(base, Box::new(self.process_type_def_particle(type_def_particle, inlinable))),
            self.process_annotation(&annotation),
            )
    }

    fn process_toplevel_element(&mut self, element: &'ast xs::Element<'input>) {
        let name = FullName::new(self.target_namespace, element.attr_name.0);
        let type_attr: Option<QName<'input>> = element.attr_type;
        let mut abstract_ = false;
        let mut substitution_group = &element.attr_substitution_group;
        for (key, &value) in element.attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "id") =>
                    (),
                (SCHEMA_URI, "type") => (),
                (SCHEMA_URI, "abstract") => {
                    match value {
                        "true" => abstract_ = true,
                        "false" => abstract_ = false,
                        _ => panic!("Invalid value for abstract attribute: {}", value),
                    }
                },
                (SCHEMA_URI, "substitutionGroup") => (),
                _ => panic!("Unknown attribute {:?} in toplevel <element>", key),
            }
        }
        let xs::Element { ref attrs, ref attr_name, ref attr_type, ref attr_substitution_group, ref attr_default, ref attr_fixed, ref attr_nillable, ref attr_abstract, ref attr_final, ref attr_block, ref annotation, type_: ref child_type, ref alternative_alt_type, ref identity_constraint } = element;
        let annotation = annotation.iter().collect();
        if let Some(heads) = attr_substitution_group {
            for head in &heads.0 {
                let head = FullName::new(head.0, head.1);
                self.substitution_groups.entry(head)
                    .or_insert(Vec::new())
                    .push(name.clone());
            }
        }
        let type_ = match (type_attr, &child_type) {
            (None, Some(ref c)) => match c {
                enums::Type::SimpleType(ref e) => {
                    let mut ty = self.process_local_simple_type(e);
                    ty.doc.extend(&self.process_annotation(&annotation));
                    ty.into_complex()
                },
                enums::Type::ComplexType(ref e) => {
                    self.process_local_complex_type(e, Some(attr_name), annotation, false)
                },
            },
            (Some(t), None) => {
                RichType::new(
                    NameHint::new(t.1),
                    Type::Alias(FullName::new(t.0, t.1)),
                    self.process_annotation(&annotation),
                    )
            },
            (None, None) => {
                RichType::new(
                    NameHint::new("empty"),
                    Type::Empty,
                    self.process_annotation(&annotation),
                    )
            },
            (Some(ref t1), Some(ref t2)) => panic!("Toplevel element '{:?}' has both a type attribute ({:?}) and a child type ({:?}).", name, t1, t2),
        };

        self.elements.insert(name, type_);
    }

    fn process_local_element(&mut self,
            element: &'ast inline_elements::LocalElement<'input>,
            ) -> RichType<'input, Type<'input>> {
        let inline_elements::LocalElement { ref attrs, ref attr_name, ref attr_ref, ref attr_min_occurs, ref attr_max_occurs, ref attr_type, ref attr_default, ref attr_fixed, ref attr_nillable, ref attr_block, ref attr_form, ref attr_target_namespace, ref annotation, ref type_, ref alternative_alt_type, ref identity_constraint } = element;
        let annotation = annotation.iter().collect();
        let name = attr_name;
        let ref_ = attr_ref.as_ref().map(|qn| FullName::new(qn.0, qn.1));
        let type_attr = attr_type;
        let mut abstract_ = false;
        //let mut substitution_group = None;
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in attrs.iter() {
            match key.as_tuple() {
                (SCHEMA_URI, "name") => (),
                (SCHEMA_URI, "id") =>
                    (),
                (SCHEMA_URI, "type") => (),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                (SCHEMA_URI, "abstract") => {
                    match value {
                        "true" => abstract_ = true,
                        "false" => abstract_ = false,
                        _ => panic!("Invalid value for abstract attribute: {}", value),
                    }
                },
                (SCHEMA_URI, "ref") => (),
                _ => panic!("Unknown attribute {} in <element>", key),
            }
        }

        if let Some(ref_) = ref_ {
            if let Some(name) = name {
                panic!("<element> has both ref={:?} and name={:?}", ref_, name);
            }
            let (_, field_name) = ref_.as_tuple();
            RichType::new(
                NameHint::new(field_name),
                Type::ElementRef(min_occurs, max_occurs, ref_),
                self.process_annotation(&annotation),
                )
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

            match (type_attr, &type_) {
                (None, Some(ref c)) => {
                    let mut t = match c {
                        enums::Type::SimpleType(ref e) => {
                            let mut ty = self.process_local_simple_type(e);
                            ty.doc.extend(&self.process_annotation(&annotation));
                            ty.into_complex()
                        },
                        enums::Type::ComplexType(ref e) => {
                            self.process_local_complex_type(e, None, annotation, false)
                        },
                    };
                    let mut name_hint = NameHint::new(name);
                    name_hint.extend(&t.name_hint);
                    let struct_name = name_from_hint(&name_hint).unwrap();
                    let (elems, doc) = self.inline_elements.entry((namespace, name, t.attrs, t.type_))
                            .or_insert((HashSet::new(), Documentation::new()));
                    elems.insert(struct_name.clone());
                    t.doc.extend(doc);
                    RichType::new(
                        NameHint::new(name),
                        Type::Element(min_occurs, max_occurs, struct_name),
                        t.doc,
                        )
                },
                (Some(t), None) => {
                    let name_hint1 = NameHint::new(t.1);
                    let mut name_hint2 = NameHint::new(name);
                    name_hint2.push(t.1);
                    // TODO: move this heuristic in names.rs
                    let name_hint = if t.1.to_lowercase().contains(&name.to_lowercase()) {
                        name_hint1
                    }
                    else {
                        name_hint2
                    };
                    let struct_name = name_from_hint(&name_hint).unwrap();
                    let mut doc = self.process_annotation(&annotation);
                    let (elems, doc2) = self.inline_elements.entry((namespace, name, Attrs::new(), Type::Alias(FullName::new(t.0, t.1))))
                            .or_insert((HashSet::new(), Documentation::new()));
                    elems.insert(struct_name.clone());
                    doc.extend(doc2);
                    RichType::new(
                        NameHint::new(name),
                        Type::Element(min_occurs, max_occurs, struct_name),
                        doc,
                        )
                },
                (None, None) => {
                    RichType::new(
                        NameHint::new("empty"),
                        Type::Empty,
                        self.process_annotation(&annotation),
                        )
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
                    let mut ref_ = e.attr_ref.as_ref().map(|qn| FullName::new(qn.0, qn.1));
                    let mut type_attr: Option<QName<'input>> = e.attr_type;
                    let mut use_ = None;
                    for (key, &value) in e.attrs.iter() {
                        match key.as_tuple() {
                            (SCHEMA_URI, "name") => (),
                            (SCHEMA_URI, "use") =>
                                use_ = Some(value),
                            (SCHEMA_URI, "default") => (), // TODO
                            (SCHEMA_URI, "type") => (),
                            (SCHEMA_URI, "ref") => (),
                            (SCHEMA_URI, "fixed") => (), // TODO
                            _ => panic!("Unknown attribute {} in <attribute>", key),
                        }
                    }
                    let use_ = match use_ {
                        Some("prohibited") => AttrUse::Prohibited,
                        Some("required") => AttrUse::Required,
                        Some("optional") => AttrUse::Optional,
                        None => AttrUse::Optional, // TODO
                        Some(s) => panic!("Unknown attribute value use={:?}", s),
                    };
                    match (name, ref_, type_attr, &e.local_simple_type) {
                        (Some(name), None, Some(t), None) => {
                            attrs.named.push((name, use_, Some(SimpleType::Alias(FullName::new(t.0, t.1)))));
                        },
                        (Some(name), None, None, Some(t)) => {
                            let t = self.process_local_simple_type(t);
                            attrs.named.push((name, use_, Some(t.type_)));
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
                    let mut ref_ = FullName::new(e.attr_ref.0, e.attr_ref.1);
                    for (key, &value) in e.attrs.iter() {
                        match key.as_tuple() {
                            (SCHEMA_URI, "ref") => (),
                            _ => panic!("Unknown attribute {} in <attributeGroup>", key),
                        }
                    }
                    attrs.group_refs.push(ref_);
                },
            }
        }
        if attr_decls.any_attribute.is_some() {
            attrs.any_attributes = true;
        }
        attrs
    }


}
