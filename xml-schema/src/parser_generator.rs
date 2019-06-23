use std::collections::{HashMap, HashSet};

use codegen as cg;
use heck::{SnakeCase, CamelCase};

use support::{ParseContext, Facets};
use primitives::PRIMITIVE_TYPES;
use processor::*;
use names::*;

#[derive(Default)]
pub struct XsdParseContext<'input> {
    namespaces: HashMap<&'input str, &'input str>,
}
impl<'input> ParseContext<'input> for XsdParseContext<'input> {
    fn on_xmlns(&mut self, name: Option<&'input str>, uri: &'input str) {
        match name {
            None => (),
            Some(ns) => { self.namespaces.insert(uri, ns); },
        }
    }
}

const MODULES_FOR_ANONYMOUS_SUBTYPES: &[&'static str] = &["enums", "unions", "inline_elements", "lists"];
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
pub struct ParserGenerator<'ast, 'input: 'ast> {
    processors: Vec<Processor<'ast, 'input>>,
    names: HashMap<Option<&'input str>, (String, HashMap<&'input str, String>)>, // URI -> (module_name, element_name -> struct/enum name)

    renames: HashMap<String, String>,
    name_gen: NameGenerator,
    scope: cg::Scope,
    self_gen: bool,
}

impl<'ast, 'input: 'ast> ParserGenerator<'ast, 'input> {
    pub fn new(processors: Vec<Processor<'ast, 'input>>, parse_context: &XsdParseContext<'input>, renames: HashMap<String, String>) -> ParserGenerator<'ast, 'input> {
        let mut names = HashMap::new();
        names.insert(None, ("unqualified".to_string(), HashMap::new()));
        let mut module_name_gen = NameGenerator::new();

        for &mod_name in MODULES_FOR_ANONYMOUS_SUBTYPES {
            module_name_gen.gen_name(mod_name.to_string()); // Make sure the same names are not reused
        }

        let mut primitive_types = HashMap::new();
        for (name, type_name) in PRIMITIVE_TYPES {
            primitive_types.insert(*name, type_name.to_string());
        }
        names.insert(Some(SCHEMA_URI), (module_name_gen.gen_name("support".to_string()), primitive_types));

        for (&uri, ns) in parse_context.namespaces.iter() {
            if names.get(&Some(uri)).is_none() {
                names.insert(Some(uri), (module_name_gen.gen_name(ns.to_string()), HashMap::new()));
            }
        }

        let mut self_gen = false;
        for proc in &processors {
            if proc.target_namespace == Some(&SCHEMA_URI) {
                self_gen = true;
            }
        }
        ParserGenerator {
            processors, renames, names, self_gen,
            name_gen: NameGenerator::new(),
            scope: cg::Scope::new(),
        }
    }

    pub fn get_module_name(&self, qname: FullName<'input>) -> String {
        self.names.get(&qname.namespace()).expect(&format!("{:?}", qname.namespace())).0.clone()
    }

    pub fn gen_target_scope(&mut self) {
        self.scope.raw("pub use std::collections::HashMap;");
        self.scope.raw("pub use std::marker::PhantomData;");
        self.create_modules();
        self.gen_elements();
        self.gen_inline_elements();
    }

    fn create_modules(&mut self) {
        let mut modules: Vec<_> = self.names.iter().collect();
        modules.sort_by(|(ns1, _), (ns2, _)| ns1.cmp(ns2));

        for submod_name in MODULES_FOR_ANONYMOUS_SUBTYPES {
            let submod_name = self.name_gen.gen_name(submod_name.to_string()); // This will always return the name as-is; we're only doing this to avoid duplicates
            let submod = self.scope.new_module(&submod_name);
            submod.vis("pub");
            submod.scope().raw("#[allow(unused_imports)]\nuse super::*;");
        }

        for (&uri, (mod_name, _type_names)) in modules {
            if !self.self_gen && uri == Some(SCHEMA_URI) {
                self.scope.raw(&format!("#[allow(unused_imports)]\npub use xml_schema::parser::xs as {};", mod_name));
            }
            else {
                let mut module = self.scope.new_module(mod_name);
                module.vis("pub");
                module.scope().raw(&format!("//! {:?}", uri));
                module.scope().raw("#[allow(unused_imports)]\nuse super::*;");
            }
        }
    }

    fn gen_complex_type(&mut self, node: &RichAstNode<'input, Type<'input>, ()>) -> Option<(String, String)> {
        let RichAstNode { attrs, type_, doc, _data } = node;
        match type_ {
            Type::Choice(items) => self.gen_choice(items, doc),
            Type::Sequence(items) => self.gen_group_or_sequence(items, doc),
            _ => unimplemented!("{:?}", type_),
        }
    }

    fn gen_choice(&mut self, items: &Vec<RichAstNode<'input, Type<'input>, ()>>, doc: &Documentation<'input>) {
        let items: Vec<_> = items.iter().filter_map(|item| self.gen_complex_type(item)).collect();
        let mut name_hint = NameHint::new("choice");
        for item in items {
            name_hint.extend(item.1);
        }
        let name = name_from_hint(name_hint).unwrap();
        let enum_name = escape_keyword(&name.to_camel_case());
        let enum_name = self.name_gen.gen_name(enum_name);
        let enum_name = self.renames.get(enum_name).unwrap_or(enum_name);

        let module = self.scope.get_module_mut("choices").unwrap();
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_enum!({},", enum_name));
        {
            let enum_ = module.scope().new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (i, item) in items.iter().enumerate() {
                let mut fields = Vec::new();
                let mut doc = doc.clone();
                {
                    let mut name_gen = NameGenerator::new();
                    let field_writer = &mut |field_name: String, type_mod_name: String, min_occurs, max_occurs, type_name: String| {
                        let field_name = escape_keyword(&name_gen.gen_name(field_name.to_snake_case()));
                        let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                        let type_name = escape_keyword(&type_name.to_camel_case());
                        fields.push((field_name, type_mod_name, min_occurs, max_occurs, type_name));
                    };
                    let doc_writer = &mut |doc2: &Documentation<'input>| doc.extend(doc2);
                    self.write_type_in_struct_def(field_writer, &mut Some(doc_writer), &item.type_);
                }
                enum_.doc(&doc.to_string());
                let variant_name = item.1;
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
        module.scope().raw(&impl_code.join("\n"));
    }
    fn gen_union(&self, items: &Vec<RichAstNode<'input, Type<'input>, ()>>, doc: &Documentation<'input>) {
        let items: Vec<_> = items.iter().filter_map(|item| self.gen_simple_type(item)).collect();

        let mut name_hint = NameHint::new("union");
        for item in items {
            name_hint.extend(item.1);
        }
        let name = name_from_hint(name_hint);
        let enum_name = escape_keyword(&name.to_camel_case());
        let enum_name = self.name_gen.gen_name(enum_name);
        let enum_name = self.renames.get(enum_name).unwrap_or(enum_name);

        let module = self.scope.module_get_mut("unions").unwrap();
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_union!({}, {{", enum_name));
        {
            let enum_ = module.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            if items.len() == 0 {
                panic!("Union of the empty set.") // XXX Should we just return a None instead?
            }
            else if items.len() == 1 {
                // Shortcut if this is a union for a single type
                return items.get(0)
            }
            else {
                let mut name_gen = NameGenerator::new();
                for (type_mod_name, type_name) in items {
                    let variant_name = name_gen.gen_name(type_name.to_camel_case());
                    enum_.new_variant(&variant_name).tuple(&format!("{}::{}<'input>", type_mod_name, type_name));
                    impl_code.push(format!("    impl_union_variant!({}),", variant_name));
                }
            }
        }
        impl_code.push(format!("}});"));
        module.scope().raw(&impl_code.join("\n"));
    }

    fn gen_list(&self, item: &RichAstNode<'input, SimpleType<'input>, ()>, doc: &Documentation<'input>) -> Option<(String, String)> {
        if let Some((type_mod_name, type_name)) = self.gen_simple_type(item) {
            let mut name = NameHint::new("list");
            name.extend(type_name.to_camel_case());
            let name = escape_keyword(&name);
            let struct_name = self.name_gen.gen_name(name);
            let module = self.scope.get_module_mut("structs").unwrap();
            let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            struct_.tuple_field(&format!("pub Vec<{}::{}<'input>>", type_mod_name, type_name));
            module.scope().raw(&format!("impl_list!({}, {}::{});", struct_name, type_mod_name, type_name));
            Some(("lists", struct_name))
        }
        else {
            None
        }
    }

    fn gen_simple_type(&self, ty: &RichAstNode<'input, SimpleType<'input>, ()>) -> Option<(String, String)> {
        let RichAstNode { attrs, type_, doc, data } = ty;
        if attrs.named.len() != 0 || attrs.refs.len() != 0 || attrs.group_refs.len() != 0 || attrs.any_attributes {
            // RichAstNode should be changed so impossibility is enforced by the type checker.
            panic!("Simple type with attributes???");
        }
        let (type_mod_name, type_name) = match type_ {
            SimpleType::Alias(name) => {
                let (type_mod_name, local_names) = self.names.get(ty.namespace())
                    .expect(&format!("Unknown namespace: {}", ty.namespace()));
                let type_name = local_names.get(ty.local_name())
                    .expect(&format!("Unknown type {} in namespace {}", ty.local_name(), ty.namespace()));
                (type_mod_name, type_name)
            },
            SimpleType::Restriction(name, facets) => self.gen_restriction(name, facets),
            SimpleType::Union(items) => self.gen_union(items),
            SimpleType::List(item) => self.gen_list(item),
            SimpleType::Empty => {
                return None
            },
        };
        Some((type_mod_name, type_name))
    }

    fn gen_simple_restriction(&mut self, base_name: FullName<'input>, facets: &Facets<'input>) {
        let name = match &facets.enumeration {
            Some(items) => {
                if items.len() == 1 {
                    format!("{}", items[0])
                }
                else {
                    format!("enumeration_{}", items.join("_"))
                }
            },
            None => format!("Restrict_{}", base_name.local_name()),
        };
        let name = name.to_camel_case();
        let name = self.name_gen.gen_name(name.clone());
        self.simple_restrictions.insert((base_name.clone(), facets.clone()), name.clone());
        let (base_mod_name, base_type_name) = self.get_simple_type_name(&SimpleType::Alias(*base_name)).unwrap(); // TODO
        let mut module = self.scope.get_module_mut("enumerations").unwrap();
        module.scope().raw(&format!("#[derive(Debug, PartialEq)] pub struct {}<'input>(pub {}::{}<'input>);", name, base_mod_name, base_type_name));
        let mut s = Vec::new();
        let f = &mut |n: &Option<_>| {
            match n.as_ref() {
                None => "None".to_string(),
                Some(f) => format!("Some(BigFloatNotNaN::from_str(\"{}\").unwrap())", f),
            }
        };
        s.push(format!("min_exclusive: {},", f(&facets.min_exclusive)));
        s.push(format!("min_inclusive: {},", f(&facets.min_inclusive)));
        s.push(format!("max_exclusive: {},", f(&facets.max_exclusive)));
        s.push(format!("max_inclusive: {},", f(&facets.max_inclusive)));
        s.push(format!("total_digits: {:?},", facets.total_digits));
        s.push(format!("fraction_digits: {:?},", facets.fraction_digits));
        s.push(format!("length: {:?},", facets.length));
        s.push(format!("min_length: {:?},", facets.min_length));
        s.push(format!("max_length: {:?},", facets.max_length));
        match &facets.enumeration {
            Some(items) => s.push(format!("enumeration: Some(vec![{}]),", items.iter().map(|i| format!("{:?}", i)).collect::<Vec<_>>().join(", "))),
            None => s.push("enumeration: None,".to_string()),
        }
        s.push(format!("white_space: {:?},", facets.white_space));
        s.push(format!("pattern: {:?},", facets.pattern));
        s.push(format!("assertion: {:?},", facets.assertion));
        s.push(format!("explicit_timezone: {:?},", facets.explicit_timezone));
        module.scope().raw(&format!("impl_simpletype_restriction!({}, Facets {{\n    {}\n}});", name, s.join("\n    ")));
    }

    fn gen_fields(&self, empty_struct: &mut bool, struct_: &mut cg::Struct, impl_code: &mut Vec<String>, doc: &mut Documentation<'input>, name_gen: &mut NameGenerator, type_: &Type<'input>) {
        let field_writer = &mut |name: String, type_mod_name: String, min_occurs, max_occurs, type_name: String| {
            *empty_struct = false;
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
                (_, ::std::usize::MAX) => {
                    struct_.field(&format!("pub {}", name), &format!("Vec<super::{}::{}<'input>>", type_mod_name, type_name));
                    impl_code.push(format!("    ({}, {}, Vec<{}; min={};>),", name, type_mod_name, type_name, min_occurs))
                },
                (_, _) => {
                    struct_.field(&format!("pub {}", name), &format!("Vec<super::{}::{}<'input>>", type_mod_name, type_name));
                    impl_code.push(format!("    ({}, {}, Vec<{}; min={}; max={};>),", name, type_mod_name, type_name, min_occurs, max_occurs))
                },
            }
        };
        let doc_writer = &mut |doc2| doc.extend(doc2);
        self.write_type_in_struct_def(field_writer, &mut Some(doc_writer), &type_);
    }

    fn gen_sequence(&mut self, items: &Vec<&RichAstNode<'input, Type<'input>, ()>>, doc: &Documentation<'input>) {
        self.gen_group_or_sequence("sequence", NameHint::new("sequence"), items, doc)
    }
    fn gen_group(&mut self, items: &Vec<&RichAstNode<'input, Type<'input>, ()>>, doc: &Documentation<'input>) {
        self.gen_group_or_sequence("group", NameHint::new("group"), items, doc)
    }
    fn gen_group_or_sequence(&mut self, mod_name: &str, name_hint: &NameHint<'input>, items: &Vec<&RichAstNode<'input, Type<'input>, ()>>, doc: &Documentation<'input>) {
        let items: Vec<_> = items.iter().filter_map(|item| self.gen_simple_type(item)).collect();

        for item in items {
            name_hint.extend(item.1);
        }
        let struct_name = name_from_hint(name_hint);
        let struct_name = escape_keyword(&struct_name.to_camel_case());
        let struct_name = self.name_gen.gen_name(struct_name);
        let struct_name = self.renames.get(&struct_name).unwrap_or(&struct_name);

        let mut module = self.scope.module_get_mut(mod_name).unwrap();
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_group_or_sequence!({},", struct_name));
        {
            let mut empty_struct = true;
            let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            let mut name_gen = NameGenerator::new();
            let mut doc = doc.clone();
            {
                for item in items {
                    self.gen_fields(&mut empty_struct, struct_, &mut impl_code, &mut doc, &mut name_gen, &item.type_);
                }
            }
            struct_.doc(&doc.to_string());
            if empty_struct {
                struct_.tuple_field(&format!("pub ::std::marker::PhantomData<&'input ()>"));
            }
        }
        impl_code.push(");".to_string());
        module.scope().raw(&impl_code.join("\n"));
    }

    fn gen_inline_elements(&mut self) {
        let module = self.scope.new_module("inline_elements");
        module.vis("pub");
        module.scope().raw("#[allow(unused_imports)]\nuse super::*;");
        for proc in &self.processors {
            let mut elements: Vec<_> = proc.inline_elements.iter().collect();

            elements.sort_by_key(|&((ns,n,_,_),(n2,_))| (ns, n, n2.iter().collect::<Vec<_>>()));
            for ((namespace, tag_name, attrs, element), (struct_names, doc)) in elements {
                // struct_names is always non-empty.

                let mut struct_names: Vec<_> = struct_names.iter().map(|s| s.to_camel_case()).collect();

                // Sort them to get the shortest one, and alias all the others to this one
                struct_names.sort_by_key(|n| n.len()); // TODO: just use a min_by_key
                for alias in &struct_names[1..] {
                    module.scope().raw(&format!("pub type {}<'input> = {}<'input>;", alias, struct_names[0]));
                }

                let tag_name = FullName::new(*namespace, tag_name);
                self.gen_element(module, &struct_names[0], &tag_name, attrs, element, doc);
            }
        }
    }

    fn gen_elements(&mut self) {
        for proc in &self.processors {
            let mut elements: Vec<_> = proc.elements.iter().collect();

            elements.sort_by_key(|&(n,_)| n);
            for (&name, element) in elements {
                let mod_name = self.get_module_name(name);
                let mut module = self.scope.get_module_mut(&mod_name).expect(&mod_name);
                let head_local_name = format!("{}_head", name.local_name());
                let mut substitutions = Vec::new();
                substitutions.push(FullName::new(name.namespace(), &head_local_name));
                for proc in &self.processors {
                    if let Some(members) = proc.substitution_groups.get(&name) {
                        substitutions.extend(members);
                    }
                }
                if substitutions.len() > 1 {
                    let enum_name = escape_keyword(&name.local_name().to_camel_case());
                    self.gen_substitution_enum(module.scope(), &enum_name, &substitutions);
                    let struct_name = escape_keyword(&head_local_name.to_camel_case());
                    self.gen_element(module, &struct_name, &name, &element.attrs, &element.type_, &element.doc);
                }
                else {
                    let struct_name = escape_keyword(&name.local_name().to_camel_case());
                    self.gen_element(module, &struct_name, &name, &element.attrs, &element.type_, &element.doc);
                }
            }
        }
    }

    fn gen_substitution_enum(&self, enum_name: &str, substitutions: &Vec<FullName<'input>>) {
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_enum!({},", enum_name));
        let mut name_gen = NameGenerator::new();
        {
            let enum_ = self.scope.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for &substitution in substitutions {
                let variant_name = escape_keyword(&name_gen.gen_name(substitution.local_name().to_camel_case()));
                let type_mod_name = escape_keyword(&self.get_module_name(substitution).to_snake_case());
                let type_name = escape_keyword(&substitution.local_name().to_camel_case());
                let mut variant = enum_.new_variant(&variant_name);
                variant.tuple(&format!("Box<super::{}::{}<'input>>", escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                impl_code.push(format!("    impl_singleton_variant!({}, {}, Box<{}>),", variant_name, escape_keyword(&type_mod_name), escape_keyword(&type_name)));
            }
            impl_code.push(");".to_string());
        }
        self.scope.raw(&impl_code.join("\n"));
    }

    fn gen_attrs(&self, struct_: &mut cg::Struct, impl_code: &mut Vec<String>, name_gen: &mut NameGenerator, attrs: &Attrs<'input>, seen_attrs: &mut HashMap<FullName<'input>, AttrUse>, generated_attrs: &mut HashSet<FullName<'input>>, inherited: bool) {
        for (attr_name, use_, attr_type) in &attrs.named {
            if generated_attrs.contains(attr_name) {
                continue;
            }
            let default_type = SimpleType::Primitive(SCHEMA_URI, "AnySimpleType");
            let type_ = attr_type.as_ref().unwrap_or(&default_type);
            let (type_mod_name, type_name) = self.get_simple_type_name(&type_).unwrap();
            let use_ = if inherited {
                *seen_attrs.get(attr_name).unwrap_or(use_)
            }
            else {
                *use_
            };
            seen_attrs.insert(attr_name.clone(), use_);
            generated_attrs.insert(attr_name.clone());
            match use_ {
                AttrUse::Optional => {
                    let field_name = self.name_gen.gen_name(format!("attr_{}", attr_name.local_name()).to_snake_case());
                    struct_.field(&format!("pub {}", field_name), &format!("Option<{}::{}<'input>>", type_mod_name, type_name));
                    impl_code.push(format!("    ({:?}, {:?}) => {}: optional,", attr_name.namespace().unwrap_or(""), attr_name.local_name(), field_name));
                },
                AttrUse::Required => {
                    let field_name = self.name_gen.gen_name(format!("attr_{}", attr_name.local_name()).to_snake_case());
                    struct_.field(&format!("pub {}", field_name), &format!("{}::{}<'input>", type_mod_name, type_name));
                    impl_code.push(format!("    ({:?}, {:?}) => {}: required,", attr_name.namespace().unwrap_or(""), attr_name.local_name(), field_name));
                },
                AttrUse::Prohibited => (),
            }
        }
        for group_name in &attrs.group_refs {
            let mut found = false;
            for processor in self.processors.iter() {
                if let Some(attrs) = processor.attribute_groups.get(group_name) {
                    self.gen_attrs(struct_, impl_code, name_gen, attrs, seen_attrs, generated_attrs, false);
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("unknown attribute group: {:?}", group_name);
            }
        }
    }

    fn gen_element(&self, module: &mut cg::Module, struct_name: &str, tag_name: &FullName<'input>, attrs: &Attrs<'input>, type_: &Type<'input>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_element!({}, {:?}, \"{}\", attributes = {{",
            struct_name, tag_name.namespace().unwrap_or(""), tag_name.local_name()));
        {
            let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            let mut empty_struct = false;
            struct_.field("pub attrs", "HashMap<FullName<'input>, &'input str>");
            let mut name_gen = NameGenerator::new();
            let mut doc = doc.clone();
            let mut generated_attrs = HashSet::new();
            let mut seen_attrs = HashMap::new();
            
            let attrs = self.compute_attrs(&type_, attrs);
            self.gen_attrs(struct_, &mut impl_code, &mut name_gen, &attrs, &mut seen_attrs, &mut generated_attrs, false);
            {
                let field_writer = &mut |_, _, _, _, _| ();
                let doc_writer = &mut |_| ();
                self.write_type_in_struct_def(field_writer, &mut Some(doc_writer), &type_);
            }
            impl_code.push(format!("}}, fields = {{"));
            self.gen_fields(&mut empty_struct, struct_, &mut impl_code, &mut doc, &mut name_gen, type_);
            struct_.doc(&doc.to_string());
        }
        impl_code.push(format!("}});"));
        module.scope().raw(&impl_code.join("\n"));
    }

    fn get_type_generated_name(&self, name: &FullName<'input>) -> (&str, str) {
        if name.namespace() == Some(SCHEMA_URI) {
            if let Some(name) = self.primitive_types.get(name.local_name()) {
                return name;
            }
        }
        let mut type_ = None;
        for proc in &self.processors {
            if proc.target_namespace != name.namespace() {
                continue;
            }
            type_ = proc.types.get(name);
            if type_.is_some() {
                break;
            }
        }
        let type_ = type_.expect(&format!("Unknown type name: {:?}", name));
        type_
    }

    fn write_type_in_struct_def<'a, F, H>(&'a self,
            field_writer: &mut F,
            doc_writer: &mut Option<&mut H>,
            type_: &'a Type<'input>,
            ) where
            F: FnMut(String, String, usize, usize, String),
            H: FnMut(&'a Documentation<'input>),
            'ast: 'a {
        let mut doc_non_writer: Option<&mut H> = None;
        match &type_ {
            Type::Alias(name) => {
                let target_type = self.get_type(name);
                if let Some(ref mut f) = doc_writer {
                    f(&target_type.doc);
                }
                self.write_type_in_struct_def(field_writer, doc_writer, &target_type.type_);
            },
            Type::InlineSequence(items) => {
                for item in items {
                    self.write_type_in_struct_def(field_writer, &mut doc_non_writer, &item.type_);
                }
            }
            Type::Sequence(min_occurs, max_occurs, name) => {
                field_writer(name.to_string(), "sequences".to_string(), *min_occurs, *max_occurs, name.to_string());
            }
            Type::Element(min_occurs, max_occurs, name) => {
                field_writer(name.to_string(), "inline_elements".to_string(), *min_occurs, *max_occurs, name.to_string());
            }
            Type::Group(min_occurs, max_occurs, name) |
            Type::ElementRef(min_occurs, max_occurs, name) => {
                let field_name = name.local_name();
                let mod_name = self.get_module_name(*name);
                field_writer(field_name.to_string(), mod_name.to_string(), *min_occurs, *max_occurs, name.local_name().to_string());
            },
            Type::Choice(min_occurs, max_occurs, ref name) => {
                field_writer(name.to_string(), "enums".to_string(), *min_occurs, *max_occurs, name.to_string());
            },
            Type::Extension(base, ext_type) => {
                let base_type = &self.get_type(base);
                self.write_type_in_struct_def(field_writer, &mut doc_non_writer, &base_type.type_);
                self.write_type_in_struct_def(field_writer, doc_writer, &ext_type.type_);
            },
            Type::Restriction(base, ext_type) => {
                if let Some(ref mut f) = doc_writer {
                    f(&ext_type.doc);
                }
                let base_type = &self.get_type(base);
                // TODO: do something with the base's type
                self.write_type_in_struct_def(field_writer, doc_writer, &ext_type.type_);
            },
            Type::Empty => (), // TODO ?
            Type::Any => {
                field_writer("any".to_string(), "support".to_string(), 1, 1, "Any".to_string())
            },
            Type::Simple(type_) => {
                let (type_mod_name, type_name) = self.get_simple_type_name(&type_).unwrap();
                field_writer(type_name.clone(), type_mod_name, 1, 1, type_name);
            }
            _ => unimplemented!("writing {:?}", type_),
        }
    }

    fn compute_attrs(&self, type_: &Type<'input>, own_attrs: &Attrs<'input>) -> Attrs<'input> {
        match type_ {
            Type::Alias(name) => {
                let target_type = self.get_type(name);
                let target_attrs = self.compute_attrs(&target_type.type_, &target_type.attrs);
                self.extend_attrs(&target_attrs, own_attrs)
            },
            Type::InlineSequence(_) |
            Type::Sequence(_, _, _) |
            Type::Element(_, _, _) |
            Type::Group(_, _, _) |
            Type::ElementRef(_, _, _) |
            Type::Choice(_, _, _) |
            Type::Empty |
            Type::Any => {
                own_attrs.clone()
            }
            Type::Extension(base, ext_type) => {
                let base_type = &self.get_type(&base);
                let base_attrs = self.compute_attrs(&base_type.type_, &base_type.attrs);
                let own_attrs = self.extend_attrs(own_attrs, &ext_type.attrs); // XXX
                self.extend_attrs(&base_attrs, &own_attrs)
            },
            Type::Restriction(base, ext_type) => {
                let base_type = &self.get_type(&base);
                let base_attrs = self.compute_attrs(&base_type.type_, &base_type.attrs);
                let own_attrs = self.extend_attrs(own_attrs, &ext_type.attrs); // XXX
                self.restrict_attrs(&base_attrs, &own_attrs)
            },
            Type::Simple(type2) => {
                own_attrs.clone()
            }
            _ => unimplemented!("writing {:?}", type_),
        }
    }

    pub fn extend_attrs(&self, base: &Attrs<'input>, other: &Attrs<'input>) -> Attrs<'input> {
        let mut other_named = HashMap::new();
        for (name, attr_use, type_) in other.named.iter() {
            other_named.insert(name.clone(), (*attr_use, type_));
        }
        let mut seen = HashSet::new();
        let mut named: Vec<_> = base.named.iter().map(|(name, attr_use, type_)| {
            seen.insert(name);
            match other_named.get(name) {
                None => (name.clone(), *attr_use, (*type_).clone()),
                Some((attr_use, type_)) => (name.clone(), *attr_use, (*type_).clone()),
            }
        }).collect();

        let mut other_refs = HashMap::new();
        for (name, attr_use, ref_) in other.refs.iter() {
            other_refs.insert((name.clone(), ref_.clone()), *attr_use);
        }
        let mut refs: Vec<_> = base.refs.iter().map(|(name, attr_use, ref_)| {
            if let Some(name) = name {
                seen.insert(name);
            }
            match other_refs.get(&(*name, *ref_)) {
                None => (name.clone(), *attr_use, (*ref_).clone()),
                Some(attr_use) => (name.clone(), *attr_use, (*ref_).clone()),
            }
        }).collect();

        for (name, attr_use, type_) in other.named.iter() {
            if !seen.contains(name) {
                named.push((name.clone(), *attr_use, type_.clone()));
            }
        }
        for (name, attr_use, ref_) in other.refs.iter() {
            match name {
                Some(name) => {
                    if !seen.contains(name) {
                        refs.push((Some(name.clone()), *attr_use, ref_.clone()));
                    }
                },
                None => (), // TODO
            }
        }

        let mut group_refs = base.group_refs.clone();
        let seen_refs: HashSet<_> = base.group_refs.iter().collect();
        for group_ref in other.group_refs.iter() {
            if !seen_refs.contains(group_ref) {
                group_refs.push(*group_ref);
            }
        }

        let res = Attrs { named, refs, group_refs, any_attributes: other.any_attributes };
        res
    }

    pub fn restrict_attrs(&self, base: &Attrs<'input>, other: &Attrs<'input>) -> Attrs<'input> {
        self.extend_attrs(base, other) // XXX
    }
}
