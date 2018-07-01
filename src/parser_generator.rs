use std::collections::HashMap;
use std::num::ParseIntError;

use codegen as cg;

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

pub const MACROS: &'static str = r#"
macro_rules! try_rollback {
    ($stream:expr, $tx:expr, $e:expr) => {
        match $e {
            Some(i) => i,
            None => {
                $tx.rollback($stream);
                return None
            }
        }
    }
}

macro_rules! impl_enum {
    ( $name:ident, $($variant_macro:ident ! ( $($variant_args: tt )* ),  )* ) => {
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = "enum $name";
            fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
                let tx = stream.transaction();
                $(
                    match $variant_macro!($name, stream, parse_context, parent_context, $($variant_args)*) {
                        Some(x) => return Some(x),
                        None => (), // TODO: should we rollback here?
                    }
                )*

                tx.rollback(stream);
                None
            }
        }
    }
}

macro_rules! impl_singleton_variant {
    ( $enum_name:ident, $stream:expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, $type_mod_name:ident, $type_name:ident ) => {
        super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context).map(Box::new).map($enum_name::$variant_name)
    }
}

macro_rules! impl_struct_variant {
    ( $enum_name:ident, $stream: expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, $( ( $field_name:ident, $type_mod_name:ident, $type_name:ident ), )* ) => {{
        let mut res = None;
        loop { // single run, used for breaking
            $(
                let $field_name = match super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context) {
                    Some(e) => Box::new(e),
                    None => break,
                };
            )*
            res = Some($enum_name::$variant_name {
                $(
                    $field_name,
                )*
            });
            break;
        }
        res
    }}
}

macro_rules! impl_element {
    ( $name:ident, $( ( $field_name:ident, $type_mod_name:ident, $type_name:ident ),  )* ) => {
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = "element $name";
            fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
                let tx = stream.transaction();
                let mut tok = stream.next().unwrap();
                loop {
                    match tok {
                        Token::Whitespaces(_) => (),
                        Token::Comment(_) => (),
                        Token::Text(_) => (),
                        _ => break,
                    }
                    tok = stream.next().unwrap();
                }
                match tok {
                    Token::ElementStart(prefix, name) => {
                        if name.to_str() == "$name" {
                            let mut attrs = HashMap::new();
                            loop {
                                let tok = stream.next().unwrap();
                                match tok {
                                    Token::Whitespaces(_) => (),
                                    Token::Comment(_) => (),
                                    Token::Text(_) => (),
                                    Token::Attribute((key_prefix, key_local), value) => {
                                        let key = QName(match key_prefix.to_str() { "" => None, s => Some(s) }, key_local.to_str());
                                        let old = attrs.insert(key, value.to_str()); assert_eq!(old, None)
                                    },
                                    Token::ElementEnd(ElementEnd::Open) => {
                                        let ret = Some($name {
                                            ATTRS: attrs,
                                            $(
                                                $field_name: try_rollback!(stream, tx, super::$type_mod_name::$type_name::parse_xml(stream, parse_context, parent_context)),
                                            )*
                                        });
                                        let mut next_tok;
                                        loop {
                                            next_tok = stream.next();
                                            match next_tok {
                                                Some(Token::Whitespaces(_)) => (),
                                                Some(Token::Comment(_)) => (),
                                                Some(Token::Text(_)) => (),
                                                Some(Token::ElementEnd(ElementEnd::Close(prefix2, name2))) => {
                                                    assert_eq!((prefix.to_str(), name.to_str()), (prefix2.to_str(), name2.to_str()));
                                                    return ret;
                                                }
                                                _ => panic!(format!("Did not expect token {:?}", next_tok)),
                                            }
                                        }
                                    },
                                    Token::ElementEnd(ElementEnd::Empty) => {
                                        panic!("Empty element $name"); // TODO
                                    },
                                    Token::ElementEnd(ElementEnd::Close(_, _)) => {
                                        tx.rollback(stream);
                                        return None
                                    },
                                    _ => panic!(format!("Did not expect token {:?}", tok)),
                                }
                            }
                        }
                        else {
                            tx.rollback(stream);
                            None
                        }
                    },
                    Token::ElementEnd(ElementEnd::Close(_, _)) => {
                        tx.rollback(stream);
                        return None
                    },
                    _ => panic!(format!("Did not expect token {:?}", tok)),
                }
            }
        }
    }
}
"#;

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
    min_occurs: Option<usize>,
    max_occurs: Option<usize>,
    type_: Type<'input>,
}
impl<'input> RichType<'input> {
    fn new(name_hint: NameHint<'input>, min_occurs: Option<usize>, max_occurs: Option<usize>, type_: Type<'input>) -> RichType<'input> {
        RichType { name_hint, min_occurs, max_occurs, type_ }
    }
}

