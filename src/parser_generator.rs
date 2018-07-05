use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

use codegen as cg;
use heck::{SnakeCase, CamelCase};
use xmlparser::Token as XmlToken;
use xmlparser::{TextUnescape, XmlSpace};

//use support::*;
use parser::*;
use names::*;

const KEYWORDS: &[&'static str] = &["override"];
fn escape_keyword(name: &str) -> String {
    if KEYWORDS.contains(&name) {
        format!("{}_", name)
    }
    else {
        name.to_string()
    }
}

const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

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
struct Documentation<'input>(Vec<&'input str>);
impl<'input> Documentation<'input> {
    fn new() -> Documentation<'input> {
        Documentation(Vec::new())
    }
    fn push(&mut self, s: &'input str) {
        self.0.push(s);
    }
    fn extend(&mut self, v: &Documentation<'input>) {
        self.0.extend(v.0.iter());
    }
}

impl<'input> ToString for Documentation<'input> {
    fn to_string(&self) -> String {
        self.0.iter().map(|doc| TextUnescape::unescape(doc, XmlSpace::Default)).collect::<Vec<_>>().join("\n")
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RichType<'input> {
    name_hint: NameHint<'input>,
    type_: Type<'input>,
    doc: Documentation<'input>,
}
impl<'input> RichType<'input> {
    fn new(name_hint: NameHint<'input>, type_: Type<'input>, doc: Documentation<'input>) -> RichType<'input> {
        RichType { name_hint, type_, doc }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Type<'input> {
    Any,
    Empty,
    Alias(FullName<'input>),
    List(Box<RichType<'input>>),
    Union(Vec<RichType<'input>>),
    Extension(FullName<'input>, Box<RichType<'input>>),
    ElementRef(usize, usize, FullName<'input>),
    Element(usize, usize, String),
    Group(usize, usize, FullName<'input>),
    Choice(usize, usize, String),
    InlineChoice(Vec<RichType<'input>>),
    Sequence(usize, usize, String),
    InlineSequence(Vec<RichType<'input>>),
}

#[derive(Debug)]
pub struct ParserGenerator<'ast, 'input: 'ast> {
    namespaces: Namespaces<'input>,
    element_form_default_qualified: bool,
    attribute_form_default_qualified: bool,
    elements: HashMap<FullName<'input>, RichType<'input>>,
    types: HashMap<FullName<'input>, (RichType<'input>, Documentation<'input>)>,
    choices: HashMap<Vec<RichType<'input>>, HashSet<String>>,
    sequences: HashMap<Vec<RichType<'input>>, (HashSet<String>, Documentation<'input>)>,
    groups: HashMap<FullName<'input>, RichType<'input>>,
    attribute_groups: HashMap<FullName<'input>, &'ast xs::AttributeGroup<'input>>,
    renames: HashMap<String, String>,
    inline_elements: HashMap<(FullName<'input>, Type<'input>), (HashSet<String>, Documentation<'input>)>,
}

impl<'ast, 'input: 'ast> ParserGenerator<'ast, 'input> {
    pub fn new(ast: &'ast xs::Schema<'input>, renames: HashMap<String, String>) -> ParserGenerator<'ast, 'input> {
        let mut target_namespace = None;
        let mut namespaces = HashMap::new();
        let mut element_form_default_qualified = false;
        let mut attribute_form_default_qualified = false;
        for (key, &value) in ast.attrs.iter() {
            match (key.0, key.1) {
                (Some("xml"), "lang") => (),
                (Some("xmlns"), ns) => {
                    let old_value = namespaces.insert(ns, value);
                    if let Some(old_value) = old_value {
                        panic!("Namespace {:?} is defined twice ({} and {})", ns, old_value, value);
                    }
                },
                (None, "targetNamespace") => target_namespace = Some(value),
                (None, "elementFormDefault") => {
                    match value {
                        "qualified" => element_form_default_qualified = true,
                        "unqualified" => element_form_default_qualified = false,
                        _ => panic!("Unknown value: elementFormDefault={:?}", value),
                    }
                },
                (None, "attributeFormDefault") => {
                    match value {
                        "qualified" => attribute_form_default_qualified = true,
                        "unqualified" => attribute_form_default_qualified = false,
                        _ => panic!("Unknown value: attributeFormDefault={:?}", value),
                    }
                },
                (None, "version") => (),
                _ => panic!("Unknown attribute {} on <schema>.", key),
            }
        }
        let target_namespace = target_namespace.expect("No target namespace.");
        ParserGenerator {
            namespaces: Namespaces::new(namespaces, target_namespace),
            element_form_default_qualified,
            attribute_form_default_qualified,
            elements: HashMap::new(),
            types: HashMap::new(),
            groups: HashMap::new(),
            choices: HashMap::new(),
            sequences: HashMap::new(),
            attribute_groups: HashMap::new(),
            renames,
            inline_elements: HashMap::new(),
        }
    }

    pub fn gen(&mut self, ast: &'ast xs::Schema<'input>) -> cg::Scope {
        self.process_ast(ast);
        self.gen_target_scope()
    }

    fn gen_target_scope(&mut self) -> cg::Scope {
        let mut scope = cg::Scope::new();
        scope.raw("extern crate xmlparser;");
        scope.raw("pub use std::collections::HashMap;");
        scope.raw("pub use std::marker::PhantomData;");
        scope.raw("pub use support::*;");
        scope.raw("pub use xmlparser::{Token, ElementEnd};");
        self.create_modules(&mut scope);
        self.gen_choices(&mut scope);
        self.gen_sequences(&mut scope);
        self.gen_elements(&mut scope);
        self.gen_inline_elements(&mut scope);
        self.gen_groups(&mut scope);
        scope
    }

    fn create_modules(&mut self, scope: &mut cg::Scope) {
        let mut mod_names: Vec<_> = self.namespaces.modules().map(|(n,_)| n).collect();
        mod_names.sort();
        for mod_name in mod_names {
            let mut module = scope.new_module(mod_name);
            module.vis("pub");
            module.scope().raw("#[allow(unused_imports)]\nuse super::*;");
        }
    }

    fn process_ast(&mut self, ast: &'ast xs::Schema<'input>) {
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
            xs::Redefinable::SimpleType(ref e) => {
                let xs::SimpleType { ref attrs, ref annotation, ref simple_derivation } = **e;
                self.process_simple_type(attrs, simple_derivation, annotation.iter().collect());
            },
            xs::Redefinable::ComplexType(e) => {
                    let xs::ComplexType { ref attrs, ref annotation, ref complex_type_model } = **e;
                    self.process_complex_type(attrs, complex_type_model, annotation.iter().collect(), inlinable);
                },
            xs::Redefinable::Group(e) => {
                let xs::Group { ref attrs, ref annotation, ref choice_all_choice_sequence } = **e;
                self.process_named_group(attrs, choice_all_choice_sequence, annotation.iter().collect());
            },
            xs::Redefinable::AttributeGroup(e) => self.process_attribute_group(e),
        }
    }

    fn process_annotation(&self, annotation: &Vec<&'ast xs::Annotation<'input>>) -> Documentation<'input> {
        let strings = annotation.iter().flat_map(|xs::Annotation { ref attrs, ref annotation_content }| {
            annotation_content.iter().filter_map(|c| {
                match c {
                    enums::AnnotationContent::Appinfo(_) => None,
                    enums::AnnotationContent::Documentation(e) => {
                        let xs::Documentation { ref attrs, ref sequence_any } = **e;
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
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut ref_ = None;
        let mut max_occurs = 1;
        let mut min_occurs = 1;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "ref") =>
                    ref_ = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        let ref_ = ref_.unwrap();
        let (_, field_name) = ref_.as_tuple();
        RichType::new(
            NameHint::new(field_name),
            Type::Group(min_occurs, max_occurs, ref_),
            self.process_annotation(&annotation),
            )
    }

    fn process_named_group(&mut self, 
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            content: &'ast enums::ChoiceAllChoiceSequence<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut name = None;
        let mut max_occurs = 1;
        let mut min_occurs = 1;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        let name = name.expect("<group> has no name or ref.");

        let type_ = match content {
            enums::ChoiceAllChoiceSequence::All(_) => unimplemented!("all"),
            enums::ChoiceAllChoiceSequence::Choice(e) => {
                let xs::Choice { ref attrs, annotation: ref annotation2, ref nested_particle } = **e;
                self.process_choice(attrs, nested_particle, vec_concat_opt(&annotation, annotation2.as_ref()), true)
            },
            enums::ChoiceAllChoiceSequence::Sequence(e) => {
                let xs::Sequence { ref attrs, annotation: ref annotation2, ref nested_particle } = **e;
                self.process_sequence(attrs, nested_particle, vec_concat_opt(&annotation, annotation2.as_ref()), true)
            },
        };

        let doc = type_.doc.clone();

        self.groups.insert(name, type_);
        RichType::new(
            NameHint::from_fullname(&name),
            Type::Group(min_occurs, max_occurs, name),
            doc,
            )
    }

    fn process_attribute_group(&mut self, group: &'ast xs::AttributeGroup<'input>) {
        let mut name = None;
        for (key, &value) in group.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(value),
                _ => panic!("Unknown attribute {} in <attributeGroup>", key),
            }
        }
        let name = name.expect("<attributeGroup> has no name.");
        self.attribute_groups.insert(self.namespaces.parse_qname(name), group);
    }

    fn process_simple_type(&mut self,
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            simple_derivation: &'ast xs::SimpleDerivation<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut name = None;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <simpleType>", key),
            }
        }
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let ty = match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => {
                let xs::Restriction { ref attrs, annotation: ref annotation2, ref simple_restriction_model } = **e;
                self.process_simple_restriction(attrs, simple_restriction_model, vec_concat_opt(&annotation, annotation2.as_ref()))
            },
            xs::SimpleDerivation::List(ref e) => self.process_list(e, annotation.clone()),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e, annotation.clone()),
        };

        if let Some(name) = name {
            let doc = self.process_annotation(&annotation);
            self.types.insert(name, (ty, doc.clone()));
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

    fn process_list(&mut self,
            list: &'ast xs::List<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut item_type = None;
        for (key, &value) in list.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "itemType") => item_type = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <list>", key),
            }
        }
        
        let item_type = match (item_type, &list.local_simple_type) {
            (None, Some(st)) => {
                let inline_elements::LocalSimpleType { attrs, annotation: annotation2, simple_derivation } = st;
                self.process_simple_type(attrs, simple_derivation, vec_concat_opt(&annotation, annotation2.as_ref()))
            },
            (Some(n), None) => {
                RichType::new(
                    NameHint::new_empty(),
                    Type::Alias(n),
                    self.process_annotation(&annotation),
                    )
            },
            (None, None) => panic!("<list> with no itemType or child type."),
            (Some(ref t1), Some(ref t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        let mut name_hint = item_type.name_hint.clone();
        name_hint.push("list");
        let doc = self.process_annotation(&annotation);
        RichType::new(
            name_hint,
            Type::List(Box::new(item_type)),
            doc,
            )
    }

    fn process_union(&mut self,
            union: &'ast xs::Union<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut member_types = Vec::new();
        for (key, &value) in union.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "memberTypes") => {
                    member_types = value.split(" ").map(|s| {
                        let name = self.namespaces.parse_qname(s);
                        let (_, field_name) = name.as_tuple();
                        RichType::new(
                            NameHint::new(field_name),
                            Type::Alias(name),
                            self.process_annotation(&annotation),
                            )
                    }).collect()
                },
                _ => panic!("Unknown attribute {} in <union>", key),
            }
        }

        let mut name_hint = NameHint::new("union");
        for t in union.local_simple_type.iter() {
            let ty = {
                let inline_elements::LocalSimpleType { attrs, annotation: annotation2, simple_derivation } = t;
                self.process_simple_type(attrs, simple_derivation, annotation2.iter().collect())
            };
            name_hint.extend(&ty.name_hint);
            member_types.push(ty)
        }

        let doc = self.process_annotation(&annotation);
        RichType::new(
            name_hint,
            Type::Union(member_types),
            doc,
            )
    }

    fn process_complex_type(&mut self,
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            model: &'ast xs::ComplexTypeModel<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> RichType<'input> {
        let mut name = None;
        let mut abstract_ = false;
        let mut mixed = false;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
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
        let ty = match model {
            xs::ComplexTypeModel::SimpleContent(_) => unimplemented!("simpleContent"),
            xs::ComplexTypeModel::ComplexContent(ref model) => self.process_complex_content(model, false),
            xs::ComplexTypeModel::CompleteContentModel { ref open_content, ref type_def_particle, ref attr_decls, ref assertions } => self.process_complete_content_model(open_content, type_def_particle, attr_decls, assertions, inlinable),
        };

        if let Some(name) = name {
            let doc = self.process_annotation(&annotation);
            self.types.insert(name, (ty, doc.clone()));
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
            ) -> RichType<'input> {
        self.process_type_def_particle(type_def_particle.as_ref().unwrap(), inlinable)
    }

    fn process_complex_content(&mut self, model: &'ast xs::ComplexContent<'input>, inlinable: bool) -> RichType<'input> {
        let xs::ComplexContent { ref attrs, ref annotation, ref choice_restriction_extension } = model;
        let annotation = annotation.iter().collect();
        match choice_restriction_extension {
            enums::ChoiceRestrictionExtension::Restriction(ref r) => {
                let inline_elements::ComplexRestrictionType {
                    ref attrs, annotation: ref annotation2,
                    ref sequence_open_content_type_def_particle,
                    ref attr_decls, ref assertions
                } = **r;
                match sequence_open_content_type_def_particle {
                    Some(sequences::SequenceOpenContentTypeDefParticle { open_content, type_def_particle }) =>
                        self.process_restriction(attrs, type_def_particle),
                    None => {
                        RichType::new(
                            NameHint::new("empty_extension"),
                            Type::Empty,
                            self.process_annotation(&vec_concat_opt(&annotation, annotation2.as_ref())),
                            )
                    },
                }
            },
            enums::ChoiceRestrictionExtension::Extension(ref e) => {
                let inline_elements::ExtensionType {
                    ref attrs, annotation: ref annotation2, ref open_content,
                    ref type_def_particle, ref attr_decls, ref assertions
                } = **e;
                match type_def_particle {
                    Some(type_def_particle) =>
                        self.process_extension(attrs, type_def_particle, vec_concat_opt(&annotation, annotation2.as_ref()), inlinable),
                    None => self.process_simple_extension(attrs, vec_concat_opt(&annotation, annotation2.as_ref())),
                }
            },
        }
    }

    fn process_restriction(&mut self, 
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            ) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "base") => base = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <restriction>", key),
            }
        }
        let base = base.expect("<restriction> has no base");
        // TODO: use the base
        self.process_type_def_particle(type_def_particle, false)
    }

