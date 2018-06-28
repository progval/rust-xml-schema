use std::collections::HashMap;

use codegen as cg;

use support::*;
use generated::UNQUAL::*;

const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

type FullName<'input> = (&'input str, &'input str);

struct Namespaces<'input> {
    target_namespace: &'input str,
    namespaces: HashMap<&'input str, &'input str>,
    default_namespace: &'input str,
    elements: HashMap<FullName<'input>, String>,
    types: HashMap<FullName<'input>, String>,
    type_aliases: HashMap<FullName<'input>, FullName<'input>>,
}

impl<'input> Namespaces<'input> {
    fn new(mut namespaces: HashMap<&'input str, &'input str>, target_namespace: &'input str) -> Namespaces<'input> {
        if let Some(uri) = namespaces.insert("xml", "xml") {
            panic!("Cannot have a namespaces named \"xml\": {}", uri);
        }
        if let Some(uri) = namespaces.insert("xmlns", "xmlns") {
            panic!("Cannot have a namespaces named \"xmlns\": {}", uri);
        }
        Namespaces {
            target_namespace,
            namespaces,
            default_namespace: target_namespace,
            elements: HashMap::new(),
            types: HashMap::new(),
            type_aliases: HashMap::new()
        }
    }

    fn expand_prefix(&self, prefix: Option<&'input str>) -> &'input str {
        match prefix {
            Some(prefix) => self.namespaces.get(prefix).expect(&format!("Unknown prefix: {:?}", prefix)),
            None => self.default_namespace,
        }
    }
    fn expand_qname(&self, qname: QName<'input>) -> (&'input str, &'input str) {
        (self.expand_prefix(qname.0), qname.1)
    }
    fn qname_eq(&self, qname1: QName<'input>, qname2: QName<'input>) -> bool {
        qname1.1 == qname2.1 && self.expand_prefix(qname1.0) == self.expand_prefix(qname2.0)
    }
    fn name_for_element(&mut self, element_name: QName<'input>) -> &str {
        let prefix = self.expand_prefix(element_name.0);
        let local = element_name.1;
        // TODO: check the local name is not already used
        self.elements.insert((prefix, local), local.to_string());
        self.elements.get(&(prefix, local)).as_ref().unwrap()
    }
    fn name_for_type(&mut self, type_name: QName<'input>) -> &str {
        let mut type_name = self.expand_qname(type_name);
        while let Some(new_type_name) = self.type_aliases.get(&type_name) {
            type_name = *new_type_name
        }
        // TODO: check the local name is not already used
        self.types.insert(type_name.clone(), type_name.1.to_string());
        self.types.get(&type_name).as_ref().unwrap()
    }
    fn type_alias(&mut self, from_type: QName<'input>, to_type: QName<'input>) {
        let from_type = self.expand_qname(from_type).clone();
        let to_type = self.expand_qname(to_type).clone();
        self.type_aliases.insert(from_type, to_type);
    }
}

enum Type<'input> {
    Any,
    Named(FullName<'input>),
}

pub struct ParserGenerator<'ast, 'input: 'ast> {
    namespaces: Namespaces<'input>,
    element_form_default_qualified: bool,
    attribute_form_default_qualified: bool,
    elements: HashMap<FullName<'input>, Type<'input>>,
    groups: HashMap<FullName<'input>, &'ast group_e<'input>>,
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
            groups: HashMap::new(),
            attribute_groups: HashMap::new(),
        }
    }

    pub fn gen(&mut self, ast: &'ast schema_e<'input>) -> cg::Scope {
        self.process_ast(ast);
        self.gen_target_scope(ast)
    }

    fn gen_target_scope(&mut self, ast: &schema_e<'input>) -> cg::Scope {
        let mut scope = cg::Scope::new();
        let mut names: Vec<_> = self.elements.keys().cloned().collect();
        names.sort();
        for name in names {
            self.gen_element(&mut scope, name);
        }
        scope
    }

    fn process_ast(&mut self, ast: &'ast schema_e<'input>) {
        for top_level_item in ast.child.schema_e_inner__extfield0.schema_e_inner__extfield0__seqfield2.0.iter() {
            match top_level_item.schemaTop {
                schemaTop::redefinable(ref r) => self.process_top_level_redefinable(r),
                schemaTop::element(ref e) => self.process_top_level_element(e),
                schemaTop::attribute(_) => unimplemented!("top-level attribute"),
                schemaTop::notation(ref e) => self.process_top_level_notation(e),
            }
        }
    }

    fn process_top_level_notation(&mut self, notation: &'ast notation_e<'input>) {
        // TODO
    }

    fn process_top_level_redefinable(&mut self, r: &'ast redefinable<'input>) {
        match r {
            redefinable::simpleType(e) => self.process_top_level_simple_type(e),
            redefinable::complexType(e) => self.process_top_level_complex_type(e),
            redefinable::group(e) => self.process_top_level_group(e),
            redefinable::attributeGroup(e) => self.process_top_level_attribute_group(e),
        }
    }

    fn process_top_level_group(&mut self, group: &'ast group_e<'input>) {
        let mut name = None;
        for (key, &value) in group.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }
        let name = name.expect("<group> has no name.");
        self.groups.insert(self.namespaces.expand_qname(QName::from(name)), group);
    }

    fn process_top_level_attribute_group(&mut self, group: &'ast attributeGroup_e<'input>) {
        let mut name = None;
        for (key, &value) in group.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                _ => panic!("Unknown attribute {} in <attributeGroup>", key),
            }
        }
        let name = name.expect("<attributeGroup> has no name.");
        self.attribute_groups.insert(self.namespaces.expand_qname(QName::from(name)), group);
    }

    fn process_top_level_simple_type(&mut self, type_: &'ast simpleType_e<'input>) {
        let mut name = None;
        for (key, &value) in type_.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                _ => panic!("Unknown attribute {} in <simpleType>", key),
            }
        }
        let name = name.expect("<simpleType> has no name.");
        //let struct_name = self.namespaces.new_type(QName::from(name));
        match (type_.child.0).0.simpleDerivation {
            simpleDerivation::restriction(ref e) => self.process_top_level_restriction(name, e),
            simpleDerivation::list(ref e) => self.process_top_level_list(name, e),
            simpleDerivation::union(ref e) => self.process_top_level_union(name, e),
        }
    }

    fn process_simple_type(&mut self, type_: &'ast simpleType_e<'input>) -> Type<'input> {
        let mut name = None;
        for (key, &value) in type_.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                _ => panic!("Unknown attribute {} in <simpleType>", key),
            }
        }
        // TODO
        Type::Any
    }

    fn process_top_level_list(&mut self, name: &'input str, list: &'ast list_e<'input>) {
        // TODO
    }

    fn process_top_level_union(&mut self, name: &'input str, union: &'ast union_e<'input>) {
        // TODO
    }

    fn process_top_level_complex_type(&mut self, type_: &'ast complexType_e<'input>) {
        let mut name = None;
        let mut abstract_ = false;
        let mut mixed = false;
        for (key, &value) in type_.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                (SCHEMA_URI, "abstract") => match value {
                    "true" => abstract_ = true,
                    "false" => abstract_ = false,
                    _ => panic!("Invalid value for abstract attribute: {}", value),
                },
                (SCHEMA_URI, "mixed") => match value {
                    "true" => mixed = true,
                    "false" => mixed = false,
                    _ => panic!("Invalid value for mixed attribute: {}", value),
                },
                _ => panic!("Unknown attribute {} in <complexType>", key),
            }
        }
        let name = name.expect("<complexType> has no name.");
        //let struct_name = self.namespaces.new_type(QName::from(name));
        match (type_.child.0).0.complexTypeModel {
            complexTypeModel::simpleContent(_) => unimplemented!("simpleContent"),
            complexTypeModel::complexContent(ref model) => self.process_top_level_complex_content(name, model),
            complexTypeModel::complexTypeModel__choicevariant2(ref model) => self.process_other_complex_type_model(name, model),
        }
    }

    fn process_complex_type(&mut self, type_: &'ast complexType_e<'input>) -> Type<'input> {
        let mut name = None;
        let mut abstract_ = false;
        let mut mixed = false;
        for (key, &value) in type_.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                (SCHEMA_URI, "mixed") => match value {
                    "true" => mixed = true,
                    "false" => mixed = false,
                    _ => panic!("Invalid value for mixed attribute: {}", value),
                },
                _ => panic!("Unknown attribute {} in <complexType>", key),
            }
        }
        // TODO
        Type::Any
    }

    fn process_other_complex_type_model(&mut self, name: &'input str, model: &'ast complexTypeModel__choicevariant2<'input>) {
        // TODO
    }

    fn process_top_level_complex_content(&mut self, name: &'input str, model: &'ast complexContent_e<'input>) {
        match model.child.complexContent_e_inner__extfield0 {
            complexContent_e_inner__extfield0::restriction(ref r) => self.process_top_level_restriction(name, r),
            complexContent_e_inner__extfield0::extension(ref e) => self.process_top_level_extension(name, e),
        }
    }

    fn process_top_level_restriction(&mut self, name: &'input str, restriction: &'ast restriction_e<'input>) {
        let mut base = None;
        for (key, &value) in restriction.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "base") => base = Some(value),
                _ => panic!("Unknown attribute {} in <restriction>", key),
            }
        }
        let base = base.expect("<restriction> has no base");
        //match (restriction_e.child.0).0.restrictionType__extfield0.
        self.namespaces.type_alias(QName::from(name), QName::from(base))
    }

    fn process_top_level_extension(&mut self, name: &'input str, extension: &'ast extension_e<'input>) {
        let mut base = None;
        for (key, &value) in extension.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "base") => base = Some(value),
                _ => panic!("Unknown attribute {} in <extension>", key),
            }
        }
        let base = base.expect("<extension> has no base");
        // TODO
    }

    fn process_top_level_element(&mut self, element: &'ast element_e<'input>) {
        let mut name = None;
        let mut type_attr = None;
        let mut abstract_ = false;
        let mut substitution_group = None;
        for (key, &value) in element.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(self.namespaces.expand_qname(QName::from(value))),
                (SCHEMA_URI, "id") => (),
                (SCHEMA_URI, "type") => type_attr = Some(self.namespaces.expand_qname(QName::from(value))),
                (SCHEMA_URI, "abstract") => match value {
                    "true" => abstract_ = true,
                    "false" => abstract_ = false,
                    _ => panic!("Invalid value for abstract attribute: {}", value),
                },
                (SCHEMA_URI, "substitutionGroup") => substitution_group = Some(self.namespaces.expand_qname(QName::from(value))),
                _ => panic!("Unknown attribute {} in <element>", key),
            }
        }
        let name = name.expect("<element> has no name.");
        let e = &(element.child.0).0.element__extfield0;
        let element__extfield0 { element__extfield0__seqfield0: ref child_type, ref alternative, ref identityConstraint } = e;
        let type_ = match (type_attr, &child_type.0) {
            (None, Some(ref c)) => match c {
                element__extfield0__seqfield0::simpleType(ref e) => self.process_simple_type(e),
                element__extfield0__seqfield0::complexType(ref e) => self.process_complex_type(e),
            },
            (Some(t), None) => Type::Named(t),
            (None, None) => Type::Any, // TODO
            (Some(t1), Some(t2)) => panic!("Element '{}' has both a type attribute ({:?}) and a child type ({:?}).", name.1, t1, t2),
        };
        self.elements.insert(name, type_);
    }

    fn gen_element(&mut self, scope: &mut cg::Scope, name: FullName<'input>) {
        let type_ = self.elements.get(&name).unwrap();
    }
}
