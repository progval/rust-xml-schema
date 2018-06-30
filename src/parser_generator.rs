use std::collections::HashMap;
use std::num::ParseIntError;

use codegen as cg;

use support::*;
use generated::UNQUAL::*;

const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

type FullName<'input> = (&'input str, &'input str);

fn parse_max_occurs(s: &str) -> Result<usize, ParseIntError> {
    if s == "unbounded" {
        Ok(usize::max_value())
    }
    else {
        s.parse()
    }
}

#[derive(Debug)]
struct Namespaces<'input> {
    target_namespace: &'input str,
    namespaces: HashMap<&'input str, &'input str>, // namespace -> URI
    module_names: HashMap<&'input str, &'input str>, // URI -> module name
    default_namespace: &'input str,
}

impl<'input> Namespaces<'input> {
    fn new(mut namespaces: HashMap<&'input str, &'input str>, target_namespace: &'input str) -> Namespaces<'input> {
        if let Some(uri) = namespaces.insert("xml", "xml") {
            panic!("Cannot have a namespaces named \"xml\": {}", uri);
        }
        if let Some(uri) = namespaces.insert("xmlns", "xmlns") {
            panic!("Cannot have a namespaces named \"xmlns\": {}", uri);
        }
        let mut module_names = HashMap::new();
        for (ns, uri) in namespaces.iter() {
            module_names.insert(*uri, *ns);
        }
        Namespaces {
            target_namespace,
            namespaces,
            default_namespace: target_namespace,
            module_names,
        }
    }

    fn expand_prefix(&self, prefix: Option<&'input str>) -> &'input str {
        match prefix {
            Some(prefix) => self.namespaces.get(prefix).expect(&format!("Unknown prefix: {:?}", prefix)),
            None => self.default_namespace,
        }
    }
    fn expand_qname(&self, qname: QName<'input>) -> FullName<'input> {
        (self.expand_prefix(qname.0), qname.1)
    }
    fn parse_qname(&self, s: &'input str) -> FullName<'input> {
        self.expand_qname(QName::from(s))
    }
    fn qname_eq(&self, qname1: QName<'input>, qname2: QName<'input>) -> bool {
        qname1.1 == qname2.1 && self.expand_prefix(qname1.0) == self.expand_prefix(qname2.0)
    }

    fn get_module_name(&self, qname: FullName<'input>) -> &'input str {
        self.module_names.get(qname.0).cloned().unwrap_or("UNQUAL")
    }

    fn name_from_hint(&self, hint: &NameHint<'input>) -> Option<String> {
        if hint.tokens.len() > 0 {
            Some(hint.tokens.join("_"))
        }
        else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct NameHint<'input> {
    tokens: Vec<&'input str>,
}
impl<'input> NameHint<'input> {
    fn new_empty() -> NameHint<'input> {
        NameHint { tokens: Vec::new() }
    }
    fn new(s: &'input str) -> NameHint<'input> {
        NameHint { tokens: vec![s] }
    }
    fn push(mut self, s: &'input str) -> NameHint<'input> {
        self.tokens.push(s);
        self
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
    choices: Vec<(String, Vec<RichType<'input>>)>,
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
            choices: Vec::new(),
            attribute_groups: HashMap::new(),
        }
    }

    pub fn gen(&mut self, ast: &'ast schema_e<'input>) -> cg::Scope {
        self.process_ast(ast);
        self.gen_target_scope(ast)
    }

    fn gen_target_scope(&mut self, ast: &schema_e<'input>) -> cg::Scope {
        let mut scope = cg::Scope::new();
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
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "ref") => ref_ = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") => min_occurs = Some(value.parse().unwrap()),
                (SCHEMA_URI, "maxOccurs") => max_occurs = Some(parse_max_occurs(value).unwrap()),
                _ => panic!("Unknown attribute {} in <group>", key),
            }
        }

        if let Some(ref_) = ref_ {
            if let Some(name) = name {
                panic!("<group> has both ref={} and name={}", ref_.1, name.1)
            }
            RichType::new(NameHint::new(ref_.1), min_occurs, max_occurs, Type::Group(ref_))
        }
        else {
            let name = name.expect("<group> has no name or ref.");

            let mut items = Vec::new();
            for particle in ((group.child.0).0).0.particle.0.iter() {
                items.push(self.process_particle(particle));
            }

            self.groups.insert(name, items);
            RichType::new(NameHint::new(name.1), min_occurs, max_occurs, Type::Group(name))
        }
    }

    fn process_attribute_group(&mut self, group: &'ast attributeGroup_e<'input>) {
        let mut name = None;
        for (key, &value) in group.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(value),
                _ => panic!("Unknown attribute {} in <attributeGroup>", key),
            }
        }
        let name = name.expect("<attributeGroup> has no name.");
        self.attribute_groups.insert(self.namespaces.parse_qname(name), group);
    }

    fn process_simple_type(&mut self, type_: &'ast simpleType_e<'input>) -> RichType<'input> {
        let mut name = None;
        for (key, &value) in type_.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(self.namespaces.parse_qname(value)),
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
            RichType::new(NameHint::new(name.1), None, None, Type::Alias(name))
        }
        else {
            ty
        }
    }

