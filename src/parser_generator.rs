use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

use codegen as cg;
use heck::{SnakeCase, CamelCase};

use support::*;
use generated::UNQUAL::*;
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

#[derive(Debug)]
struct RichType<'input> {
    name_hint: NameHint<'input>,
    type_: Type<'input>,
}
impl<'input> RichType<'input> {
    fn new(name_hint: NameHint<'input>, type_: Type<'input>) -> RichType<'input> {
        RichType { name_hint, type_ }
    }
}

#[derive(Debug)]
enum Type<'input> {
    Any,
    Alias(FullName<'input>),
    List(Box<RichType<'input>>),
    Union(Vec<RichType<'input>>),
    Extension(FullName<'input>, Box<RichType<'input>>),
    Element(usize, usize, FullName<'input>),
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
    types: HashMap<FullName<'input>, RichType<'input>>,
    choices: HashMap<String, Vec<RichType<'input>>>,
    sequences: HashMap<String, Vec<RichType<'input>>>,
    groups: HashMap<FullName<'input>, RichType<'input>>,
    attribute_groups: HashMap<FullName<'input>, &'ast attributeGroup_e<'input>>,
    renames: HashMap<String, String>,
}

impl<'ast, 'input: 'ast> ParserGenerator<'ast, 'input> {
    pub fn new(ast: &'ast schema_e<'input>, renames: HashMap<String, String>) -> ParserGenerator<'ast, 'input> {
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
        }
    }

    pub fn gen(&mut self, ast: &'ast schema_e<'input>) -> cg::Scope {
        self.process_ast(ast);
        self.gen_target_scope(ast)
    }

    fn gen_target_scope(&mut self, ast: &schema_e<'input>) -> cg::Scope {
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
        self.gen_groups(&mut scope);
        scope
    }

    fn create_modules(&mut self, scope: &mut cg::Scope) {
        let mut mod_names: Vec<_> = self.namespaces.modules().map(|(n,_)| n).collect();
        mod_names.sort();
        for mod_name in mod_names {
            let mut module = scope.new_module(mod_name);
            module.vis("pub");
            module.scope().raw("use super::*;");
        }
    }

    fn process_ast(&mut self, ast: &'ast schema_e<'input>) {
        for top_level_item in ast.child.schema_e_inner__extfield0.schema_e_inner__extfield0__seqfield2.0.iter() {
            match top_level_item.schemaTop {
                schemaTop::redefinable(ref r) => self.process_redefinable(r),
                schemaTop::element(ref e) => { self.process_element(e); },
                schemaTop::attribute(_) => unimplemented!("top-level attribute"),
                schemaTop::notation(ref e) => self.process_notation(e),
            }
        }
    }

    fn process_notation(&mut self, notation: &'ast notation_e<'input>) {
        // TODO
    }

    fn process_redefinable(&mut self, r: &'ast redefinable<'input>) {
        match r {
            redefinable::simpleType(e) => { self.process_simple_type(e); },
            redefinable::complexType(e) => { self.process_complex_type(e); },
            redefinable::group(e) => { self.process_group(e); },
            redefinable::attributeGroup(e) => self.process_attribute_group(e),
        }
    }

    fn process_group(&mut self, group: &'ast group_e<'input>) -> RichType<'input> {
        let mut name = None;
        let mut ref_ = None;
        let mut max_occurs = 1;
        let mut min_occurs = 1;
        for (key, &value) in group.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "ref") =>
                    ref_ = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = value.parse().unwrap(),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = parse_max_occurs(value).unwrap(),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        if let Some(ref_) = ref_ {
            if let Some(name) = name {
                panic!("<group> has both ref={:?} and name={:?}", ref_, name)
            }
            let (_, field_name) = ref_.as_tuple();
            RichType::new(NameHint::new(field_name), Type::Group(min_occurs, max_occurs, ref_))
        }
        else {
            let name = name.expect("<group> has no name or ref.");

            let mut items = Vec::new();
            let particles = &((group.child.0).0).0.particle.0;
            if particles.len() == 1 {
                let type_ = self.process_particle(particles.get(0).unwrap(), true);
                self.groups.insert(name, type_);
                RichType::new(NameHint::from_fullname(&name), Type::Group(min_occurs, max_occurs, name))
            }
            else {
                for particle in particles.iter() {
                    items.push(self.process_particle(particle, false));
                }

                assert_eq!(items.len(), 1, "{:?}", items);
                let type_ = items.remove(0);

                self.groups.insert(name, type_);
                RichType::new(NameHint::from_fullname(&name), Type::Group(min_occurs, max_occurs, name))
            }
        }
    }

    fn process_attribute_group(&mut self, group: &'ast attributeGroup_e<'input>) {
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

    fn process_simple_type(&mut self, type_: &'ast simpleType_e<'input>) -> RichType<'input> {
        let mut name = None;
        for (key, &value) in type_.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <simpleType>", key),
            }
        }
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let ty = match (type_.child.0).0.simpleDerivation {
            simpleDerivation::restriction(ref e) => self.process_restriction(e),
            simpleDerivation::list(ref e) => self.process_list(e),
            simpleDerivation::union(ref e) => self.process_union(e),
        };

        if let Some(name) = name {
            self.types.insert(name, ty);
            RichType::new(NameHint::from_fullname(&name), Type::Alias(name))
        }
        else {
            ty
        }
    }

    fn process_list(&mut self, list: &'ast list_e<'input>) -> RichType<'input> {
        let mut item_type = None;
        for (key, &value) in list.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "itemType") => item_type = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <list>", key),
            }
        }
        
        let item_type = match (item_type, &list.child.simpleType.0) {
            (None, Some(st)) => self.process_simple_type(st),
            (Some(n), None) => RichType::new(NameHint::new_empty(), Type::Alias(n)),
            (None, None) => panic!("<list> with no itemType or child type."),
            (Some(ref t1), Some(ref t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        let mut name_hint = item_type.name_hint.clone();
        name_hint.push("list");
        RichType::new(name_hint, Type::List(Box::new(item_type)))
    }

    fn process_union(&mut self, union: &'ast union_e<'input>) -> RichType<'input> {
        let mut member_types = Vec::new();
        for (key, &value) in union.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "memberTypes") => {
                    member_types = value.split(" ").map(|s| {
                        let name = self.namespaces.parse_qname(s);
                        let (_, field_name) = name.as_tuple();
                        RichType::new(NameHint::new(field_name), Type::Alias(name))
                    }).collect()
                },
                _ => panic!("Unknown attribute {} in <union>", key),
            }
        }

        let mut name_hint = NameHint::new("union");
        for t in union.child.simpleType.0.iter() {
            let ty = self.process_simple_type(t);
            name_hint.extend(&ty.name_hint);
            member_types.push(ty)
        }

        RichType::new(name_hint, Type::Union(member_types))
    }

    fn process_complex_type(&mut self, type_: &'ast complexType_e<'input>) -> RichType<'input> {
        let mut name = None;
        let mut abstract_ = false;
        let mut mixed = false;
        for (key, &value) in type_.attrs.iter() {
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
        let ty = match (type_.child.0).0.complexTypeModel {
            complexTypeModel::simpleContent(_) => unimplemented!("simpleContent"),
            complexTypeModel::complexContent(ref model) => self.process_complex_content(model),
            complexTypeModel::complexTypeModel__choicevariant2(ref model) => self.process_other_complex_type_model(model),
        };

        if let Some(name) = name {
            self.types.insert(name, ty);
            RichType::new(NameHint::from_fullname(&name), Type::Alias(name))
        }
        else {
           ty 
        }
    }

    fn process_other_complex_type_model(&mut self, model: &'ast complexTypeModel__choicevariant2<'input>) -> RichType<'input> {
        RichType::new(NameHint::new_empty(), Type::Any) // TODO
    }

    fn process_complex_content(&mut self, model: &'ast complexContent_e<'input>) -> RichType<'input> {
        match model.child.complexContent_e_inner__extfield0 {
            complexContent_e_inner__extfield0::restriction(ref r) => self.process_restriction(r),
            complexContent_e_inner__extfield0::extension(ref e) => self.process_extension(e),
        }
    }

    fn process_restriction(&mut self, restriction: &'ast restriction_e<'input>) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in restriction.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "base") => base = Some(value),
                _ => panic!("Unknown attribute {} in <restriction>", key),
            }
        }
        let base = base.expect("<restriction> has no base");
        //match (restriction_e.child.0).0.restrictionType__extfield0.
        RichType::new(NameHint::new_empty(), Type::Alias(self.namespaces.parse_qname(base))) // TODO
    }

    fn process_type_def_particle(&mut self, particle: &'ast typeDefParticle<'input>, inlinable: bool) -> RichType<'input> {
        match particle {
            typeDefParticle::group(e) => self.process_group(e),
            typeDefParticle::all(_) => unimplemented!("all"),
            typeDefParticle::choice(e) => self.process_choice(e, inlinable),
            typeDefParticle::sequence(e) => self.process_sequence(e, inlinable),
        }
    }
    fn process_particle(&mut self, particle: &'ast particle<'input>, inlinable: bool) -> RichType<'input> {
        match particle {
            particle::element(e) => self.process_element(e),
            particle::group(e) => self.process_group(e),
            particle::all(_) => unimplemented!("all"),
            particle::choice(e) => self.process_choice(e, inlinable),
            particle::sequence(sequence) => self.process_sequence(sequence, inlinable),
            particle::any(e) => self.process_any(e),
        }
    }

    fn process_any(&mut self, any: &'ast any_e<'input>) -> RichType<'input> {
        RichType::new(NameHint::new_empty(), Type::Any)
    }

    fn process_sequence(&mut self, sequence: &'ast sequence_e<'input>, inlinable: bool) -> RichType<'input> {
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in sequence.attrs.iter() {
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
        let particles = &(sequence.child.0).0.particle.0;
        if min_occurs == 1 && max_occurs == 1 && inlinable && particles.len() == 1 {
            self.process_particle(particles.get(0).unwrap(), true)
        }
        else {
            for particle in particles.iter() {
                let ty = self.process_particle(particle, false);
                name_hint.extend(&ty.name_hint);
                items.push(ty);
            }
            if min_occurs == 1 && max_occurs == 1 {
                RichType::new(name_hint, Type::InlineSequence(items))
            }
            else {
                let name = self.namespaces.name_from_hint(&name_hint).unwrap();
                self.sequences.insert(name.clone(), items);
                RichType::new(name_hint, Type::Sequence(min_occurs, max_occurs, name))
            }
        }
    }

    fn process_choice(&mut self, choice: &'ast choice_e<'input>, inlinable: bool) -> RichType<'input> {
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in choice.attrs.iter() {
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
        for particle in (choice.child.0).0.particle.0.iter() {
            let ty = self.process_particle(particle, false);
            name_hint.extend(&ty.name_hint);
            items.push(ty);
        }
        match (min_occurs, max_occurs, inlinable) {
            (1, 1, true) => {
                RichType::new(name_hint, Type::InlineChoice(items))
            },
            (_, _, _) => {
                let name = self.namespaces.name_from_hint(&name_hint).unwrap();
                self.choices.insert(name.clone(), items);
                RichType::new(name_hint, Type::Choice(min_occurs, max_occurs, name))
            }
        }
    }

    fn process_extension(&mut self, extension: &'ast extension_e<'input>) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in extension.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "base") => base = Some(value),
                _ => panic!("Unknown attribute {} in <extension>", key),
            }
        }
        let base = base.expect("<extension> has no base");
        let base = self.namespaces.parse_qname(base);
        if let Some(ref particle) = extension.child.0.extensionType__extfield0.typeDefParticle.0 {
            RichType::new(NameHint::new_empty(), Type::Extension(base, Box::new(self.process_type_def_particle(particle, false))))
        }
        else {
            RichType::new(NameHint::new_empty(), Type::Alias(base))
        }
    }

    fn process_element(&mut self, element: &'ast element_e<'input>) -> RichType<'input> {
        let mut name = None;
        let mut ref_ = None;
        let mut type_attr = None;
        let mut abstract_ = false;
        let mut substitution_group = None;
        let mut min_occurs = 1;
        let mut max_occurs = 1;
        for (key, &value) in element.attrs.iter() {
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
            RichType::new(NameHint::new(field_name), Type::Element(min_occurs, max_occurs, ref_))
        }
        else {
            let name = name.expect("<element> has no name.");
            let e = &(element.child.0).0.element__extfield0;
            let element__extfield0 { element__extfield0__seqfield0: ref child_type, ref alternative, ref identityConstraint } = e;
            let type_ = match (type_attr, &child_type.0) {
                (None, Some(ref c)) => match c {
                    element__extfield0__seqfield0::simpleType(ref e) => self.process_simple_type(e),
                    element__extfield0__seqfield0::complexType(ref e) => self.process_complex_type(e),
                },
                (Some(t), None) => {
                    let (_, field_name) = t.as_tuple();
                    RichType::new(NameHint::new(field_name), Type::Alias(t))
                },
                (None, None) => RichType::new(NameHint::new_empty(), Type::Any), // TODO
                (Some(ref t1), Some(ref t2)) => panic!("Element '{:?}' has both a type attribute ({:?}) and a child type ({:?}).", name, t1, t2),
            };

            self.elements.insert(name, type_);
            let (_, field_name) = name.as_tuple();
            RichType::new(NameHint::new(field_name), Type::Element(min_occurs, max_occurs, name))
        }
    }

    fn gen_choices(&self, scope: &mut cg::Scope) {
        let module = scope.new_module("enums");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut choices: Vec<_> = self.choices.iter().collect();
        choices.sort_by_key(|&(n,_)| n);
        for (ref name, ref choice) in choices {
            self.gen_choice(module.scope(), name, choice);
        }
    }
    fn gen_choice(&self, scope: &mut cg::Scope, name: &str, items: &Vec<RichType<'input>>) {
        let mut impl_code = Vec::new();
        let enum_name = escape_keyword(&name.to_camel_case());
        let enum_name = self.renames.get(&enum_name).unwrap_or(&enum_name);
        impl_code.push(format!("impl_enum!({},", enum_name));
        {
            let mut enum_ = scope.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (i, item) in items.iter().enumerate() {
                let mut fields = Vec::new();
                {
                    let mut name_gen = NameGenerator::new();
                    let writer = &mut |variant_name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
                        let variant_name = escape_keyword(&name_gen.gen_name(variant_name.to_snake_case()));
                        let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                        let type_name = escape_keyword(&type_name.to_camel_case());
                        fields.push((variant_name, type_mod_name, min_occurs, max_occurs, type_name));
                    };
                    self.write_type_in_struct_def(writer, &item);
                }
                let variant_name = self.namespaces.name_from_hint(&item.name_hint)
                    .unwrap_or(format!("{}{}", name, i)).to_camel_case();
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
        let mut module = scope.new_module("sequences");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut sequences: Vec<_> = self.sequences.iter().collect();

        sequences.sort_by_key(|&(n,_)| n);
        for (name, sequence) in sequences {
            module.vis("pub");
            self.gen_group_or_sequence(module, name, &sequence.iter().collect());
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
                self.gen_choice(module.scope(), struct_name, items);
            }
            else if let Type::InlineSequence(ref items) = group.type_ {
                self.gen_group_or_sequence(module, struct_name, &items.iter().collect());
            }
            else {
                self.gen_group_or_sequence(module, struct_name, &vec![group]);
            }
        }
    }

    fn gen_group_or_sequence(&self, module: &mut cg::Module, struct_name: &'input str, items: &Vec<&RichType<'input>>) {
        let mut impl_code = Vec::new();
        let struct_name = escape_keyword(&struct_name.to_camel_case());
        let struct_name = self.renames.get(&struct_name).unwrap_or(&struct_name);
        impl_code.push(format!("impl_group_or_sequence!({},", struct_name));
        {
            let mut struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            let mut name_gen = NameGenerator::new();
            let writer = &mut |name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
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
            for item in items {
                self.write_type_in_struct_def(writer, item);
            }
        }
        impl_code.push(");".to_string());
        module.scope().raw(&impl_code.join("\n"));
    }

    fn gen_elements(&mut self, scope: &mut cg::Scope) {
        let mut elements: Vec<_> = self.elements.iter().collect();

        elements.sort_by_key(|&(n,_)| n);
        for (&name, element) in elements {
            let mod_name = self.namespaces.get_module_name(name);
            let mut module = scope.get_module_mut(mod_name).unwrap();
            self.gen_element(module, name, element);
        }
    }

    fn gen_element(&self, module: &mut cg::Module, name: FullName<'input>, type_: &RichType<'input>) {
        let mut impl_code = Vec::new();
        let (mod_name, tag_name) = name.as_tuple();
        let struct_name = escape_keyword(&tag_name.to_camel_case());
        let mut can_be_empty = true;
        impl_code.push(format!("impl_element!({}, \"{}\", {{", struct_name, tag_name));
        {
            let mut struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            struct_.field("pub attrs", "HashMap<QName<'input>, &'input str>");
            let mut name_gen = NameGenerator::new();
            let writer = &mut |name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
                let name = escape_keyword(&name_gen.gen_name(name.to_snake_case()));
                let name = self.renames.get(&name).unwrap_or(&name);
                let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                let type_name = escape_keyword(&type_name.to_camel_case());
                let type_name = self.renames.get(&type_name).unwrap_or(&type_name);
                match (min_occurs, max_occurs) {
                    (1, 1) => {
                        can_be_empty = false; // TODO: what if all subelements can be empty?
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
                        can_be_empty = false; // TODO: what if all subelements can be empty?
                        struct_.field(&format!("pub {}", name), &format!("Vec<super::{}::{}<'input>>", type_mod_name, type_name));
                        impl_code.push(format!("    ({}, {}, Vec<{}>),", name, type_mod_name, type_name))
                    },
                }
            };
            self.write_type_in_struct_def(writer, type_);
        }
        impl_code.push(format!("}}, can_be_empty={:?});", can_be_empty));
        module.scope().raw(&impl_code.join("\n"));
    }

    fn write_type_in_struct_def<'a, F>(&'a self, mut writer: &mut F, rich_type: &'a RichType<'input>)
            where F: FnMut(&'a str, &'a str, usize, usize, &'a str), 'ast: 'a {
        let RichType { name_hint, type_ } = rich_type;
        match &type_ {
            Type::Alias(name) => {
                let target_type = self.types.get(name).unwrap();
                self.write_type_in_struct_def(writer, target_type)
            },
            Type::InlineSequence(items) => {
                for item in items {
                    self.write_type_in_struct_def(writer, item)
                }
            }
            Type::Sequence(1, 1, name) => { // shortcut, and a lot more readable
                let items = self.sequences.get(name).unwrap();
                for item in items {
                    self.write_type_in_struct_def(writer, item)
                }
            }
            Type::Sequence(min_occurs, max_occurs, name) => {
                writer(name, "sequences", *min_occurs, *max_occurs, name);
            }
            Type::Group(min_occurs, max_occurs, name) |
            Type::Element(min_occurs, max_occurs, name) => {
                let (mod_name, type_name) = name.as_tuple();
                let field_name = type_name;
                let mod_name = self.namespaces.module_names.get(mod_name).expect(mod_name);
                writer(field_name, mod_name, *min_occurs, *max_occurs, type_name);
            },
            Type::Choice(min_occurs, max_occurs, ref name) => {
                writer(name, "enums", *min_occurs, *max_occurs, name);
            },
            Type::Extension(base, ext_type) => {
                let base_type = &self.types.get(base).unwrap();
                self.write_type_in_struct_def(writer, base_type);
                self.write_type_in_struct_def(writer, ext_type);
            },
            Type::Any => {
                writer("any", "support", 0, 0, "Any")
            },
            _ => unimplemented!("writing {:?}", type_),
        }
    }
}
