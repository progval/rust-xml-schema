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

pub const MACROS: &'static str = r#"
macro_rules! impl_parsexml_for_element {
    ( $element_type:ident, $element_name:expr, $child_type:ident ) => {
        impl<'input> ParseXml<'input> for $element_type<'input> {
            const NODE_NAME: &'static str = "element (normal) $element_type";
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
                        if name.to_str() == $element_name {
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
                                        let ret = Some($element_type {
                                            attrs,
                                            child: try_rollback!(stream, tx, $child_type::parse_xml(stream, parse_context, parent_context)),
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
                                    Token::ElementEnd(ElementEnd::Empty) =>
                                        return Some($element_type {
                                            attrs,
                                            child: $child_type::default(),
                                        }),
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

macro_rules! impl_parsexml_for_substitution {
    ( $element_type:ident, $($variant_name:ident => $variant_type:ident,)* ) => {
        impl<'input> ParseXml<'input> for $element_type<'input> {
            const NODE_NAME: &'static str = "element (substitution group) $element_type";
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
                        tx.rollback(stream);
                        $(
                            match $variant_type::parse_xml(stream, parse_context, parent_context) {
                                Some(s) => return Some($element_type::$variant_name(s)),
                                None => (),
                            }
                        )*
                        None
                    }
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


#[derive(Debug)]
pub struct ParserGenerator<'a> {
    //document: &'a Document<'a>
    schema: &'a Schema<'a>,
    target_uri: &'a str,
    ns_to_uri: HashMap<String, &'a str>,
    pub nsuri_to_module: HashMap<&'a str, (String, cg::Module)>,
    generated_elements: HashMap<QName<'a>, String>,
    generated_types: HashMap<String, String>,
}

impl<'a> ParserGenerator<'a> {
    pub fn new(document: &'a Document<'a>) -> ParserGenerator<'a> {
        ParserGenerator {
            schema: document.schema.as_ref().unwrap(),
            target_uri: document.schema.as_ref().unwrap().target_namespace,
            ns_to_uri: document.schema.as_ref().unwrap().namespaces.clone(),
            nsuri_to_module: HashMap::new(),
            generated_elements: HashMap::new(),
            generated_types: HashMap::new(),
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
            module.scope().raw("use std::collections::HashMap;");
            module.scope().raw("use std::marker::PhantomData;");
            module.scope().raw("use support::*;");
            module.scope().raw("use xmlparser::{Token, ElementEnd};");
            module.scope().raw("macro_rules! try_rollback { ($stream:expr, $tx:expr, $e:expr) => { match $e { Some(i) => i, None => { $tx.rollback($stream); return None } } } }");
            module.scope().raw("\n/////////// types\n");
            self.nsuri_to_module.insert(self.target_uri, ("UNQUAL".to_string(), module));
        }
        let mut types: Vec<_> = self.schema.types.iter().collect();
        types.sort_by_key(|&(n,_)| n);
        for (name, (min_occurs, max_occurs, attrs, mixed, abstract_, type_tree)) in types {
            self.type_occurs(*min_occurs, *max_occurs, &name, &type_tree);
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// elements\n");
        let mut elements: Vec<_> = self.schema.elements.iter().collect();
        elements.sort_by_key(|&(n,_)| n);
        for (i, (_name, element)) in elements.iter().enumerate() {
            self.element(&element, None);
        }
        self.nsuri_to_module.get_mut(self.target_uri).unwrap().1.scope().raw("\n/////////// groups\n");
        let mut groups: Vec<_> = self.schema.groups.iter().collect();
        groups.sort_by_key(|&(n,_)| n);
        for (name, (min_occurs, max_occurs, attrs, type_tree)) in groups.iter() {
            match type_tree {
                Some(tt) => { self.type_occurs(*min_occurs, *max_occurs, &name, &tt); },
                None => { self.empty_type(&name); },
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

    fn element(&mut self, element: &Element<'a>, name_override: Option<&str>) -> String {
        if let Some(name) = self.generated_elements.get(&element.name) {
            return name.to_string();
        }
        let Element { name, attrs, mixed, type_, min_occurs, max_occurs, abstract_ } = element;
        let type_name = name_override.unwrap_or(name.1);
        let type_name = format!("{}_e", type_name);
        let uri = match name.0 {
            Some(prefix) => self.ns_to_uri.get(prefix).unwrap(),
            None => self.target_uri.clone(),
        };

        let (real_name, type_name) = if let Some(substitutes) = self.schema.substitution_groups.get(name.1) {
            if substitutes.len() > 0 {
                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                let (real_name, type_name, default_impl) = {
                    let mut e = module.new_enum(&type_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
                    let mut last_item_name = None;
                    for substitute in substitutes.iter() {
                        e.new_variant(substitute).tuple(&format!("{}_e<'input>", substitute));
                        last_item_name = Some(substitute.clone())
                    }
                    let last_item_name = last_item_name.unwrap();
                    let real_name = type_name.clone();
                    let type_name = format!("{}_self", type_name);
                    let default_impl = format!("impl<'input> Default for {}<'input> {{ fn default() -> {}<'input> {{ {}::{}(Default::default()) }} }}", real_name, real_name, real_name, last_item_name ); // TODO: remove this, that's a temporary hack
                    (real_name, type_name, default_impl)
                };
                module.scope().raw(&format!(r#"impl_parsexml_for_substitution!({}, "#, real_name));
                for substitute in substitutes.iter() {
                    module.scope().raw(&format!(r#"{} => {}_e,"#, substitute, substitute));
                }
                module.scope().raw(&format!(r#");"#));
                module.scope().raw(&default_impl);
                (real_name, type_name)
            }
            else {
                (type_name.clone(), type_name)
            }
        }
        else {
            (type_name.clone(), type_name)
        };


        match type_ {
            Some(type_) => {
                let (_, inner_type_name) = self.type_occurs(*min_occurs, *max_occurs, &format!("{}_inner", type_name), type_);


                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                {
                    let mut s = module.new_struct(&type_name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").field("attrs", &format!("HashMap<QName<'input>, &'input str>")).field("child", &format!("{}<'input>", inner_type_name));
                }
                module.scope().raw(&format!("// ^-- from {:?}", element));
                module.scope().raw(&format!(r#"impl_parsexml_for_element!({}, {:?}, {});"#,
                    type_name, name.1, inner_type_name));
            }
            None => {
                let (_, ref mut module) = self.nsuri_to_module.get_mut(uri).unwrap();
                module.new_struct(&type_name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field("PhantomData<&'input ()>");
                module.scope().raw(&format!("// ^-- from {:?}", element));
                module.scope().raw(&format!(r#"
impl<'input> ParseXml<'input> for {}<'input> {{
    const NODE_NAME: &'static str = "element (empty) {}";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Self> {{
        Some({}(Default::default()))
    }}
}}"#,           type_name, type_name, type_name));
            }
        }
        self.generated_elements.insert(element.name, real_name.clone());
        real_name
    }

    fn type_occurs(&mut self, min_occurs: Option<usize>, max_occurs: Option<usize>, name: &str, type_tree: &ElementType<'a>) -> (String, String) {
        match (min_occurs, max_occurs) {
            (None, None) | (None, Some(1)) | (Some(1), None) | (Some(1), Some(1)) =>
                self.type_(name, type_tree),
            (Some(0), None) | (Some(0), Some(1)) => {
                let (field_name, child_typename) = self.type_(name, type_tree);
                let name = format!("opt_{}", field_name);
                if let Some(type_name) = self.generated_types.get(&name) {
                    return (field_name, type_name.clone());
                }
                self.generated_types.insert(name.clone(), name.clone());
                let mut s = cg::Struct::new(&name);
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
                (field_name, name.to_string())
            }
            (_, _) => {
                let (field_name, child_typename) = self.type_(&name, type_tree);
                let name = format!("many_{}", field_name);
                if let Some(type_name) = self.generated_types.get(&name) {
                    return (field_name, type_name.clone());
                }
                self.generated_types.insert(name.clone(), name.clone());
                let mut s = cg::Struct::new(&name);
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
                (field_name, name.to_string())
            }
        }
    }
    fn type_(&mut self, name: &str, type_tree: &ElementType<'a>) -> (String, String) {
        match type_tree {
            ElementType::String => {
                ("string".to_string(), "XmlString".to_string())
            }
            ElementType::Date => {
                ("date".to_string(), "XmlString".to_string())
            }
            ElementType::Sequence(items) => {
                let mut s = cg::Struct::new(name);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input");
                let mut fields = Vec::new();
                for (i, (min_occurs, max_occurs, item)) in items.iter().enumerate() {
                    match (min_occurs.unwrap_or(1), max_occurs.unwrap_or(1), item) {
                        (1, 1, ElementType::Element(elem)) => {
                            let field_name = elem.name.1;
                            let field_type_name = self.element(elem, None);
                            s.field(&field_name, &format!("{}<'input>", field_type_name));
                            fields.push((field_name.to_string(), field_type_name.to_string())); // TODO: namespace
                        },
                        (1, 1, ElementType::GroupRef(ref_name)) => {
                            let field_name = escape_keyword(ref_name.1);
                            let field_type_name = escape_keyword(ref_name.1);
                            s.field(&field_name, &format!("{}<'input>", field_type_name));
                            fields.push((field_name.to_string(), field_type_name.to_string())); // TODO: namespace
                        },
                        (1, 1, ElementType::Ref(ref_name)) => {
                            let field_name = escape_keyword(ref_name.1);
                            let field_type_name = escape_keyword(&format!("{}_e", ref_name.1));
                            s.field(&field_name, &format!("{}<'input>", field_type_name));
                            fields.push((field_name.to_string(), field_type_name.to_string())); // TODO: namespace
                        },
                        _ => {
                            let element_name = format!("seqfield{}", i);
                            let (element_name, field_typename) = self.type_occurs(*min_occurs, *max_occurs, &format!("{}__{}", name, element_name), item);
                            s.field(&element_name, &format!("{}<'input>", field_typename)); // TODO: make sure there is no name conflict
                            fields.push((element_name, field_typename));
                        }
                    }
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
                (name.to_string(), name.to_string())
            },
            ElementType::Ref(id) => {
                let escaped_name = escape_keyword(&format!("{}_e", id.1));
                let mut newid = QName(id.0, &escaped_name);
                (id.1.to_string(), self.id_to_type_name(newid))
            },
            ElementType::GroupRef(id) => {
                (id.1.to_string(), self.id_to_type_name(*id))
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
                (id_.1.to_string(), name.to_string())
            }
            ElementType::Extension(base, items) => {
                let struct_name = escape_keyword(name);
                let mut s = cg::Struct::new(&struct_name);
                s.vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input");
                let base_typename = self.id_to_type_name(*base);
                s.field("BASE", format!("{}<'input>", base_typename));
                let mut fields = Vec::new();
                for (i, (min_occurs, max_occurs, item)) in items.iter().enumerate() {
                    let element_name = format!("extfield{}", i);
                    let (element_name, field_typename) = self.type_occurs(*min_occurs, *max_occurs, &format!("{}__{}", name, element_name), item);
                    s.field(&element_name, &format!("{}<'input>", field_typename)); // TODO: make sure there is no name conflict
                    fields.push((element_name, field_typename));
                }
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
                for (item_name, item_typename) in fields {
                    module.scope().raw(&format!(r#"
            {}: try_rollback!(stream, tx, {}::parse_xml(stream, parse_context, parent_context)),
"#,                 item_name, item_typename));
                }
                module.scope().raw(&format!(r#"
        }})
    }}
}}
"#,             ));
                (name.to_string(), struct_name)
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
                        let (item_name, item_type) = self.type_occurs(*min_occurs, *max_occurs, &item_type_name, item);
                        s.field(&item_name, &format!("{}<'input>", item_type));
                    }
                }
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.push_struct(s);
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                (name.to_string(), struct_name)
            },
            ElementType::Choice(items) => {
                let enum_name = escape_keyword(name);
                let mut e = cg::Enum::new(&enum_name);
                e.vis("pub").derive("Debug").derive("PartialEq").generic("'input");
                let mut last_item_name = None;
                let mut variants = Vec::new();
                for (i, (min_occurs, max_occurs, item)) in items.iter().enumerate() {
                    match (min_occurs.unwrap_or(1), max_occurs.unwrap_or(1), item) {
                        (1, 1, ElementType::Element(elem)) => {
                            let variant_name = elem.name.1;
                            let variant_type_name = self.element(elem, None);
                            e.new_variant(&variant_name).tuple(&format!("Box<{}<'input>>", variant_type_name));
                            variants.push((variant_name.to_string(), variant_type_name.to_string())); // TODO: namespace
                            last_item_name = Some(variant_name.to_string());
                        },
                        (1, 1, ElementType::GroupRef(ref_name)) => {
                            let variant_name = escape_keyword(ref_name.1);
                            let variant_type_name = escape_keyword(ref_name.1);
                            e.new_variant(&variant_name).tuple(&format!("Box<{}<'input>>", variant_type_name));
                            variants.push((variant_name.to_string(), variant_type_name.to_string())); // TODO: namespace
                            last_item_name = Some(variant_name.to_string());
                        },
                        (1, 1, ElementType::Ref(ref_name)) => {
                            let variant_name = escape_keyword(ref_name.1);
                            let variant_type_name = escape_keyword(&format!("{}_e", ref_name.1));
                            e.new_variant(&variant_name).tuple(&format!("Box<{}<'input>>", variant_type_name));
                            variants.push((variant_name.to_string(), variant_type_name.to_string())); // TODO: namespace
                            last_item_name = Some(variant_name.to_string());
                        },
                        _ => {
                            let element_name = format!("choicevariant{}", i);
                            let (element_name, field_typename) = self.type_occurs(*min_occurs, *max_occurs, &format!("{}__{}", name, element_name), item);
                            e.new_variant(&element_name).tuple(&format!("Box<{}<'input>>", field_typename));
                            last_item_name = Some(element_name.clone());
                            variants.push((element_name, field_typename));
                        }
                    }
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
                (name.to_string(), enum_name)
            },
            ElementType::List(List::ComplexList(min_occurs, max_occurs, mixed, item_type)) => {
                let struct_name = escape_keyword(name);
                let (_, type_) = self.type_(&format!("{}__valuetype", name), item_type);
                if let Some(type_name) = self.generated_types.get(name) {
                    return (name.to_string(), type_name.clone());
                }
                self.generated_types.insert(name.to_string(), name.to_string());
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(&name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(&format!("Vec<{}<'input>>", type_));
                module.scope().raw(&format!("// ^-- from {:?}", type_tree));
                (name.to_string(), name.to_string())
            },
            ElementType::List(List::SimpleList(item_type)) => {
                let type_name = self.id_to_type_name(*item_type);
                let name = format!("{}_list", name);
                let (_, ref mut module) = self.nsuri_to_module.get_mut(self.target_uri).unwrap();
                module.new_struct(&name).vis("pub").derive("Debug").derive("PartialEq").derive("Default").generic("'input").tuple_field(&format!("Vec<{}<'input>>", type_name));
                (name.clone(), name)
            },
            ElementType::Element(e) => {
                let element_type_name = self.element(e, None);
                (e.name.1.to_string(), element_type_name)
            }
            _ => unimplemented!("{:?}", type_tree),
        }
    }
}
