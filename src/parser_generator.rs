use std::collections::HashMap;

use codegen as cg;

use parser::*;
use support::QName;

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
        {
            let mut module = cg::Module::new("UNQUAL");
            module.vis("pub");
            module.scope().raw("use std::marker::PhantomData;");
            module.scope().raw("use support::*;");
            module.scope().raw("use xmlparser::{Token, ElementEnd};");
            module.scope().raw("macro_rules! try_rollback { ($stream:expr, $tx:expr, $e:expr) => { match $e { Some(i) => i, None => { $tx.rollback($stream); return None } } } }");
            module.scope().raw("\n/////////// types\n");
            self.nsuri_to_module.insert(self.target_uri, ("UNQUAL".to_string(), module));
        }
        let mut types: Vec<_> = self.schema.types.iter().collect();
        types.sort_by_key(|&(n,_)| n);
        for (name, (min_occurs, max_occurs, attrs, mixed, type_tree)) in types {
            self.type_occurs(*min_occurs, *max_occurs, &name, &type_tree);
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// elements\n");
        let mut elements: Vec<_> = self.schema.elements.iter().collect();
        elements.sort_by_key(|&n| n.name);
        for (i, element) in elements.iter().enumerate() {
            self.element(&element, None);
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// groups\n");
        let mut groups: Vec<_> = self.schema.groups.iter().collect();
        elements.sort_by_key(|&n| n.name);
        for (name, (min_occurs, max_occurs, attrs, type_tree)) in groups.iter() {
            match type_tree {
                Some(tt) => self.type_occurs(*min_occurs, *max_occurs, &name, &tt),
                None => self.empty_type(&name),
            };
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap() // Won't panic because we inserted it before.
    }

    fn id_to_type_name(&self, id: QName) -> String {
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
        let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
        module.new_struct(name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field("PhantomData<&'input ()>");
        name.to_string()
    }

    fn element(&mut self, element: &Element, name_override: Option<&str>) -> String {
        let Element { name, attrs, mixed, type_, min_occurs, max_occurs } = element;
        let type_name = name_override.unwrap_or(name.1);
        let type_name = format!("{}_e", type_name);
        let uri = match name.0 {
            Some(prefix) => self.ns_to_uri.get(prefix).unwrap(),
            None => self.target_uri.clone(),
        };
        match type_ {
            Some(type_) => {
                let inner_type_name = self.type_(&format!("{}_inner", type_name), type_);

                let mut attrs_types = Vec::new();
                for (i, attr) in attrs.iter().enumerate() {
                    match attr {
                        Attribute::Def { name, type_: Some(type_), default } => {
                            let type_ = self.type_(&format!("{}_attr{}", type_name, i), type_);
                            attrs_types.push((name, type_));
                        },
                        Attribute::Def { name, type_: None, default } => {
                        },
                        Attribute::Any => {} // TODO
                        Attribute::Ref { .. } => {} // TODO
                        Attribute::GroupRef { .. } => {} // TODO
                    }
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                {
                    let mut s = module.new_struct(&type_name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").field("child", &format!("{}<'input>", inner_type_name));
                    for (name, type_) in attrs_types.iter() {
                        s.field(name, &format!("Option<{}<'input>>", type_));
                    }
                }
                module.scope().raw(&format!("// ^-- from {:?}", element));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "element (normal) {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        let tx = stream.transaction();
        let mut tok = stream.next().unwrap();
        loop {{
            match tok {{
                Token::Whitespaces(_) => (),
                Token::Comment(_) => (),
                _ => break,
            }}
            tok = stream.next().unwrap();
        }}
        match tok {{
            Token::ElementStart(prefix, name) => {{
                if name.to_str() == {:?} {{
"#,             type_name, type_name, name.1));
                for (name, type_) in attrs_types.iter() {
                    module.scope().raw(&format!(r#"
                        let {}: Option<{}> = None;
"#,                 name, type_));
                }
                module.scope().raw(&format!(r#"
                    loop {{
                        let tok = stream.next().unwrap();
                        match tok {{
                            Token::Whitespaces(_) => (),
                            Token::Comment(_) => (),
                            Token::Attribute(_, _) => (),
                            Token::ElementEnd(ElementEnd::Open) =>
                                return Some({} {{
                                    child: try_rollback!(stream, tx, {}::parse_xml(stream, parse_context, parent_context)),
"#,             type_name, inner_type_name));
                for (name, type_) in attrs_types {
                    module.scope().raw(&format!(r#"
                                    {},
"#,                 name));
                }
                module.scope().raw(&format!(r#"
                                }}),
                            _ => panic!(format!("Did not expect token {{:?}}", tok)),
                        }}
                    }}
                }}
                else {{
                    tx.rollback(stream);
                    None
                }}
            }},
            _ => panic!(format!("Did not expect token {{:?}}", tok)),
        }}
    }}
}}"#,           ));
            }
            None => {
                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                module.new_struct(&type_name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field("PhantomData<&'input ()>");
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "element (empty) {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Self> {{
        Some({}(Default::default()))
    }}
}}"#,           type_name, type_name, type_name));
            }
        }
        type_name
    }

    fn unbloat_element(&mut self, element: &Element, parent_name: &str, fallback_name: String) -> (String, String) {
        let Element { name: element_name, type_: element_type, min_occurs, max_occurs, .. } = element;
        let can_unbloat = match (min_occurs, max_occurs) {
            (None, None) | (None, Some(1)) | (Some(1), None) | (Some(1), Some(1)) => true,
            _ => false,
        };
        match element_type {
            /*Some(ElementType::Ref(id)) if can_unbloat => {
                let n = format!("{}_e", id.1);
                let field_typename: String = self.id_to_type_name(QName(id.0, &n));
                (element_name.to_string(), field_typename)
            },
            Some(ElementType::Custom(id)) |
            Some(ElementType::GroupRef(id)) if can_unbloat => {
                let field_typename = self.id_to_type_name(*id);
                (element_name.to_string(), field_typename)
            },
            */
            _ => { // Normal case, no unbloat
                let element_name = element_name.1.to_string(); // TODO: deduplication
                let field_typename = self.element(element, Some(&format!("{}__{}", parent_name, element_name)));
                (element_name, field_typename)
            },
        }
    }

    fn type_occurs(&mut self, min_occurs: Option<usize>, max_occurs: Option<usize>, name: &str, type_tree: &ElementType) -> String {
        match (min_occurs, max_occurs) {
            (None, None) | (None, Some(1)) | (Some(1), None) | (Some(1), Some(1)) =>
                self.type_(name, type_tree),
            (Some(0), None) | (Some(0), Some(1)) => {
                let item_name = format!("{}_item", name);
                let mut s = cg::Struct::new(name);
                let child_typename = self.type_(&item_name, type_tree);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(format!("Option<{}<'input>>", child_typename));
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "option {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        Some({}({}::parse_xml(stream, parse_context, parent_context)))
    }}
}}"#,           name, name, name, child_typename));
                name.to_string()
            }
            (_, _) => {
                let item_name = format!("{}_item", name);
                let mut s = cg::Struct::new(name);
                let child_typename = self.type_(&item_name, type_tree);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(format!("Vec<{}<'input>>", child_typename));
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "vec {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        let mut items = Vec::new();
        while let Some(new_item) = {}::parse_xml(stream, parse_context, parent_context) {{
            items.push(new_item);
        }}
        Some({}(items))
    }}
}}"#,           name, name, child_typename, name)); // TODO: validate the size
                name.to_string()
            }
        }
    }
    fn type_(&mut self, name: &str, type_tree: &ElementType) -> String {
        match type_tree {
            ElementType::String | ElementType::Date => {
                "XmlString".to_string()
            }
            ElementType::Sequence(items) => {
                let mut s = cg::Struct::new(name);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input");
                let mut fields = Vec::new();
                for (i, (min_occurs, max_occurs, item)) in items.iter().enumerate() {
                    let element_name = format!("seqfield{}", i);
                    let field_typename = self.type_occurs(*min_occurs, *max_occurs, &format!("{}__{}", name, element_name), item);
                    s.field(&element_name, &format!("{}<'input>", field_typename)); // TODO: make sure there is no name conflict
                    fields.push((element_name, field_typename));
                }
                if items.len() == 0 {
                    s.tuple_field("PhantomData<&'input ()>");
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "sequence {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        let tx = stream.transaction();
        Some({} {{
"#,             name, name, name));
                if fields.len() == 0 {
                    module.scope().raw(&format!(r#"
            0: Default::default()
"#,                 ));
                }
                for (item_name, item_typename) in fields {
                    module.scope().raw(&format!(r#"
            {}: try_rollback!(stream, tx, {}::parse_xml(stream, parse_context, parent_context)),
"#,                 item_name, item_typename));
                }
                module.scope().raw(&format!(r#"
        }})
    }}
}}"#,           ));
                name.to_string()
            },
            ElementType::Ref(id) => {
                let escaped_name = escape_keyword(&format!("{}_e", id.1));
                let mut id = QName(id.0, &escaped_name);
                self.id_to_type_name(id)
            },
            ElementType::GroupRef(id) => {
                self.id_to_type_name(*id)
            },
            ElementType::Custom(id_) => {
                let type_name = self.id_to_type_name(*id_);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(format!("{}<'input>", type_name));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "custom {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        {}::parse_xml(stream, parse_context, parent_context).map({})
    }}
}}"#,           name, name, type_name, name));
                name.to_string()
            }
            ElementType::Extension(base, inner) => {
                let struct_name = escape_keyword(name);
                let mut s = cg::Struct::new(&struct_name);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input");
                let base_typename = self.id_to_type_name(*base);
                s.field("BASE", format!("{}<'input>", base_typename));
                let extension_type_name = format!("{}__extension", name);
                let extension_type_name = self.type_(&extension_type_name, inner);
                s.field("EXTENSION", format!("{}<'input>", extension_type_name));
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "extension {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        let tx = stream.transaction();
        Some({} {{
            BASE: try_rollback!(stream, tx, {}::parse_xml(stream, parse_context, parent_context)),
"#,             name, name, name, base_typename));
                module.scope().raw(&format!(r#"
            EXTENSION: try_rollback!(stream, tx, {}::parse_xml(stream, parse_context, parent_context)),
"#,             extension_type_name));
                module.scope().raw(&format!(r#"
        }})
    }}
}}
"#,             ));
                struct_name
            },
            ElementType::Union(member_types, items) => {
                let struct_name = escape_keyword(name);
                let mut s = cg::Struct::new(&struct_name);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input");
                if let Some(member_types) = member_types {
                    for (i, member_type) in member_types.iter().enumerate() {
                        let member_name = format!("member{}", i);
                        s.field(&member_name, &format!("{}<'input>", self.id_to_type_name(*member_type)));
                    }
                }
                if let Some(items) = items {
                    for (i, (min_occurs, max_occurs, item)) in items.iter().enumerate() {
                        let item_name = format!("item{}", i);
                        let item_type_name = format!("{}__item{}", name, i);
                        let item_type = self.type_occurs(*min_occurs, *max_occurs, &item_type_name, item);
                        s.field(&item_name, &format!("{}<'input>", item_type));
                    }
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                struct_name
            },
            ElementType::Choice(items) => {
                let enum_name = escape_keyword(name);
                let mut e = cg::Enum::new(&enum_name);
                e.vis("pub").derive("Debug").derive("PartialEq").generic("'input");
                let mut last_item_name = None;
                let mut variants = Vec::new();
                for (i, (min_occurs, max_occurs, item)) in items.iter().enumerate() {
                    let element_name = format!("choicevariant{}", i);
                    let field_typename = self.type_occurs(*min_occurs, *max_occurs, &format!("{}__{}", name, element_name), item);
                    e.new_variant(&element_name).tuple(&format!("Box<{}<'input>>", field_typename));
                    last_item_name = Some(element_name.clone());
                    variants.push((element_name, field_typename));
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_enum(e);
                let last_item_name = last_item_name.expect(&format!("enum {} has no variant", enum_name));
                module.scope().raw(&format!("impl<'input> Default for {}<'input> {{ fn default() -> {}<'input> {{ {}::{}(Default::default()) }} }}", enum_name, enum_name, enum_name, last_item_name )); // TODO: remove this, that's a temporary hack
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "choice {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
"#,             enum_name, enum_name));
                for (item_name, item_typename) in variants {
                    module.scope().raw(&format!(r#"
        match {}::parse_xml(stream, parse_context, parent_context) {{ Some(r) => return Some({}::{}(Box::new(r))), None => () }}
"#,                 item_typename, enum_name, item_name));
                }
                module.scope().raw(&format!(r#"
        None
    }}
}}"#,           ));
                enum_name
            },
            ElementType::List(List::ComplexList(mixed, item_type)) => {
                let struct_name = escape_keyword(name);
                let item_type_name = format!("{}__valuetype", name);
                let type_ = self.type_(&item_type_name, item_type);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(&format!("Vec<{}<'input>>", type_));
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                struct_name
            },
            ElementType::List(List::SimpleList(item_type)) => {
                let struct_name = escape_keyword(name);
                let type_name = self.id_to_type_name(*item_type);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(&format!("Vec<{}<'input>>", type_name));
                struct_name
            },
            ElementType::Element(e) => {
                let (element_name, field_typename) = self.unbloat_element(e, name, format!("{}__element", name));
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(format!("{}<'input>", field_typename));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "elementtype element {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {{
        {}::parse_xml(stream, parse_context, parent_context).map({})
    }}
}}"#,           name, name, field_typename, name));
                name.to_string()
            }
            _ => unimplemented!("{:?}", type_tree),
        }
    }
}
