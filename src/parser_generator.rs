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
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("use xml_schema::support::*;");
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

    fn element(&mut self, element: &Element, name_override: Option<&str>) -> String {
        let Element { name, attrs, mixed, type_ } = element;
        let type_name = match (name, name_override) {
            (_, Some(name_override)) => name_override.to_string(),
            (Some(name), None) => name.1.to_string(),
            (n1, n2) => panic!(format!("Conflict: {:?} {:?}", n1, n2)),
        };
        let type_name = format!("{}_e", type_name);
        let uri = name.unwrap_or(Id(None, "")).0.and_then(|ns| self.ns_to_uri.get(ns)).unwrap_or(&self.target_uri).clone();
        match type_ {
            Some(ElementType::Custom(id)) => {
                let inner_type_name = self.id_to_type_name(*id);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                module.new_struct(&type_name).tuple_field(&inner_type_name);
                module.scope().raw(&format!("// ^-- from {:?}", element));
            }
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

    fn unbloat_element(&mut self, element: &Element, parent_name: &str, fallback_name: String) -> (String, String) {
        let Element { name: element_name, type_: element_type, .. } = element;
        match (element_name, element_type) {
            (Some(id), _) => {
                let element_name = id.1.to_string(); // TODO: deduplication
                let field_typename = self.element(element, Some(&format!("{}__{}", parent_name, element_name)));
                (element_name, field_typename)
            },
            (None, Some(ElementType::Ref(id))) => {
                let element_name = escape_keyword(id.1);
                let n = format!("{}_e", id.1);
                let field_typename: String = self.id_to_type_name(Id(id.0, &n));
                (element_name, field_typename)
            },
            (None, Some(ElementType::Custom(id))) |
            (None, Some(ElementType::GroupRef(id))) => {
                let element_name = escape_keyword(id.1);
                let field_typename = self.id_to_type_name(*id);
                (element_name, field_typename)
            },
            (None, _) => {
                let field_typename = self.element(element, Some(&format!("{}__{}", parent_name, fallback_name)));
                (fallback_name, field_typename)
            }
        }
    }

    fn type_(&mut self, name: &str, type_tree: &ElementType) -> String {
        match type_tree {
            ElementType::String | ElementType::Date => {
                "String".to_string()
            }
            ElementType::Sequence(items) => {
                let mut s = cg::Struct::new(name);
                for (i, item) in items.iter().enumerate() {
                    let (element_name, field_typename) = self.unbloat_element(item, name, format!("seqfield{}", i));
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
                        let (element_name, field_typename) = self.unbloat_element(item, name, format!("choicevariant{}", i));
                        e.new_variant(&element_name).tuple(&format!("Box<{}>", field_typename));
                    }
                //}
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_enum(e);
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
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
