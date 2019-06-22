use std::fmt::Debug;
use std::hash::Hash;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use xmlparser::Token as XmlToken;
use xmlparser::{TextUnescape, XmlSpace};

use parser::*;
use names::*;
use support::Facets;
use primitives::{QName,NcName,AnyUri,NonNegativeInteger};

pub const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

macro_rules! register_rich_type {
    ($self:expr, $rich_type:expr) => {{
        let id = TypeId::new();
        $self.anonymous_types.insert(id, $rich_type);
        id
    }}
}

macro_rules! register_rich_simple_type {
    ($self:expr, $rich_type:expr) => {{
        let id = SimpleTypeId::new();
        $self.anonymous_simple_types.insert(id, $rich_type);
        id
    }}
}

macro_rules! many {
    ($self:expr, $min_occurs:expr, $max_occurs:expr, $id:expr) => {{
        if $min_occurs != 1 || $max_occurs != 1 {
            register_rich_type!($self, RichType::new(
                NameHint::new("many"),
                Type::Many($min_occurs, $max_occurs, $id),
                Documentation::new(),
                ))
        }
        else {
            $id
        }
    }}
}

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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attrs<'input> {
    pub named: Vec<(FullName<'input>, AttrUse, Option<SimpleTypeId>)>,
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

static TYPE_NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId(usize);

impl TypeId {
    pub fn new() -> TypeId {
        TypeId(TYPE_NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

static SIMPLE_TYPE_NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleTypeId(usize);

impl SimpleTypeId {
    pub fn new() -> SimpleTypeId {
        SimpleTypeId(SIMPLE_TYPE_NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type<'input> {
    Any,
    Empty,
    Alias(FullName<'input>),
    AliasId(TypeId),
    AliasSimpleId(SimpleTypeId),
    Extension(FullName<'input>, TypeId),
    Restriction(FullName<'input>, TypeId),
    ElementRef(FullName<'input>),
    Element(String),
    GroupRef(FullName<'input>),
    Choice(Vec<TypeId>),
    Sequence(Vec<TypeId>),
    Many(usize, usize, TypeId),
    Simple(SimpleType<'input>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleType<'input> {
    Primitive(&'static str, &'static str),
    Alias(FullName<'input>),
    AliasId(SimpleTypeId),
    Restriction(FullName<'input>, Facets<'input>),
    List(SimpleTypeId),
    Union(Vec<SimpleTypeId>),
    Empty,
}

#[derive(Debug)]
pub struct Processor<'ast, 'input: 'ast> {
    pub target_namespace: Option<&'input str>,
    pub element_form_default_qualified: bool,
    pub attribute_form_default_qualified: bool,
    pub elements: HashMap<FullName<'input>, RichType<'input, Type<'input>>>,
    pub types: HashMap<FullName<'input>, TypeId>,
    pub simple_types: HashMap<FullName<'input>, (RichType<'input, SimpleType<'input>>, Documentation<'input>)>,
    pub sequences: HashMap<Vec<RichType<'input, Type<'input>>>, (HashSet<String>, Documentation<'input>)>,
    pub groups: HashMap<FullName<'input>, TypeId>,
    pub attribute_groups: HashMap<FullName<'input>, Attrs<'input>>,
    pub inline_elements: HashMap<(Option<&'input str>, &'input str, Attrs<'input>, Type<'input>), (HashSet<String>, Documentation<'input>)>,

    pub lists: HashMap<RichType<'input, SimpleType<'input>>, HashSet<String>>,
    pub unions: HashMap<Vec<RichType<'input, SimpleType<'input>>>, HashSet<String>>,
    pub simple_restrictions: HashSet<(FullName<'input>, Facets<'input>)>,
    pub substitution_groups: HashMap<FullName<'input>, Vec<FullName<'input>>>,

    pub anonymous_types: HashMap<TypeId, RichType<'input, Type<'input>>>,
    pub anonymous_simple_types: HashMap<SimpleTypeId, RichType<'input, SimpleType<'input>>>,

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
            types: HashMap::new(),
            groups: HashMap::new(),
            lists: HashMap::new(),
            unions: HashMap::new(),
            sequences: HashMap::new(),
            attribute_groups: HashMap::new(),
            inline_elements: HashMap::new(),
            simple_types: HashMap::new(),
            simple_restrictions: HashSet::new(),
            substitution_groups: HashMap::new(),
            anonymous_types: HashMap::new(),
            anonymous_simple_types: HashMap::new(),
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
            ) -> TypeId {
        let inline_elements::GroupRef { ref attrs, ref attr_id, ref attr_ref, ref attr_min_occurs, ref attr_max_occurs, ref annotation } = group_ref;
        let ref_ = attr_ref;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);

        let ref_ = FullName::from_qname(ref_, self.target_namespace);
        let id = register_rich_type!(self, RichType::new(
            NameHint::new(ref_.local_name()),
            Type::GroupRef(ref_),
            self.process_annotation(&annotation.iter().collect()),
            ));
        many!(self, min_occurs, max_occurs, id)
    }

    fn process_named_group(&mut self, 
            group: &'ast xs::Group<'input>,
            ) -> TypeId {
        let xs::Group { ref attrs, ref attr_id, ref attr_name, ref annotation, choice_all_choice_sequence: ref content } = group;
        let name = attr_name;
        let max_occurs = 1;
        let min_occurs = 1;

        let mut id = match content {
            enums::ChoiceAllChoiceSequence::All(_) => unimplemented!("all"),
            enums::ChoiceAllChoiceSequence::Choice(e) => self.process_choice(e, true),
            enums::ChoiceAllChoiceSequence::Sequence(e) => self.process_sequence(e, true),
        };

        let type_ = self.anonymous_types.get_mut(&id).unwrap();
        type_.doc.extend(&self.process_annotation(&annotation.iter().collect()));
        let doc = type_.doc.clone();

        let name = FullName::new(self.target_namespace, name.0);
        self.groups.insert(name, id);
        let id = register_rich_type!(self, RichType::new(
            NameHint::from_fullname(&name),
            Type::GroupRef(name),
            doc.clone(),
            ));
        many!(self, min_occurs, max_occurs, id)
    }

    fn process_attribute_group(&mut self, group: &'ast xs::AttributeGroup<'input>) {
        let name = &group.attr_name;
        let attrs = self.process_attr_decls(&group.attr_decls);
        let name = FullName::new(self.target_namespace, name.0);
        self.attribute_groups.insert(name, attrs);
    }

    fn process_local_simple_type(&mut self,
            simple_type: &'ast inline_elements::LocalSimpleType<'input>,
            ) -> SimpleTypeId {
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
            ) -> SimpleTypeId {
        let xs::SimpleType { ref attrs, ref attr_id, ref attr_name, ref attr_final, ref annotation, ref simple_derivation } = simple_type;
        let annotation: Vec<_> = annotation.iter().collect();
        let name = attr_name;
        let name = FullName::new(self.target_namespace, name.0);
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let id = match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => 
                self.process_simple_restriction(e, annotation.clone()),
            xs::SimpleDerivation::List(ref e) => self.process_list(e, annotation.clone()),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e, annotation.clone()),
        };

        let doc = self.process_annotation(&annotation);
        self.anonymous_simple_types.get_mut(&id).unwrap().name_hint = NameHint::from_fullname(&name); // TODO: enrich the name hint instead of replacing it
        id
    }

    fn process_list(&mut self,
            list: &'ast xs::List<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> SimpleTypeId {
        let item_type = list.attr_item_type;
        let item_type = item_type.as_ref().map(|n| FullName::from_qname(n, self.target_namespace));
        
        let (name_hint, item_type_id) = match (item_type, &list.local_simple_type) {
            (None, Some(st)) => {
                let id: SimpleTypeId = self.process_local_simple_type(st);
                let ty = self.anonymous_simple_types.get_mut(&id).unwrap();
                ty.doc.extend(&self.process_annotation(&annotation));
                (ty.name_hint, id)
            },
            (Some(n), None) => {
                let name_hint = NameHint::new(n.local_name());
                let id: SimpleTypeId = register_rich_simple_type!(self, RichType::new(
                    name_hint,
                    SimpleType::Alias(n),
                    Documentation::new(),
                    ));
                (name_hint, id)
            },
            (None, None) => panic!("<list> with no itemType or child type."),
            (Some(ref t1), Some(ref t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        let mut name_hint = name_hint.clone();
        name_hint.push("list");
        let doc = self.process_annotation(&annotation);
        register_rich_simple_type!(self, RichType::new(
            name_hint,
            SimpleType::List(item_type_id),
            doc,
            ))
    }

    fn process_union(&mut self,
            union: &'ast xs::Union<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> SimpleTypeId {
        let default_vec = Vec::new();
        let mut name_hint = NameHint::new("union");
        let member_types = union.attr_member_types.as_ref().map(|l| &l.0).unwrap_or(&default_vec);
        let mut member_types: Vec<SimpleTypeId> = member_types.iter().map(|name| {
            let name = FullName::from_qname(name, self.target_namespace);
            name_hint.push(name.local_name());
            register_rich_simple_type!(self, RichType::new(
                    NameHint::new(name.local_name()),
                    SimpleType::Alias(name),
                    Documentation::new(),
                    ))
        }).collect();

        for t in union.local_simple_type.iter() {
            let id = self.process_local_simple_type(t);
            name_hint.extend(&self.anonymous_simple_types.get(&id).unwrap().name_hint);
            member_types.push(id)
        }

        let doc = self.process_annotation(&annotation);
        let name = name_from_hint(&name_hint).unwrap();
        register_rich_simple_type!(self, RichType::new(
            name_hint,
            SimpleType::Union(member_types),
            doc,
            ))
    }

    fn process_complex_type(&mut self,
            complex_type: &'ast xs::ComplexType<'input>,
            inlinable: bool,
            ) -> TypeId {
        let xs::ComplexType { ref attrs, ref attr_id, ref attr_name, ref attr_mixed, ref attr_abstract, ref attr_final, ref attr_block, ref attr_default_attributes_apply, ref annotation, ref complex_type_model } = complex_type;
        let name = attr_name;
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let mut id = match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(ref model) =>
                self.process_simple_content(model),
            xs::ComplexTypeModel::ComplexContent(ref model) =>
                self.process_complex_content(model, false),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } =>
                self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions, inlinable),
        };
        let ty = self.anonymous_types.get(&id).unwrap();
        ty.doc.extend(&self.process_annotation(&annotation.iter().collect()));

        let doc = ty.doc.clone();
        let name = FullName::new(self.target_namespace, name.0);
        self.anonymous_types.get_mut(&id).unwrap().name_hint = NameHint::from_fullname(&name); // TODO enrich instead of replacing
        id
    }

    fn process_local_complex_type(&mut self,
            complex_type: &'ast inline_elements::LocalComplexType<'input>,
            attr_name: Option<&'ast NcName<'input>>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> TypeId {
        let inline_elements::LocalComplexType { ref attrs, ref attr_id, ref attr_mixed, ref attr_default_attributes_apply, annotation: ref annotation2, ref complex_type_model } = complex_type;
        let name = attr_name;
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let mut id = match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(ref model) =>
                self.process_simple_content(model),
            xs::ComplexTypeModel::ComplexContent(ref model) =>
                self.process_complex_content(model, false),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } =>
                self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions, inlinable),
        };
        let mut ty = self.anonymous_types.get_mut(&id).unwrap();
        ty.doc.extend(&self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())));

        if let Some(name) = name {
            let name = FullName::new(self.target_namespace, name.0);
            ty.name_hint = NameHint::from_fullname(&name); // TODO enrich instead of replacing
        }
        id
    }

    fn process_complete_content_model(&mut self,
            open_content: &'ast Option<Box<xs::OpenContent<'input>>>,
            type_def_particle: &'ast Option<Box<xs::TypeDefParticle<'input>>>,
            attr_decls: &'ast xs::AttrDecls<'input>,
            assertions: &'ast xs::Assertions<'input>,
            inlinable: bool,
            ) -> TypeId {
        let id = match type_def_particle.as_ref() {
            Some(type_def_particle) => self.process_type_def_particle(type_def_particle, inlinable),
            None => register_rich_type!(self, RichType::new(
                NameHint::new("empty_particle"),
                Type::Empty,
                Documentation::new()
            )),
        };
        self.anonymous_types.get_mut(&id).unwrap().add_attrs(self.process_attr_decls(attr_decls));
        id
    }

    fn process_simple_content(&mut self, model: &'ast xs::SimpleContent<'input>) -> TypeId {
        let xs::SimpleContent { ref attrs, ref attr_id, ref annotation, ref choice_restriction_extension } = model;

        panic!("not implemented");
    }

    fn process_complex_content(&mut self, model: &'ast xs::ComplexContent<'input>, inlinable: bool) -> TypeId {
        let xs::ComplexContent { ref attrs, ref attr_id, ref attr_mixed, ref annotation, ref choice_restriction_extension } = model;
        let annotation = annotation.iter().collect();
        match choice_restriction_extension {
            enums::ChoiceRestrictionExtension::Restriction(ref r) => {
                let inline_elements::ComplexRestrictionType {
                    ref attrs, ref attr_id, ref attr_base, annotation: ref annotation2,
                    ref sequence_open_content_type_def_particle,
                    ref attr_decls, ref assertions
                } = **r;
                let id = match sequence_open_content_type_def_particle {
                    Some(sequences::SequenceOpenContentTypeDefParticle { open_content, type_def_particle }) =>
                        self.process_complex_restriction(attr_base, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref())),
                    None => {
                        register_rich_type!(self, RichType::new(
                            NameHint::new("empty_extension"),
                            Type::Empty,
                            self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                            ))
                    },
                };
                self.anonymous_types.get_mut(&id).unwrap().add_attrs(self.process_attr_decls(attr_decls));
                id
            },
            enums::ChoiceRestrictionExtension::Extension(ref e) => {
                let inline_elements::ExtensionType {
                    ref attrs, ref attr_base, ref attr_id, annotation: ref annotation2, ref open_content,
                    ref type_def_particle, ref attr_decls, ref assertions
                } = **e;
                let id = match type_def_particle {
                    Some(type_def_particle) =>
                        self.process_extension(attrs, attr_base, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref()), inlinable),
                    None => self.process_trivial_extension(attrs, attr_base, vec_concat_opt(&annotation, annotation2.as_ref())),
                };
                self.anonymous_types.get_mut(&id).unwrap().add_attrs(self.process_attr_decls(attr_decls));
                id
            },
        }
    }

    fn process_complex_restriction(&mut self, 
            attr_base: &'ast QName<'input>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> TypeId {
        // TODO: use the base
        let id = self.process_type_def_particle(type_def_particle, false);
        let base = FullName::from_qname(attr_base, self.target_namespace);
        register_rich_type!(self, RichType::new(
            NameHint::new_empty(),
            Type::Restriction(base, id),
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
            ) -> SimpleTypeId {
        let xs::Restriction { ref attrs, ref attr_id, ref attr_base, annotation: ref annotation2, ref simple_restriction_model } = restriction;
        let base = attr_base;
        let base = base.unwrap_or(QName { namespace: Some(SCHEMA_URI), local_name: "anySimpleType" });
        let xs::SimpleRestrictionModel { ref local_simple_type, ref choice_facet_any } = simple_restriction_model;
        let facets = self.process_facets(choice_facet_any);

        let base = FullName::from_qname(&base, self.target_namespace);

        self.simple_restrictions.insert((base, facets.clone()));

        match local_simple_type {
            Some(inline_elements::LocalSimpleType { ref attrs, ref attr_id, annotation: ref annotation2, ref simple_derivation }) => {
                register_rich_simple_type!(self, RichType::new(
                    NameHint::new(base.local_name()),
                    SimpleType::Restriction(base, facets),
                    self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                    ))
            },
            None => {
                register_rich_simple_type!(self, RichType::new(
                    NameHint::new(base.local_name()),
                    SimpleType::Restriction(base, facets),
                    self.process_annotation(&annotation),
                    ))
            },
        }
    }

    fn process_type_def_particle(&mut self, particle: &'ast xs::TypeDefParticle<'input>, inlinable: bool) -> TypeId {
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
            ) -> TypeId {
        let mut id = match particle {
            xs::NestedParticle::Element(e) => self.process_local_element(e),
            xs::NestedParticle::Group(e) => self.process_group_ref(e),
            xs::NestedParticle::Choice(e) => self.process_choice(e, inlinable),
            xs::NestedParticle::Sequence(e) => self.process_sequence(e, inlinable),
            xs::NestedParticle::Any(e) => self.process_any(e, Vec::new()),
        };

        self.anonymous_types.get_mut(&id).unwrap().doc.extend(&self.process_annotation(&annotation));
        id
    }

    fn process_any(&mut self,
            any: &'ast xs::Any<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> TypeId {
        register_rich_type!(self, RichType::new(
            NameHint::new("any"),
            Type::Any,
            self.process_annotation(&annotation),
            ))
    }

    fn process_sequence(&mut self,
            seq: &'ast xs::Sequence<'input>,
            inlinable: bool,
            ) -> TypeId {
        let xs::Sequence { ref attrs, ref attr_id, ref attr_min_occurs, ref attr_max_occurs, ref annotation, ref nested_particle } = seq;
        let particles = nested_particle;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);
        let mut items = Vec::new();
        let mut name_hint = NameHint::new("sequence");
        let id = if inlinable && particles.len() == 1 {
            self.process_nested_particle(particles.get(0).unwrap(), annotation.iter().collect(), inlinable)
        }
        else {
            for particle in particles.iter() {
                let id = self.process_nested_particle(particle, vec![], false);
                name_hint.extend(&self.anonymous_types.get(&id).unwrap().name_hint);
                items.push(id);
            }
            let doc = self.process_annotation(&annotation.iter().collect());
            register_rich_type!(self, RichType::new(
                name_hint.clone(),
                Type::Sequence(items),
                doc,
                ))
        };
        many!(self, min_occurs, max_occurs, id)
    }

    fn process_choice(&mut self,
            choice: &'ast xs::Choice<'input>,
            inlinable: bool
            ) -> TypeId {
        let xs::Choice { ref attrs, ref attr_id, ref attr_min_occurs, ref attr_max_occurs, ref annotation, ref nested_particle } = choice;
        let particles = nested_particle;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);
        let mut items = Vec::new();
        let mut name_hint = NameHint::new("choice");
        let doc = self.process_annotation(&annotation.iter().collect());
        let id = if particles.len() == 1 {
            let particle = particles.get(0).unwrap();
            self.process_nested_particle(particle, annotation.iter().collect(), inlinable)
        }
        else {
            for particle in particles.iter() {
                let id = self.process_nested_particle(particle, vec![], false);
                name_hint.extend(&self.anonymous_types.get(&id).unwrap().name_hint);
                items.push(id);
            }
            register_rich_type!(self, RichType::new(
                name_hint,
                Type::Choice(items),
                doc.clone(),
            ))
        };
        many!(self, min_occurs, max_occurs, id)
    }

    fn process_trivial_extension(&mut self,
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> TypeId {
        let base = FullName::from_qname(&attr_base, self.target_namespace);
        register_rich_type!(self, RichType::new(
            NameHint::new_empty(),
            Type::Alias(base),
            self.process_annotation(&annotation),
            ))
    }

    fn process_extension(&mut self,
            attrs: &'ast HashMap<FullName<'input>, &'input str>,
            attr_base: &'ast QName<'input>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> TypeId {
        let base = FullName::from_qname(attr_base, self.target_namespace);
        register_rich_type!(self, RichType::new(
            NameHint::new_empty(),
            Type::Extension(base, self.process_type_def_particle(type_def_particle, inlinable)),
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
        let ty = match (type_attr, &child_type) {
            (None, Some(ref c)) => match c {
                enums::Type::SimpleType(ref e) => {
                    let mut id = self.process_local_simple_type(e);
                    let mut ty = self.anonymous_simple_types.remove(&id).unwrap();
                    ty.doc.extend(&self.process_annotation(&annotation));
                    ty.into_complex()
                },
                enums::Type::ComplexType(ref e) => {
                    let id = self.process_local_complex_type(e, Some(attr_name), annotation, false);
                    self.anonymous_types.remove(&id).unwrap()
                },
            },
            (Some(t), None) => {
                let t = FullName::from_qname(&t, self.target_namespace);
                RichType::new(
                    NameHint::new(t.local_name()),
                    Type::Alias(t.into()),
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

        self.elements.insert(name, ty);
    }

    fn process_local_element(&mut self,
            element: &'ast inline_elements::LocalElement<'input>,
            ) -> TypeId {
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
            many!(self, min_occurs, max_occurs,
                register_rich_type!(self, RichType::new(
                    NameHint::new(ref_.local_name()),
                    Type::ElementRef(ref_),
                    self.process_annotation(&annotation),
                    )))
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
                    let mut id = match c {
                        enums::Type::SimpleType(ref e) => {
                            let id = self.process_local_simple_type(e);
                            let mut ty = self.anonymous_simple_types.remove(&id).unwrap();
                            ty.doc.extend(&self.process_annotation(&annotation));
                            register_rich_type!(self, ty.into_complex())
                        },
                        enums::Type::ComplexType(ref e) => {
                            self.process_local_complex_type(e, None, annotation, false)
                        },
                    };
                    let t = self.anonymous_types.get_mut(&id).unwrap();
                    let mut name_hint = NameHint::new(name);
                    name_hint.extend(&t.name_hint);
                    let struct_name = name_from_hint(&name_hint).unwrap();
                    let (elems, doc) = self.inline_elements.entry((namespace, name, t.attrs, t.type_))
                            .or_insert((HashSet::new(), Documentation::new()));
                    elems.insert(struct_name.clone());
                    t.doc.extend(doc);
                    many!(self, min_occurs, max_occurs,
                        register_rich_type!(self, RichType::new(
                            NameHint::new(name),
                            Type::Element(struct_name),
                            t.doc,
                            )))
                },
                (Some(t), None) => {
                    let name_hint1 = NameHint::new(t.local_name);
                    let mut name_hint2 = NameHint::new(name);
                    name_hint2.push(t.local_name);
                    // TODO: move this heuristic in names.rs
                    let name_hint = if t.local_name.to_lowercase().contains(&name.to_lowercase()) {
                        name_hint1
                    }
                    else {
                        name_hint2
                    };
                    let struct_name = name_from_hint(&name_hint).unwrap();
                    let mut doc = self.process_annotation(&annotation);
                    let t = FullName::from_qname(t, self.target_namespace);
                    let (elems, doc2) = self.inline_elements.entry((namespace, name, Attrs::new(), Type::Alias(t)))
                            .or_insert((HashSet::new(), Documentation::new()));
                    elems.insert(struct_name.clone());
                    doc.extend(doc2);
                    many!(self, min_occurs, max_occurs,
                        register_rich_type!(self, RichType::new(
                            NameHint::new(name),
                            Type::Element(struct_name),
                            doc,
                            )))
                },
                (None, None) => {
                    many!(self, min_occurs, max_occurs,
                        register_rich_type!(self, RichType::new(
                            NameHint::new("empty"),
                            Type::Empty,
                            self.process_annotation(&annotation),
                            )))
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
                            let id = register_rich_simple_type!(self, RichType::new(
                                NameHint::new(&t.local_name()),
                                SimpleType::Alias(t),
                                Documentation::new(),
                                ));
                            attrs.named.push((name, use_, Some(id)));
                        },
                        (Some(name), None, None, Some(t)) => {
                            let id = self.process_local_simple_type(t);
                            attrs.named.push((name, use_, Some(id)));
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