    fn process_simple_restriction(&mut self, 
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            model: &'ast xs::SimpleRestrictionModel<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "base") => base = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <restriction>", key),
            }
        }
        let base = base.expect("<restriction> has no base");
        let xs::SimpleRestrictionModel { ref local_simple_type, ref choice_facet_any } = model;
        match local_simple_type {
            Some(inline_elements::LocalSimpleType { ref attrs, annotation: ref annotation2, ref simple_derivation }) =>
                self.process_simple_type(attrs, simple_derivation, vec_concat_opt(&annotation, annotation2.as_ref())),
            None => {
                RichType::new(
                    NameHint::new(base.as_tuple().1),
                    Type::Empty,
                    self.process_annotation(&annotation),
                    )
            },
        }
    }

    fn process_type_def_particle(&mut self, particle: &'ast xs::TypeDefParticle<'input>, inlinable: bool) -> RichType<'input> {
        match particle {
            xs::TypeDefParticle::Group(e) => {
                let inline_elements::GroupRef { ref attrs, ref annotation } = **e;
                self.process_group_ref(attrs, annotation.iter().collect())
            },
            xs::TypeDefParticle::All(_) => unimplemented!("all"),
            xs::TypeDefParticle::Choice(e) => {
                let xs::Choice { ref attrs, ref annotation, ref nested_particle } = **e;
                self.process_choice(attrs, nested_particle, annotation.iter().collect(), inlinable)
            },
            xs::TypeDefParticle::Sequence(e) => {
                let xs::Sequence { ref attrs, ref annotation, ref nested_particle } = **e;
                self.process_sequence(attrs, nested_particle, annotation.iter().collect(), inlinable)
            },
        }
    }

    fn process_nested_particle(&mut self,
            particle: &'ast xs::NestedParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool
            ) -> RichType<'input> {
        match particle {
            xs::NestedParticle::Element(e) => {
                let inline_elements::LocalElement { ref attrs, annotation: ref annotation2, ref type_, ref alternative_alt_type, ref identity_constraint } = **e;
                self.process_element(attrs, type_, vec_concat_opt(&annotation, annotation2.as_ref()))
            },
            xs::NestedParticle::Group(e) => {
                let inline_elements::GroupRef { ref attrs, annotation: ref annotation2 } = **e;
                self.process_group_ref(attrs, vec_concat_opt(&annotation, annotation2.as_ref()))
            },
            xs::NestedParticle::Choice(e) => {
                let xs::Choice { ref attrs, annotation: ref annotation2, ref nested_particle } = **e;
                self.process_choice(attrs, nested_particle, vec_concat_opt(&annotation, annotation2.as_ref()), inlinable)
            },
            xs::NestedParticle::Sequence(e) => {
                let xs::Sequence { ref attrs, annotation: ref annotation2, ref nested_particle } = **e;
                self.process_sequence(attrs, nested_particle, vec_concat_opt(&annotation, annotation2.as_ref()), inlinable)
            },
            xs::NestedParticle::Any(e) => self.process_any(e, annotation),
        }
    }

    fn process_any(&mut self,
            any: &'ast xs::Any<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        RichType::new(
            NameHint::new("any"),
            Type::Any,
            self.process_annotation(&annotation),
            )
    }

    fn process_sequence(&mut self,
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            particles: &'ast Vec<xs::NestedParticle<'input>>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> RichType<'input> {
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
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
            self.process_nested_particle(particles.get(0).unwrap(), annotation, inlinable)
        }
        else {
            for particle in particles.iter() {
                let ty = self.process_nested_particle(particle, vec![], false);
                name_hint.extend(&ty.name_hint);
                items.push(ty);
            }
            let doc = self.process_annotation(&annotation);
            if min_occurs == 1 && max_occurs == 1 {
                RichType::new(
                    name_hint,
                    Type::InlineSequence(items),
                    doc,
                    )
            }
            else {
                let name = self.namespaces.name_from_hint(&name_hint).unwrap();
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
            attrs: &HashMap<QName<'input>, &'input str>,
            particles: &'ast Vec<xs::NestedParticle<'input>>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool
            ) -> RichType<'input> {
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
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
            let RichType { name_hint, type_, doc } =
                self.process_nested_particle(particle, annotation, inlinable);
            match (min_occurs, max_occurs, type_) {
                (_, _, Type::Element(1, 1, e)) => return RichType {
                    name_hint, type_: Type::Element(min_occurs, max_occurs, e), doc },
                (_, _, Type::Group(1, 1, e)) => return RichType {
                    name_hint, type_: Type::Group(min_occurs, max_occurs, e), doc },
                (_, _, Type::Choice(1, 1, e)) => return RichType {
                    name_hint, type_: Type::Choice(min_occurs, max_occurs, e), doc },
                (_, _, Type::Sequence(1, 1, e)) => return RichType {
                    name_hint, type_: Type::Sequence(min_occurs, max_occurs, e), doc },
                (1, 1, type_) => return RichType { name_hint, type_, doc },
                (_, _, type_) => {
                    let name = self.namespaces.name_from_hint(&name_hint).unwrap();
                    let items = vec![RichType { name_hint: name_hint.clone(), type_, doc: doc.clone() }];
                    let (names, docs) = self.sequences.entry(items)
                        .or_insert((HashSet::new(), Documentation::new()));
                    names.insert(name.clone());
                    docs.extend(&doc);
                    let type_ = Type::Sequence(min_occurs, max_occurs, name);
                    return RichType { name_hint, type_, doc }
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
        let doc = self.process_annotation(&annotation);
        match (min_occurs, max_occurs, inlinable) {
            (1, 1, true) => {
                RichType::new(
                    name_hint,
                    Type::InlineChoice(items),
                    doc,
                    )
            },
            (_, _, _) => {
                let name = self.namespaces.name_from_hint(&name_hint).unwrap();
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

    fn process_simple_extension(&mut self,
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "base") => base = Some(value),
                _ => panic!("Unknown attribute {} in <extension>", key),
            }
        }
        let base = base.expect("<extension> has no base");
        let base = self.namespaces.parse_qname(base);
        RichType::new(
            NameHint::new_empty(),
            Type::Alias(base),
            self.process_annotation(&annotation),
            )
    }

    fn process_extension(&mut self,
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            type_def_particle: &'ast xs::TypeDefParticle<'input>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            inlinable: bool,
            ) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "base") => base = Some(value),
                _ => panic!("Unknown attribute {} in <extension>", key),
            }
        }
        let base = base.expect("<extension> has no base");
        let base = self.namespaces.parse_qname(base);
        RichType::new(
            NameHint::new_empty(),
            Type::Extension(base, Box::new(self.process_type_def_particle(type_def_particle, inlinable))),
            self.process_annotation(&annotation),
            )
    }

    fn process_toplevel_element(&mut self, element: &'ast xs::Element<'input>) {
        let mut name = None;
        let mut type_attr = None;
        let mut abstract_ = false;
        let mut substitution_group = None;
        for (key, &value) in element.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "id") =>
                    (),
                (SCHEMA_URI, "type") =>
                    type_attr = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "abstract") => {
                    match value {
                        "true" => abstract_ = true,
                        "false" => abstract_ = false,
                        _ => panic!("Invalid value for abstract attribute: {}", value),
                    }
                },
                (SCHEMA_URI, "substitutionGroup") =>
                    substitution_group = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in toplevel <element>", key),
            }
        }
        let name = name.expect("<element> has no name.");
        let xs::Element { ref attrs, ref annotation, type_: ref child_type, ref alternative_alt_type, ref identity_constraint } = element;
        let annotation = annotation.iter().collect();
        let type_ = match (type_attr, &child_type) {
            (None, Some(ref c)) => match c {
                enums::Type::SimpleType(ref e) => {
                    let inline_elements::LocalSimpleType { ref attrs, annotation: ref annotation2, ref simple_derivation } = **e;
                    self.process_simple_type(attrs, simple_derivation, vec_concat_opt(&annotation, annotation2.as_ref()))
                },
                enums::Type::ComplexType(ref e) => {
                    let inline_elements::LocalComplexType { ref attrs, annotation: ref annotation2, ref complex_type_model } = **e;
                    self.process_complex_type(attrs, complex_type_model, vec_concat_opt(&annotation, annotation2.as_ref()), false)
                },
            },
            (Some(t), None) => {
                let (_, field_name) = t.as_tuple();
                RichType::new(
                    NameHint::new(field_name),
                    Type::Alias(t),
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

    fn process_element(&mut self,
            attrs: &'ast HashMap<QName<'input>, &'input str>,
            child_type: &'ast Option<enums::Type<'input>>,
            annotation: Vec<&'ast xs::Annotation<'input>>,
            ) -> RichType<'input> {
        let mut name = None;
        let mut ref_ = None;
        let mut type_attr = None;
        let mut abstract_ = false;
        let mut substitution_group = None;
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "id") =>
                    (),
                (SCHEMA_URI, "type") =>
                    type_attr = Some(self.namespaces.parse_qname(value)),
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
                (SCHEMA_URI, "substitutionGroup") =>
                    substitution_group = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "ref") =>
                    ref_ = Some(self.namespaces.parse_qname(value)),
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
            let name = name.expect("<element> has no name.");
            match (type_attr, &child_type) {
                (None, Some(ref c)) => {
                    let t = match c {
                        enums::Type::SimpleType(ref e) => {
                            let inline_elements::LocalSimpleType { ref attrs, annotation: ref annotation2, ref simple_derivation } = **e;
                            self.process_simple_type(attrs, simple_derivation, vec_concat_opt(&annotation, annotation2.as_ref()))
                        },
                        enums::Type::ComplexType(ref e) => {
                            let inline_elements::LocalComplexType { ref attrs, annotation: ref annotation2, ref complex_type_model } = **e;
                            self.process_complex_type(attrs, complex_type_model, vec_concat_opt(&annotation, annotation2.as_ref()), false)
                        },
                    };
                    let (prefix, local) = name.as_tuple();
                    let mut name_hint = NameHint::new(local);
                    name_hint.extend(&t.name_hint);
                    let struct_name = self.namespaces.name_from_hint(&name_hint).unwrap();
                    let mut doc = self.process_annotation(&annotation);
                    doc.extend(&t.doc);
                    let (elems, doc2) = self.inline_elements.entry((name, t.type_))
                            .or_insert((HashSet::new(), Documentation::new()));
                    elems.insert(struct_name.clone());
                    doc.extend(doc2);
                    RichType::new(
                        NameHint::new(local),
                        Type::Element(min_occurs, max_occurs, struct_name),
                        doc,
                        )
                },
                (Some(t), None) => {
                    let (prefix, local) = name.as_tuple();
                    let name_hint1 = NameHint::new(t.as_tuple().1);
                    let mut name_hint2 = NameHint::new(local);
                    name_hint2.push(t.as_tuple().1);
                    // TODO: move this heuristic in names.rs
                    let name_hint = if t.as_tuple().1.to_lowercase().contains(&local.to_lowercase()) {
                        name_hint1
                    }
                    else {
                        name_hint2
                    };
                    let struct_name = self.namespaces.name_from_hint(&name_hint).unwrap();
                    let mut doc = self.process_annotation(&annotation);
                    let (elems, doc2) = self.inline_elements.entry((name, Type::Alias(t)))
                            .or_insert((HashSet::new(), Documentation::new()));
                    elems.insert(struct_name.clone());
                    doc.extend(doc2);
                    RichType::new(
                        NameHint::new(local),
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

    fn gen_choices(&self, scope: &mut cg::Scope) {
        let module = scope.new_module("enums");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut choices: Vec<_> = self.choices.iter().collect();
        choices.sort_by_key(|&(t,names)| (t, names.iter().collect::<Vec<_>>()));
        let mut name_gen = NameGenerator::new();
        for (ref choice, ref names) in choices {
            // TODO: deduplicate
            for name in names.iter() {
                let enum_name = escape_keyword(&name.to_camel_case());
                let enum_name = name_gen.gen_name(enum_name);
                self.gen_choice(module.scope(), &enum_name, choice, &Documentation::new());
            }
        }
    }
    fn gen_choice(&self, scope: &mut cg::Scope, enum_name: &String, items: &Vec<RichType<'input>>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        let enum_name = self.renames.get(enum_name).unwrap_or(enum_name);
        impl_code.push(format!("impl_enum!({},", enum_name));
        {
            let enum_ = scope.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (i, item) in items.iter().enumerate() {
                let mut fields = Vec::new();
                let mut doc = doc.clone();
                {
                    let mut name_gen = NameGenerator::new();
                    let field_writer = &mut |variant_name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
                        let variant_name = escape_keyword(&name_gen.gen_name(variant_name.to_snake_case()));
                        let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                        let type_name = escape_keyword(&type_name.to_camel_case());
                        fields.push((variant_name, type_mod_name, min_occurs, max_occurs, type_name));
                    };
                    let doc_writer = &mut |doc2: &Documentation<'input>| doc.extend(doc2);
                    self.write_type_in_struct_def(field_writer, doc_writer, &item.type_);
                }
                enum_.doc(&doc.to_string());
                let variant_name = self.namespaces.name_from_hint(&item.name_hint)
                    .unwrap_or(format!("{}{}", enum_name, i)).to_camel_case();
                let variant_name = escape_keyword(&variant_name.to_camel_case());
                let variant_name = self.renames.get(&variant_name).unwrap_or(&variant_name);
                let mut variant = enum_.new_variant(&variant_name);
                if fields.len() == 1 {
                    let (_, type_mod_name, min_occurs, max_occurs, type_name) = fields.remove(0);
                    match (min_occurs, max_occurs) {
                        (1, 1) => {
                            variant.tuple(&format!("Box<super::{}::{}<'input>>", escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                            impl_code.push(format!("    impl_singleton_variant!({}, {}, Box<{}>),", variant_name, escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                        },
                        (0, 1) => {
                            variant.tuple(&format!("Option<super::{}::{}<'input>>", escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                            impl_code.push(format!("    impl_singleton_variant!({}, {}, Option<{}>),", variant_name, escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                        },
                        (_, _) => {
                            variant.tuple(&format!("Vec<super::{}::{}<'input>>", escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                            impl_code.push(format!("    impl_singleton_variant!({}, {}, Vec<{}>),", variant_name, escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                        },
                    }
                }
                else {
                    impl_code.push(format!("    impl_struct_variant!({},", variant_name));
                    for (field_name, type_mod_name, min_occurs, max_occurs, type_name) in fields {
                        match (min_occurs, max_occurs) {
                            (1, 1) => {
                                impl_code.push(format!("        ({}, {}, Box<{}>),", field_name, type_mod_name, type_name));
                                variant.named(&field_name, &format!("Box<super::{}::{}<'input>>", type_mod_name, type_name));
                            },
                            (0, 1) => {
                                impl_code.push(format!("        ({}, {}, Option<Box<{}> >),", field_name, type_mod_name, type_name));
                                variant.named(&field_name, &format!("Option<Box<super::{}::{}<'input>> >", type_mod_name, type_name));
                            },
                            (_, _) => {
                                impl_code.push(format!("        ({}, {}, Vec<{}>),", field_name, type_mod_name, type_name));
                                variant.named(&field_name, &format!("Vec<super::{}::{}<'input> >", type_mod_name, type_name));
                            },
                        }
                    }
                    impl_code.push(format!("    ),"));
                }
            }
        }
        impl_code.push(");".to_string());
        scope.raw(&impl_code.join("\n"));
    }

    fn gen_sequences(&mut self, scope: &mut cg::Scope) {
        let module = scope.new_module("sequences");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut sequences: Vec<_> = self.sequences.iter().collect();

        sequences.sort_by_key(|&(i,(n,_))| (n.iter().collect::<Vec<_>>(), i));
        for (sequence, (names, doc)) in sequences {
            for name in names.iter() {
                self.gen_group_or_sequence(module, name, &sequence.iter().collect(), doc);
            }
        }
    }

    fn gen_groups(&mut self, scope: &mut cg::Scope) {
        let mut groups: Vec<_> = self.groups.iter().collect();

        groups.sort_by_key(|&(n,_)| n);
        for (&name, group) in groups {
            let mod_name = self.namespaces.get_module_name(name);
            let mut module = scope.get_module_mut(mod_name).unwrap();
            let (mod_name, struct_name) = name.as_tuple();
            if let Type::InlineChoice(ref items) = group.type_ {
                self.gen_choice(module.scope(), &struct_name.to_string().to_camel_case(), items, &group.doc);
            }
            else if let Type::InlineSequence(ref items) = group.type_ {
                self.gen_group_or_sequence(module, struct_name, &items.iter().collect(), &group.doc);
            }
            else {
                self.gen_group_or_sequence(module, struct_name, &vec![group], &group.doc);
            }
        }
    }

    fn gen_group_or_sequence(&self, module: &mut cg::Module, struct_name: &'input str, items: &Vec<&RichType<'input>>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        let struct_name = escape_keyword(&struct_name.to_camel_case());
        let struct_name = self.renames.get(&struct_name).unwrap_or(&struct_name);
        impl_code.push(format!("impl_group_or_sequence!({},", struct_name));
        {
            let mut empty_struct = true;
            let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            let mut name_gen = NameGenerator::new();
            let mut doc = doc.clone();
            {
                let field_writer = &mut |name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
                    empty_struct = false;
                    let name = escape_keyword(&name_gen.gen_name(name.to_snake_case()));
                    let name = self.renames.get(&name).unwrap_or(&name);
                    let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                    let type_name = escape_keyword(&type_name.to_camel_case());
                    let type_name = self.renames.get(&type_name).unwrap_or(&type_name);
                    match (min_occurs, max_occurs) {
                        (1, 1) => {
                            struct_.field(&format!("pub {}", name), &format!("super::{}::{}<'input>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, {}),", name, type_mod_name, type_name))
                        },
                        (0, 1) => {
                            struct_.field(&format!("pub {}", name), &format!("Option<super::{}::{}<'input>>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, Option<{}>),", name, type_mod_name, type_name))
                        },
                        (_, _) => {
                            struct_.field(&format!("pub {}", name), &format!("Vec<super::{}::{}<'input>>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, Vec<{}>),", name, type_mod_name, type_name))
                        },
                    }
                };
                let doc_writer = &mut |doc2| doc.extend(doc2);
                for item in items {
                    self.write_type_in_struct_def(field_writer, doc_writer, &item.type_);
                }
            }
            struct_.doc(&doc.to_string());
            if empty_struct {
                struct_.tuple_field(&format!("pub ::std::marker::PhantomData<&'input ()>"));
            }
        }
        impl_code.push(");".to_string());
        module.scope().raw(&impl_code.join("\n"));
    }

    fn gen_inline_elements(&mut self, scope: &mut cg::Scope) {
        let module = scope.new_module("inline_elements");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut elements: Vec<_> = self.inline_elements.iter().collect();

        elements.sort_by_key(|&((n, _),(n2,_))| (n, n2.iter().collect::<Vec<_>>()));
        for ((tag_name, element), (struct_names, doc)) in elements {
            // struct_names is always non-empty.

            let mut struct_names: Vec<_> = struct_names.iter().map(|s| s.to_camel_case()).collect();

            // Sort them to get the shortest one, and alias all the others to this one
            struct_names.sort_by_key(|n| n.len()); // TODO: just use a min_by_key
            for alias in &struct_names[1..] {
                module.scope().raw(&format!("pub type {}<'input> = {}<'input>;", alias, struct_names[0]));
            }

            self.gen_element(module, &struct_names[0], tag_name, element, doc);
        }
    }

    fn gen_elements(&mut self, scope: &mut cg::Scope) {
        let mut elements: Vec<_> = self.elements.iter().collect();

        elements.sort_by_key(|&(n,_)| n);
        for (&name, element) in elements {
            let mod_name = self.namespaces.get_module_name(name);
            let mut module = scope.get_module_mut(mod_name).unwrap();
            let (prefix, local) = name.as_tuple();
            let struct_name = escape_keyword(&local.to_camel_case());
            self.gen_element(module, &struct_name, &name, &element.type_, &element.doc);
        }
    }

    fn gen_element(&self, module: &mut cg::Module, struct_name: &str, tag_name: &FullName<'input>, type_: &Type<'input>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        let (_, tag_name) = tag_name.as_tuple();
        impl_code.push(format!("impl_element!({}, \"{}\", {{", struct_name, tag_name));
        {
            let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            struct_.field("pub attrs", "HashMap<QName<'input>, &'input str>");
            let mut name_gen = NameGenerator::new();
            let mut doc = doc.clone();
            {
                let field_writer = &mut |name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
                    let name = escape_keyword(&name_gen.gen_name(name.to_snake_case()));
                    let name = self.renames.get(&name).unwrap_or(&name);
                    let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                    let type_name = escape_keyword(&type_name.to_camel_case());
                    let type_name = self.renames.get(&type_name).unwrap_or(&type_name);
                    match (min_occurs, max_occurs) {
                        (1, 1) => {
                            struct_.field(&format!("pub {}", name), &format!("super::{}::{}<'input>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, {}),", name, type_mod_name, type_name))
                        },
                        (0, 1) => {
                            struct_.field(&format!("pub {}", name), &format!("Option<super::{}::{}<'input>>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, Option<{}>),", name, type_mod_name, type_name))
                        },
                        (0, _) => {
                            struct_.field(&format!("pub {}", name), &format!("Vec<super::{}::{}<'input>>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, Vec<{}>),", name, type_mod_name, type_name))
                        },
                        (_, _) => {
                            struct_.field(&format!("pub {}", name), &format!("Vec<super::{}::{}<'input>>", type_mod_name, type_name));
                            impl_code.push(format!("    ({}, {}, Vec<{}>),", name, type_mod_name, type_name))
                        },
                    }
                };
                let doc_writer = &mut |doc2| doc.extend(doc2);
                self.write_type_in_struct_def(field_writer, doc_writer, type_);
            }
            struct_.doc(&doc.to_string());
        }
        impl_code.push(format!("}});"));
        module.scope().raw(&impl_code.join("\n"));
    }

    fn write_type_in_struct_def<'a, F, G>(&'a self,
            field_writer: &mut F,
            doc_writer: &mut G,
            type_: &'a Type<'input>,
            ) where
            F: FnMut(&'a str, &'a str, usize, usize, &'a str),
            G: FnMut(&'a Documentation<'input>),
            'ast: 'a {
        match &type_ {
            Type::Alias(name) => {
                let (target_type, doc) = self.types.get(name).unwrap();
                doc_writer(doc);
                self.write_type_in_struct_def(field_writer, doc_writer, &target_type.type_);
            },
            Type::InlineSequence(items) => {
                for item in items {
                    self.write_type_in_struct_def(field_writer, doc_writer, &item.type_);
                }
            }
            Type::Sequence(min_occurs, max_occurs, name) => {
                field_writer(name, "sequences", *min_occurs, *max_occurs, name);
            }
            Type::Element(min_occurs, max_occurs, name) => {
                field_writer(name, "inline_elements", *min_occurs, *max_occurs, name);
            }
            Type::Group(min_occurs, max_occurs, name) |
            Type::ElementRef(min_occurs, max_occurs, name) => {
                let (mod_name, type_name) = name.as_tuple();
                let field_name = type_name;
                let mod_name = self.namespaces.module_names.get(mod_name).expect(mod_name);
                field_writer(field_name, mod_name, *min_occurs, *max_occurs, type_name);
            },
            Type::Choice(min_occurs, max_occurs, ref name) => {
                field_writer(name, "enums", *min_occurs, *max_occurs, name);
            },
            Type::Extension(base, ext_type) => {
                let (base_type, doc) = &self.types.get(base).unwrap();
                self.write_type_in_struct_def(field_writer, doc_writer, &base_type.type_);
                self.write_type_in_struct_def(field_writer, doc_writer, &ext_type.type_);
            },
            Type::Empty => (), // TODO ?
            Type::Any => {
                field_writer("any", "support", 1, 1, "Any")
            },
            _ => unimplemented!("writing {:?}", type_),
        }
    }
}