    fn process_list(&mut self, list: &'ast list_e<'input>) -> RichType<'input> {
        let mut item_type = None;
        for (key, &value) in list.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "itemType") => item_type = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <list>", key),
            }
        }
        
        let item_type = match (item_type, &list.child.simpleType.0) {
            (None, Some(st)) => self.process_simple_type(st),
            (Some(n), None) => RichType::new(NameHint::new_empty(), None, None, Type::Alias(n)),
            (None, None) => RichType::new(NameHint::new_empty(), None, None, Type::Any), // TODO
            (Some(t1), Some(t2)) => panic!("<list> has both an itemType attribute ({:?}) and a child type ({:?}).", t1, t2),
        };

        RichType::new(item_type.name_hint.clone().push("list"), None, None, Type::List(Box::new(item_type)))
    }

    fn process_union(&mut self, union: &'ast union_e<'input>) -> RichType<'input> {
        let mut member_types = Vec::new();
        for (key, &value) in union.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "memberTypes") => {
                    member_types = value.split(" ").map(|s| {
                        let name = self.namespaces.parse_qname(s);
                        RichType::new(NameHint::new(name.1), None, None, Type::Alias(name))
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
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(self.namespaces.parse_qname(value)),
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
        //let struct_name = self.namespaces.new_type(QName::from(name));
        let ty = match (type_.child.0).0.complexTypeModel {
            complexTypeModel::simpleContent(_) => unimplemented!("simpleContent"),
            complexTypeModel::complexContent(ref model) => self.process_complex_content(model),
            complexTypeModel::complexTypeModel__choicevariant2(ref model) => self.process_other_complex_type_model(model),
        };

        if let Some(name) = name {
            self.types.insert(name, ty);
            RichType::new(NameHint::new(name.1), None, None, Type::Alias(name))
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
            match self.namespaces.expand_qname(*key) {
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
            name_hint.tokens.extend(&ty.name_hint.tokens);
            items.push(ty);
        }
        let name = self.namespaces.name_from_hint(&name_hint).unwrap();
        self.choices.push((name.clone(), items));
        RichType::new(name_hint, None, None, Type::Choice(name))
    }

    fn process_extension(&mut self, extension: &'ast extension_e<'input>) -> RichType<'input> {
        let mut base = None;
        for (key, &value) in extension.attrs.iter() {
            match self.namespaces.expand_qname(*key) {
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
            match self.namespaces.expand_qname(*key) {
                (SCHEMA_URI, "name") => name = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "id") => (),
                (SCHEMA_URI, "type") => type_attr = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "minOccurs") => min_occurs = Some(value.parse().unwrap()),
                (SCHEMA_URI, "maxOccurs") => max_occurs = Some(parse_max_occurs(value).unwrap()),
                (SCHEMA_URI, "abstract") => match value {
                    "true" => abstract_ = true,
                    "false" => abstract_ = false,
                    _ => panic!("Invalid value for abstract attribute: {}", value),
                },
                (SCHEMA_URI, "substitutionGroup") => substitution_group = Some(self.namespaces.parse_qname(value)),
                (SCHEMA_URI, "ref") => ref_ = Some(self.namespaces.parse_qname(value)),
                _ => panic!("Unknown attribute {} in <element>", key),
            }
        }
        if let Some(ref_) = ref_ {
            if let Some(name) = name {
                panic!("<element> has both ref={} and name={}", ref_.1, name.1);
            }
            RichType::new(NameHint::new(ref_.1), min_occurs, max_occurs, Type::Element(ref_))
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
                (Some(t), None) => RichType::new(NameHint::new(t.1), None, None, Type::Alias(t)),
                (None, None) => RichType::new(NameHint::new_empty(), None, None, Type::Any), // TODO
                (Some(t1), Some(t2)) => panic!("Element '{}' has both a type attribute ({:?}) and a child type ({:?}).", name.1, t1, t2),
            };

            self.elements.insert(name, type_);
            RichType::new(NameHint::new(name.1), min_occurs, max_occurs, Type::Element(name))
        }
    }

    fn gen_choices(&mut self, scope: &mut cg::Scope) {
        let mut module = scope.new_module("ENUMS");
        // TODO: sort the choices
        for (name, items) in self.choices.iter() {
            let mut enum_ = module.new_enum(&name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (i, item) in items.iter().enumerate() {
                let mut fields: Vec<(&str, &str)> = Vec::new();
                {
                    let writer = &mut |variant_name, type_name| { fields.push((variant_name, type_name)); };
                    self.write_type_in_struct_def(writer, &item);
                }
                let mut variant = enum_.new_variant(&self.namespaces.name_from_hint(&item.name_hint).unwrap_or(format!("{}{}", name, i)));
                if fields.len() == 1 {
                    let (_, type_name) = fields.remove(0);
                    variant.tuple(type_name);
                }
                else {
                    for (field_name, type_name) in fields {
                        variant.named(field_name, type_name);
                    }
                }
            }
        }
    }

    fn gen_elements(&mut self, scope: &mut cg::Scope) {
        let mut elements: Vec<_> = self.elements.iter().collect();

        elements.sort_by_key(|&(n,_)| n);
        for (&name, element) in elements {
            let module = &mut scope.get_or_new_module(self.namespaces.get_module_name(name));
            self.gen_element(module, name, element);
        }
    }

    fn gen_element(&self, module: &mut cg::Module, name: FullName<'input>, type_: &RichType<'input>) {
        let mut struct_ = module.new_struct(name.1).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
        struct_.field("attrs", "HashMap<&'input str, &'input str>");
        self.write_type_in_struct_def(&mut |name, type_name| { struct_.field(name, &format!("{}<'input>", type_name)); }, type_);
    }

    fn write_type_in_struct_def<'a, F>(&'a self, mut writer: &mut F, rich_type: &'a RichType<'input>)
            where F: FnMut(&'a str, &'a str), 'ast: 'a {
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
                writer(name.1, name.1);
            },
            Type::Choice(ref name) => {
                writer(name, name);
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
