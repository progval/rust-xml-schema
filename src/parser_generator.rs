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
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// types\n");
        for (name, (attrs, mixed, type_tree)) in self.schema.types.iter() {
            self.type_(&name, &type_tree);
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// elements\n");
        for (i, element) in self.schema.elements.iter().enumerate() {
            self.element(&element, None);
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// groups\n");
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
        let s = cg::Struct::new(name);
        let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
        module.push_struct(s);
        name.to_string()
    }

    fn element(&mut self, element: &Element, name_prefix: Option<&str>) -> String {
        let Element { name, attrs, mixed, type_ } = element;
        let name = name.unwrap_or(Id(None, "ANONYMOUSELEMENT"));
        let type_name = match name_prefix {
            Some(name_prefix) => format!("{}__{}", name_prefix, name.1),
            None => name.1.to_string(),
        };
        let type_name = format!("{}_e", type_name);
        let uri = name.0.and_then(|ns| self.ns_to_uri.get(ns)).unwrap_or(&self.target_uri).clone();
        match type_ {
            Some(type_) => {
                let inner_type_name = self.type_(&format!("{}_inner", type_name), type_);

                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                module.new_struct(&type_name).tuple_field(&inner_type_name);
                module.scope().raw(&format!("// ^-- from {:?}", element));
            }
            None => {
                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                module.new_struct(&type_name);
            }
        }
        type_name
    }
    fn type_(&mut self, name: &str, type_tree: &ElementType) -> String {
        match type_tree {
            ElementType::String | ElementType::Date => {
                "String".to_string()
            }
            ElementType::Sequence(items) => {
                let mut s = cg::Struct::new(name);
                for (i, item) in items.iter().enumerate() {
                    let Element { name: element_name, .. } = item;
                    let element_name = match element_name {
                        Some(id) => id.1.to_string(), // TODO: deduplication
                        None => format!("anonymous{}", i),
                    };
                    let field_typename = self.element(item, Some(&format!("{}__{}", name, element_name)));
                    s.field(&element_name, field_typename); // TODO: make sure there is no name conflict
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                name.to_string()
            },
            ElementType::Ref(id) => {
                let escaped_name = escape_keyword(&format!("{}_e", id.1));
                let mut id = Id(id.0, &escaped_name);
                self.id_to_type_name(id)
            },
            ElementType::GroupRef(id) => {
                self.id_to_type_name(*id)
            },
            ElementType::Custom(id_) => {
                let type_name = self.id_to_type_name(*id_);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(name).tuple_field(type_name);
                name.to_string()
            }
            ElementType::Extension(base, attrs, inner) => {
                let struct_name = escape_keyword(name);
                let mut s = cg::Struct::new(&struct_name);
                s.field("base", self.id_to_type_name(*base));
                if let Some(inner) = inner {
                    let inner_type_name = format!("{}__extension", name);
                    s.field("extension", self.type_(&inner_type_name, inner));
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                struct_name
            },
            ElementType::Union(member_types, items) => {
                let struct_name = escape_keyword(name);
                let mut e = cg::Struct::new(&struct_name);
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
                struct_name
            },
            ElementType::Choice(items) => {
                let enum_name = escape_keyword(name);
                let mut e = cg::Enum::new(&enum_name);
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
                enum_name
            },
            ElementType::ComplexList(mixed, item_type) => {
                let struct_name = escape_keyword(name);
                let item_type_name = format!("{}__valuetype", name);
                let type_ = self.type_(&item_type_name, item_type);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(&struct_name).tuple_field(&format!("Vec<{}>", type_));
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                struct_name
            },
            ElementType::SimpleList(item_type) => {
                let struct_name = escape_keyword(name);
                let type_name = self.id_to_type_name(*item_type);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(&struct_name).tuple_field(&format!("Vec<{}>", type_name));
                struct_name
            },
            ElementType::Any => {
                "Vec<u8>".to_string()
            },
            _ => unimplemented!("{:?}", type_tree),
        }
    }
}
