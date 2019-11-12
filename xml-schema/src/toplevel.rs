//! Collects named entities and global information from the root of the AST.
use std::collections::hash_map::{Entry, HashMap};

use names::FullName;
use parser::xs;

fn insert_unique<'ast, 'input: 'ast, T>(
    type_name: &'static str,
    map: &mut HashMap<FullName<'input>, T>,
    name: FullName<'input>,
    sub_ast: T,
) {
    let entry = map.entry(name);
    match entry {
        Entry::Occupied(_) => panic!("Duplicate {}: {:?}", type_name, name),
        Entry::Vacant(e) => {
            e.insert(sub_ast);
        }
    }
}

#[derive(Debug)]
pub struct Toplevel<'ast, 'input: 'ast> {
    pub target_namespace: Option<&'input str>,
    pub element_form_default_qualified: bool,
    pub attribute_form_default_qualified: bool,
    pub elements: HashMap<FullName<'input>, &'ast xs::Element<'input>>,
    pub simple_types: HashMap<FullName<'input>, &'ast xs::SimpleType<'input>>,
    pub complex_types: HashMap<FullName<'input>, &'ast xs::ComplexType<'input>>,
    pub groups: HashMap<FullName<'input>, &'ast xs::Group<'input>>,
    pub attribute_groups: HashMap<FullName<'input>, &'ast xs::AttributeGroup<'input>>,
}

impl<'ast, 'input: 'ast> Toplevel<'ast, 'input> {
    pub fn new(ast: &'ast xs::Schema<'input>) -> Toplevel<'ast, 'input> {
        let target_namespace = ast.attr_target_namespace.as_ref().map(|t| t.0);
        let element_form_default_qualified =
            match ast.attr_element_form_default.as_ref().map(|x| ((x.0).0).0) {
                Some("qualified") => true,
                Some("unqualified") | None => false,
                _ => unreachable!(),
            };
        let attribute_form_default_qualified = match ast
            .attr_attribute_form_default
            .as_ref()
            .map(|x| ((x.0).0).0)
        {
            Some("qualified") => true,
            Some("unqualified") | None => false,
            _ => unreachable!(),
        };
        let mut toplevel = Toplevel {
            target_namespace,
            element_form_default_qualified,
            attribute_form_default_qualified,
            elements: HashMap::new(),
            simple_types: HashMap::new(),
            complex_types: HashMap::new(),
            groups: HashMap::new(),
            attribute_groups: HashMap::new(),
        };
        toplevel.process_ast(ast);
        toplevel
    }

    pub fn process_ast(&mut self, ast: &'ast xs::Schema<'input>) {
        for top_level_item in ast.sequence_schema_top_annotation.iter() {
            match top_level_item.schema_top {
                xs::SchemaTop::Redefinable(ref r) => self.process_redefinable(r),
                xs::SchemaTop::Element(ref e) => self.process_element(e),
                xs::SchemaTop::Attribute(_) => unimplemented!("top-level attribute"),
                xs::SchemaTop::Notation(_) => unimplemented!("notation"),
            }
        }
    }

    fn process_redefinable(&mut self, r: &'ast xs::Redefinable<'input>) {
        match r {
            xs::Redefinable::SimpleType(ref e) => self.process_simple_type(e),
            xs::Redefinable::ComplexType(e) => self.process_complex_type(e),
            xs::Redefinable::Group(e) => self.process_named_group(e),
            xs::Redefinable::AttributeGroup(e) => self.process_attribute_group(e),
        }
    }

    fn process_element(&mut self, element: &'ast xs::Element<'input>) {
        let name = FullName::new(self.target_namespace, element.attr_name.0);
        insert_unique("element", &mut self.elements, name, element);
    }

    fn process_simple_type(&mut self, simple_type: &'ast xs::SimpleType<'input>) {
        let name = FullName::new(self.target_namespace, simple_type.attr_name.0.clone());
        self.simple_types.insert(name, simple_type);
    }

    fn process_complex_type(&mut self, complex_type: &'ast xs::ComplexType<'input>) {
        let name = FullName::new(self.target_namespace, complex_type.attr_name.0.clone());
        self.complex_types.insert(name, complex_type);
    }

    fn process_named_group(&mut self, group: &'ast xs::Group<'input>) {
        let name = FullName::new(self.target_namespace, group.attr_name.0.clone());
        self.groups.insert(name, group);
    }

    fn process_attribute_group(&mut self, attribute_group: &'ast xs::AttributeGroup<'input>) {
        let name = FullName::new(self.target_namespace, attribute_group.attr_name.0.clone());
        self.attribute_groups.insert(name, attribute_group);
    }
}
