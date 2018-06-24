use std::collections::HashMap;

use codegen as cg;

use parser::*;

const KEYWORDS: &[&'static str] = &["override"];
fn escape_keyword(name: &str) -> String {
    if KEYWORDS.contains(&name) {
        format!("{}_", name)
    }
    else {
        name.to_string()
    }
}

#[derive(Debug)]
pub struct ParserGenerator<'a> {
    //document: &'a Document<'a>
    schema: &'a Schema<'a>,
    target_uri: &'a str,
    ns_to_uri: HashMap<String, &'a str>,
    pub nsuri_to_module: HashMap<&'a str, (String, cg::Module)>,
}

impl<'a> ParserGenerator<'a> {
    pub fn new(document: &'a Document<'a>) -> ParserGenerator<'a> {
        ParserGenerator {
            schema: document.schema.as_ref().unwrap(),
            target_uri: document.schema.as_ref().unwrap().target_namespace,
            ns_to_uri: document.schema.as_ref().unwrap().namespaces.clone(),
            nsuri_to_module: HashMap::new(),
        }
    }

    /*
    pub fn module(&mut self, uri: Option<&str>) -> &cg::Module {
        match (uri, self.nsuri_to_module.get(uri.unwrap_or(self.target_uri))) {
            (_, Some(m)) => return m, // We already did the work
            (None, None) => (),
            (Some(uri), None) if uri == self.target_uri => (),
            (Some(uri), None) => panic!(format!("Reference to namespace URI {} but that URI is unknown.", uri)),
        }
        self.gen_unqual_module()
    }
    */

    pub fn gen_unqual_module(&mut self) -> &mut (String, cg::Module) {
        self.nsuri_to_module.insert(self.target_uri, ("UNQUAL".to_string(), cg::Module::new("UNQUAL")));
        for (name, (attrs, mixed, type_tree)) in self.schema.types.iter() {
            self.type_(&name, &type_tree);
        }
        for (name, (attrs, type_tree)) in self.schema.groups.iter() {
            match type_tree {
                Some(tt) => self.type_(&name, &tt),
                None => self.empty_type(&name),
            };
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap() // Won't panic because we inserted it before.
    }

    fn id_to_type_name(&self, id: Id) -> String {
        match id.0 {
            None => id.1.to_string(),
            Some(ns) => {
                let uri = self.ns_to_uri.get(ns).expect(&format!("Unknown namespace: {:?}", ns));
                let (module_name, _) = self.nsuri_to_module.get(uri).expect("ref to unknown ns URI");
                format!("super::{}::{}", escape_keyword(module_name), escape_keyword(id.1))
            }
        }
    }

    fn empty_type(&mut self, name: &str) -> String {
        let mut s = cg::Struct::new(name);
        let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
        module.push_struct(s);
        name.to_string()
    }

    fn type_(&mut self, name: &str, type_tree: &ElementType) -> String {
        match type_tree {
            ElementType::String | ElementType::Date => {
                "String".to_string()
            }
            ElementType::Sequence(items) => {
                let mut s = cg::Struct::new(name);
                for (i, item) in items.iter().enumerate() {
                    let Element { name: element_name, attrs, mixed, type_ } = item;
                    let default_element_name = format!("anonymous{}", i);
                    let element_name = element_name.unwrap_or(Id(None, &default_element_name));
                    let type_name = match element_name.0 {
                        Some(ns) => self.id_to_type_name(element_name),
                        None => format!("{}__{}", name, element_name.1),
                    };
                    let type_ = type_.as_ref().unwrap();
                    let field_typename = self.type_(&type_name, type_);
                    s.field(element_name.1, field_typename); // TODO: make sure there is no name conflict
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                name.to_string()
            },
            ElementType::Ref(id) | ElementType::GroupRef(id) => {
                self.id_to_type_name(*id)
            },
            ElementType::Custom(id_) => {
                let type_name = self.id_to_type_name(*id_);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(name).tuple_field(type_name);
                name.to_string()
            }
            ElementType::Extension(base, attrs, inner) => {
                let mut s = cg::Struct::new(name);
                s.field("base", self.id_to_type_name(*base));
                if let Some(inner) = inner {
                    let inner_type_name = format!("{}__extension", name);
                    s.field("extension", self.type_(&inner_type_name, inner));
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                name.to_string()
            },
            ElementType::Union(member_types, items) => {
                let mut e = cg::Struct::new(name);
                if let Some(member_types) = member_types {
                    for (i, member_type) in member_types.iter().enumerate() {
                        let member_name = format!("member{}", i);
                        e.field(&member_name, &self.id_to_type_name(*member_type));
                    }
                }
                if let Some(items) = items {
                    for (i, item) in items.iter().enumerate() {
                        let item_name = format!("item{}", i);
                        let item_type_name = format!("{}__item{}", name, i);
                        e.field(&item_name, &self.type_(&item_type_name, item.type_.as_ref().unwrap()));
                    }
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(e);
                name.to_string()
            },
            ElementType::Choice(items) => {
                let mut e = cg::Enum::new(name);
                /*
                if let Some(member_types) = member_types {
                    for (i, member_type) in member_types.iter().enumerate() {
                        let member_name = format!("member{}", i);
                        e.new_variant(&member_name).tuple(&self.id_to_type_name(member_type));
                    }
                }
                if let Some(items) = items {
                    */
                    for (i, item) in items.iter().enumerate() {
                        let item_name = format!("item{}", i);
                        let item_type_name = format!("{}__item{}", name, i);
                        let item_type_name = self.type_(&item_type_name, item.type_.as_ref().unwrap());
                        e.new_variant(&item_name).tuple(&format!("Box<{}>", item_type_name));
                    }
                //}
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_enum(e);
                name.to_string()
            },
            ElementType::List(mixed, item_type) => {
                let item_type_name = format!("{}__valuetype", name);
                let type_ = self.type_(&item_type_name, item_type);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(name).tuple_field(&format!("Vec<{}>", type_));
                name.to_string()
            },
            ElementType::Any => {
                "Vec<u8>".to_string()
            },
            _ => unimplemented!("{:?}", type_tree),
        }
    }
}