#[derive(Debug)]
enum Type<'input> {
    Any,
    Alias(FullName<'input>),
    List(Box<RichType<'input>>),
    Union(Vec<RichType<'input>>),
    Extension(FullName<'input>, Box<RichType<'input>>),
    Sequence(Vec<RichType<'input>>),
    Element(FullName<'input>),
    Group(FullName<'input>),
    Choice(String),
}

#[derive(Debug)]
pub struct ParserGenerator<'ast, 'input: 'ast> {
    namespaces: Namespaces<'input>,
    element_form_default_qualified: bool,
    attribute_form_default_qualified: bool,
    elements: HashMap<FullName<'input>, RichType<'input>>,
    types: HashMap<FullName<'input>, RichType<'input>>,
    choices: HashMap<String, Vec<RichType<'input>>>,
    groups: HashMap<FullName<'input>, Vec<RichType<'input>>>,
    attribute_groups: HashMap<FullName<'input>, &'ast attributeGroup_e<'input>>,
}

impl<'ast, 'input: 'ast> ParserGenerator<'ast, 'input> {
    pub fn new(ast: &'ast schema_e<'input>) -> ParserGenerator<'ast, 'input> {
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
            attribute_groups: HashMap::new(),
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
        self.gen_choices(&mut scope);
        self.gen_elements(&mut scope);
        scope
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
        let mut max_occurs = None;
        let mut min_occurs = None;
        for (key, &value) in group.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "ref") =>
                    ref_ = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = Some(value.parse().unwrap()),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = Some(parse_max_occurs(value).unwrap()),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        if let Some(ref_) = ref_ {
            if let Some(name) = name {
                panic!("<group> has both ref={:?} and name={:?}", ref_, name)
            }
            let (_, field_name) = ref_.as_tuple();
            RichType::new(NameHint::new(field_name), min_occurs, max_occurs, Type::Group(ref_))
        }
        else {
            let name = name.expect("<group> has no name or ref.");

            let mut items = Vec::new();
            for particle in ((group.child.0).0).0.particle.0.iter() {
                items.push(self.process_particle(particle));
            }

            self.groups.insert(name, items);
            RichType::new(NameHint::from_fullname(&name), min_occurs, max_occurs, Type::Group(name))
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
            RichType::new(NameHint::from_fullname(&name), None, None, Type::Alias(name))
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
            (Some(n), None) => RichType::new(NameHint::new_empty(), None, None, Type::Alias(n)),
            (None, None) => RichType::new(NameHint::new_empty(), None, None, Type::Any), // TODO
            (Some(ref t1), Some(ref t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        let mut name_hint = item_type.name_hint.clone();
        name_hint.push("list");
        RichType::new(name_hint, None, None, Type::List(Box::new(item_type)))
    }

    fn process_union(&mut self, union: &'ast union_e<'input>) -> RichType<'input> {
        let mut member_types = Vec::new();
        for (key, &value) in union.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "memberTypes") => {
                    member_types = value.split(" ").map(|s| {
                        let name = self.namespaces.parse_qname(s);
                        let (_, field_name) = name.as_tuple();
                        RichType::new(NameHint::new(field_name), None, None, Type::Alias(name))
                    }).collect()
                },
                _ => panic!("Unknown attribute {} in <union>", key),
            }
        }

        for t in union.child.simpleType.0.iter() {
            member_types.push(self.process_simple_type(t))
        }

        RichType::new(NameHint::new_empty(), None, None, Type::Union(member_types))
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
            RichType::new(NameHint::from_fullname(&name), None, None, Type::Alias(name))
        }
        else {
           ty 
        }
    }

    fn process_other_complex_type_model(&mut self, model: &'ast complexTypeModel__choicevariant2<'input>) -> RichType<'input> {
        RichType::new(NameHint::new_empty(), None, None, Type::Any) // TODO
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
        RichType::new(NameHint::new_empty(), None, None, Type::Alias(self.namespaces.parse_qname(base))) // TODO
    }

    fn process_type_def_particle(&mut self, particle: &'ast typeDefParticle<'input>) -> RichType<'input> {
        match particle {
            typeDefParticle::group(e) => self.process_group(e),
            typeDefParticle::all(_) => unimplemented!("all"),
            typeDefParticle::choice(e) => self.process_choice(e),
            typeDefParticle::sequence(e) => self.process_sequence(e),
        }
    }
    fn process_particle(&mut self, particle: &'ast particle<'input>) -> RichType<'input> {
        match particle {
            particle::element(e) => self.process_element(e),
            particle::group(e) => self.process_group(e),
            particle::all(_) => unimplemented!("all"),
            particle::choice(e) => self.process_choice(e),
            particle::sequence(sequence) => self.process_sequence(sequence),
            particle::any(e) => self.process_any(e),
        }
    }

    fn process_any(&mut self, any: &'ast any_e<'input>) -> RichType<'input> {
        RichType::new(NameHint::new_empty(), None, None, Type::Any)
    }

    fn process_sequence(&mut self, sequence: &'ast sequence_e<'input>) -> RichType<'input> {
        let mut items = Vec::new();
        for particle in (sequence.child.0).0.particle.0.iter() {
            items.push(self.process_particle(particle));
        }
        RichType::new(NameHint::new_empty(), None, None, Type::Sequence(items))
    }

    fn process_choice(&mut self, choice: &'ast choice_e<'input>) -> RichType<'input> {
        let mut items = Vec::new();
        let mut name_hint = NameHint::new("choice");
        for particle in (choice.child.0).0.particle.0.iter() {
            let ty = self.process_particle(particle);
            name_hint.extend(&ty.name_hint);
            items.push(ty);
        }
        let name = self.namespaces.name_from_hint(&name_hint).unwrap();
        self.choices.insert(name.clone(), items);
        RichType::new(name_hint, None, None, Type::Choice(name))
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
            RichType::new(NameHint::new_empty(), None, None, Type::Extension(base, Box::new(self.process_type_def_particle(particle))))
        }
        else {
            RichType::new(NameHint::new_empty(), None, None, Type::Alias(base))
        }
    }

    fn process_element(&mut self, element: &'ast element_e<'input>) -> RichType<'input> {
        let mut name = None;
        let mut ref_ = None;
        let mut type_attr = None;
        let mut abstract_ = false;
        let mut substitution_group = None;
        let mut min_occurs = None;
        let mut max_occurs = None;
        for (key, &value) in element.attrs.iter() {
            match self.namespaces.expand_qname(*key).as_tuple() {
                (SCHEMA_URI, "name") =>
                    name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "id") =>
                    (),
                (SCHEMA_URI, "type") =>
                    type_attr = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") =>
                    min_occurs = Some(value.parse().unwrap()),
                (SCHEMA_URI, "maxOccurs") =>
                    max_occurs = Some(parse_max_occurs(value).unwrap()),
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
            RichType::new(NameHint::new(field_name), min_occurs, max_occurs, Type::Element(ref_))
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
                    RichType::new(NameHint::new(field_name), None, None, Type::Alias(t))
                },
                (None, None) => RichType::new(NameHint::new_empty(), None, None, Type::Any), // TODO
                (Some(ref t1), Some(ref t2)) => panic!("Element '{:?}' has both a type attribute ({:?}) and a child type ({:?}).", name, t1, t2),
            };

            self.elements.insert(name, type_);
            let (_, field_name) = name.as_tuple();
            RichType::new(NameHint::new(field_name), min_occurs, max_occurs, Type::Element(name))
        }
    }

    fn gen_choices(&mut self, scope: &mut cg::Scope) {
        let mut module = scope.new_module("ENUMS");
        module.scope().raw("use super::*;");
        // TODO: sort the choices
        for (name, items) in self.choices.iter() {
            let mut impl_code = Vec::new();
            impl_code.push(format!("impl_enum!({},", name));
            {
                let mut enum_ = module.new_enum(&escape_keyword(&name)).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
                for (i, item) in items.iter().enumerate() {
                    let mut fields = Vec::new();
                    {
                        let writer = &mut |variant_name, type_mod_name, type_name| {
                            fields.push((escape_keyword(variant_name), escape_keyword(type_mod_name), escape_keyword(type_name)));
                        };
                        self.write_type_in_struct_def(writer, &item);
                    }
                    let variant_name = self.namespaces.name_from_hint(&item.name_hint)
                        .unwrap_or(format!("{}{}", name, i));
                    let mut variant = enum_.new_variant(&variant_name);
                    if fields.len() == 1 {
                        let (_, type_mod_name, type_name) = fields.remove(0);
                        variant.tuple(&format!("Box<super::{}::{}<'input>>", escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                        impl_code.push(format!("    impl_singleton_variant!({}, {}, {}),", escape_keyword(&variant_name), escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                    }
                    else {
                        impl_code.push(format!("    impl_struct_variant!({},", variant_name));
                        for (field_name, type_mod_name, type_name) in fields {
                            impl_code.push(format!("        ({}, {}, {}),", field_name, type_mod_name, type_name));
                            variant.named(&field_name, &format!("Box<super::{}::{}<'input>>", type_mod_name, type_name));
                        }
                        impl_code.push(format!("    ),"));
                    }
                }
            }
            impl_code.push(");".to_string());
            module.scope().raw(&impl_code.join("\n"));
        }
    }

    fn gen_elements(&mut self, scope: &mut cg::Scope) {
        let mut elements: Vec<_> = self.elements.iter().collect();

        elements.sort_by_key(|&(n,_)| n);
        for (&name, element) in elements {
            let module = &mut scope.get_or_new_module(self.namespaces.get_module_name(name));
            module.scope().raw("use super::*;");
            self.gen_element(module, name, element);
        }
    }

    fn gen_element(&self, module: &mut cg::Module, name: FullName<'input>, type_: &RichType<'input>) {
        let mut impl_code = Vec::new();
        let (mod_name, struct_name) = name.as_tuple();
        let struct_name = escape_keyword(struct_name);
        impl_code.push(format!("impl_element!({},", struct_name));
        {
            let mut struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            struct_.field("ATTRS", "HashMap<QName<'input>, &'input str>");
            let mut nb_uses = HashMap::<&str, usize>::new(); // TODO: use a proper implementation for handling duplicate names.
            let writer = &mut |name, type_mod_name, type_name| {
                let nb_use = nb_uses.get(name).cloned().unwrap_or(0);
                nb_uses.insert(&name, nb_use+1);
                let name = format!("{}{}", name, "_".repeat(nb_use));
                struct_.field(&name, &format!("super::{}::{}<'input>", escape_keyword(type_mod_name), escape_keyword(type_name)));
                impl_code.push(format!("    ({}, {}, {}),", escape_keyword(&name), escape_keyword(type_mod_name), escape_keyword(type_name)))
            };
            self.write_type_in_struct_def(writer, type_);
        }
        impl_code.push(");".to_string());
        module.scope().raw(&impl_code.join("\n"));
    }

    fn write_type_in_struct_def<'a, F>(&'a self, mut writer: &mut F, rich_type: &'a RichType<'input>)
            where F: FnMut(&'a str, &'a str, &'a str), 'ast: 'a {
        let RichType { name_hint, type_, min_occurs, max_occurs } = rich_type;
        // TODO: handle min_occurs and max_occurs
        match &type_ {
            Type::Alias(name) => {
                let target_type = self.types.get(name).unwrap();
                self.write_type_in_struct_def(writer, target_type)
            },
            Type::Group(name) => {
                let group = self.groups.get(name).unwrap();
                for field in group.iter() {
                    self.write_type_in_struct_def(writer, field);
                }
            },
            Type::Sequence(fields) => {
                for field in fields.iter() {
                    self.write_type_in_struct_def(writer, field);
                }
            },
            Type::Element(name) => {
                let (mod_name, type_name) = name.as_tuple();
                let field_name = type_name;
                let mod_name = self.namespaces.module_names.get(mod_name).expect(mod_name);
                writer(field_name, mod_name, type_name);
            },
            Type::Choice(ref name) => {
                writer(name, "ENUMS", name);
            },
            Type::Extension(base, ext_type) => {
                let base_type = &self.types.get(base).unwrap();
                self.write_type_in_struct_def(writer, base_type);
                self.write_type_in_struct_def(writer, ext_type);
            },
            Type::Any => (),
            _ => unimplemented!("writing {:?}", type_),
        }
    }
}
