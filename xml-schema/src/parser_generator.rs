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
    pub scope: cg::Scope,
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
        //self.gen_inline_elements();
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
        let RichAstNode { attrs, type_, doc, data: _ } = node;
        match type_ {
            Type::Choice(items) => self.gen_choice(items, doc),
            Type::Sequence(items) => self.gen_sequence(items, doc),
            Type::Alias(name) => Some(self.get_type_name(*name)),
            _ => unimplemented!("{:?}", type_),
        }
    }

    fn gen_choice(&mut self, items: &Vec<Vec<InlineComplexType<'input, ()>>>, doc: &Documentation<'input>) -> Option<(String, String)> {
        let items: Vec<Vec<_>> = items.iter().map(
            |variants| variants.iter().filter_map(
                |InlineComplexType { min_occurs, max_occurs, type_ }|
                self.gen_complex_type(type_).map(|item| (min_occurs, max_occurs, item))
            ).collect()
        ).collect();
        let mut name_hint = NameHint::new("choice");
        for variant in items.iter() {
            for (_min_occurs, _max_occurs, field) in variant.iter() {
                name_hint.push(&field.1);
            }
        }
        let name = name_from_hint(&name_hint).unwrap();
        let enum_name = escape_keyword(&name.to_camel_case());
        let enum_name = self.name_gen.gen_name(enum_name);
        let enum_name = self.renames.get(&enum_name).unwrap_or(&enum_name);

        let mut variant_name_gen = NameGenerator::new();

        let module = self.scope.get_module_mut("choices").unwrap();
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_enum!({},", enum_name));
        {
            let enum_ = module.scope().new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (i, fields) in items.iter().enumerate() {
                // TODO: handle min_occurs and max_occurs
                let mut doc = doc.clone();
                let mut variant_name = NameHint::new_empty();
                for (_min_occurs, _max_occurs, (_mod_name, field_type_name)) in fields.iter() {
                    variant_name.push(&field_type_name);
                }
                let variant_name = name_from_hint(&variant_name).unwrap(); // TODO: handle fields.len()==0
                let variant_name = escape_keyword(&variant_name.to_camel_case());
                let variant_name = variant_name_gen.gen_name(variant_name);
                let variant_name = self.renames.get(&variant_name).unwrap_or(&variant_name);
                // TODO set the doc on the variant instead of the enum
                let mut variant = enum_.new_variant(&variant_name);
                if fields.len() == 1 {
                    let (min_occurs, max_occurs, (type_mod_name, type_name)) = fields.get(0).unwrap();
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
                    let mut field_name_gen = NameGenerator::new();
                    for (min_occurs, max_occurs, (type_mod_name, type_name)) in fields.iter() {
                        let field_name = NameHint::new(type_name);
                        let field_name = name_from_hint(&field_name).unwrap();
                        let field_name = field_name.to_snake_case();
                        let field_name = field_name_gen.gen_name(field_name);
                        let field_name = self.renames.get(&field_name).unwrap_or(&field_name);
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
        Some(("choices".to_owned(), enum_name.to_owned()))
    }
    fn gen_union(&mut self, items: &Vec<InlineSimpleType<'input, ()>>, doc: &Documentation<'input>) -> Option<(String, String)>{
        let items: Vec<_> = items.iter().filter_map(
            |InlineSimpleType { type_ }| self.gen_simple_type(type_)
        ).collect();

        let mut name_hint = NameHint::new("union");
        for item in items.iter() {
            name_hint.push(&item.1);
        }
        let name = name_from_hint(&name_hint).unwrap();
        let enum_name = escape_keyword(&name.to_camel_case());
        let enum_name = self.name_gen.gen_name(enum_name);
        let enum_name = self.renames.get(&enum_name).unwrap_or(&enum_name);

        let module = self.scope.get_module_mut("unions").unwrap();
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_union!({}, {{", enum_name));
        {
            let enum_ = module.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            if items.len() == 0 {
                panic!("Union of the empty set.") // XXX Should we just return a None instead?
            }
            else if items.len() == 1 {
                // Shortcut if this is a union for a single type
                return items.get(0).cloned()
            }
            else {
                let mut name_gen = NameGenerator::new();
                for (type_mod_name, type_name) in items.iter() {
                    let variant_name = name_gen.gen_name(type_name.to_camel_case());
                    enum_.new_variant(&variant_name).tuple(&format!("{}::{}<'input>", type_mod_name, type_name));
                    impl_code.push(format!("    impl_union_variant!({}),", variant_name));
                }
            }
        }
        impl_code.push(format!("}});"));
        module.scope().raw(&impl_code.join("\n"));
        Some(("union".to_owned(), enum_name.clone()))
    }

    fn gen_list(&mut self, item: &InlineSimpleType<'input, ()>, doc: &Documentation<'input>) -> Option<(String, String)> {
        let InlineSimpleType { type_ } = item;
        if let Some((type_mod_name, type_name)) = self.gen_simple_type(type_) {
            let mut name = NameHint::new("list");
            name.push(&type_name);
            let name = name_from_hint(&name).unwrap().to_camel_case();
            let name = escape_keyword(&name);
            let struct_name = self.name_gen.gen_name(name);
            let module = self.scope.get_module_mut("structs").unwrap();
            let mut struct_ = cg::Struct::new(&struct_name);
            struct_.vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            struct_.tuple_field(&format!("pub Vec<{}::{}<'input>>", type_mod_name, type_name));
            module.push_struct(struct_);
            module.scope().raw(&format!("impl_list!({}, {}::{});", struct_name, type_mod_name, type_name));
            Some(("lists".to_owned(), struct_name))
        }
        else {
            None
        }
    }

    fn get_type(&self, name: FullName<'input>) -> Option<&RichAstNode<'input, Type<'input>, ()>> {
        for proc in self.processors.iter() {
            if let Some(type_) = proc.complex_types.get(&name) {
                return Some(type_)
            }
        }
        None
    }

    fn get_type_name(&self, name: FullName<'input>) -> (String, String) {
        let (mod_name, ns_names) = self.names.get(&name.namespace()).expect("Unknown namespace");
        let type_name = ns_names.get(&name.local_name()).expect(&format!("Unknown simple type name {:?}", name));
        (mod_name.clone(), type_name.clone())
    }

    fn gen_simple_type(&mut self, ty: &RichAstNode<'input, SimpleType<'input>, ()>) -> Option<(String, String)> {
        let RichAstNode { attrs, type_, doc, data } = ty;
        if attrs.named.len() != 0 || attrs.refs.len() != 0 || attrs.group_refs.len() != 0 || attrs.any_attributes {
            // RichAstNode should be changed so impossibility is enforced by the type checker.
            panic!("Simple type with attributes???");
        }
        match type_ {
            SimpleType::Alias(name) => Some(self.get_type_name(*name)),
            SimpleType::Restriction(name, facets) => self.gen_simple_restriction(*name, facets, doc),
            SimpleType::Union(items) => self.gen_union(items, doc),
            SimpleType::List(item) => self.gen_list(item, doc),
            SimpleType::Empty => None,
        }
    }

    fn gen_simple_restriction(&mut self, base_name: FullName<'input>, facets: &Facets<'input>, doc: &Documentation<'input>) -> Option<(String, String)> {
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
        let (base_mod_name, base_type_name) = self.get_type_name(base_name);
        let module = self.scope.get_module_mut("enumerations").unwrap();
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
        Some(("enumeration".to_owned(), name))
    }

    fn gen_field(&self, empty_struct: &mut bool, struct_: &mut cg::Struct, impl_code: &mut Vec<String>, doc: &Documentation<'input>, name_gen: &mut NameGenerator, min_occurs: usize, max_occurs: usize, type_name: (String, String)) {
        let (type_mod_name, type_name) = type_name;
        *empty_struct = false;
        let name = escape_keyword(&name_gen.gen_name(type_name.to_snake_case()));
        let name = self.renames.get(&name).unwrap_or(&name);
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
    }

    fn gen_sequence(&mut self, items: &Vec<InlineComplexType<'input, ()>>, doc: &Documentation<'input>)-> Option<(String, String)> {
        self.gen_group_or_sequence("sequence", "sequence", items, doc)
    }
    fn gen_group(&mut self, items: &Vec<InlineComplexType<'input, ()>>, doc: &Documentation<'input>) -> Option<(String, String)> {
        self.gen_group_or_sequence("group", "group", items, doc)
    }
    fn gen_group_or_sequence(&mut self, mod_name: &str, name_prefix: &str, items: &Vec<InlineComplexType<'input, ()>>, doc: &Documentation<'input>) -> Option<(String, String)> {
        let items: Vec<_> = items.iter().filter_map(
            |InlineComplexType { min_occurs, max_occurs, ref type_ }|
            self.gen_complex_type(type_).map(|type_name| (*min_occurs, *max_occurs, type_name))
        ).collect();
        let struct_name = {
            let mut name_hint = NameHint::new(name_prefix);
            for (_min_occurs, _max_occurs, type_name) in items.iter() {
                name_hint.push(&type_name.1);
            }
            let struct_name = name_from_hint(&name_hint).unwrap();
            let struct_name = escape_keyword(&struct_name.to_camel_case());
            struct_name
        };
        let struct_name = self.name_gen.gen_name(struct_name);
        let struct_name = self.renames.get(&struct_name).unwrap_or(&struct_name);

        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_group_or_sequence!({},", struct_name));

        let mut empty_struct = true;
        let mut struct_ = cg::Struct::new(&struct_name);
        struct_.vis("pub").derive("Debug").derive("PartialEq").generic("'input");
        let mut name_gen = NameGenerator::new();
        let mut doc = doc.clone();
        {
            for (min_occurs, max_occurs, type_name) in items.into_iter() {
                self.gen_field(&mut empty_struct, &mut struct_, &mut impl_code, &mut doc, &mut name_gen, min_occurs, max_occurs, type_name);
            }
        }
        struct_.doc(&doc.to_string());
        if empty_struct {
            // TODO: return None instead
            struct_.tuple_field(&format!("pub ::std::marker::PhantomData<&'input ()>"));
        }
        impl_code.push(");".to_string());

        let module = self.scope.get_module_mut(mod_name).unwrap();
        module.push_struct(struct_);
        module.scope().raw(&impl_code.join("\n"));
        Some((mod_name.to_owned(), struct_name.to_owned()))
    }

    /*
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
    }*/

    fn gen_elements(&mut self) {
        let elements: Vec<(FullName<'input>, InlineComplexType<'input, ()>)> = self.processors.iter().flat_map(
            |proc| proc.elements.iter().map(|(name, type_)| (*name, type_.clone()))
        ).collect();
        let mut elements = elements.clone(); // TODO: do not clone

        elements.sort_by_key(|&(n,_)| n);

        for (name, element) in elements {
            self.gen_element_with_substitution(name, &element);
        }
    }

    fn gen_element_with_substitution(&mut self, name: FullName<'input>, element: &InlineComplexType<'input, ()>) {
        let mut substitutions = Vec::new();
        substitutions.push(name);
        for proc in self.processors.iter() {
            if let Some(members) = proc.substitution_groups.get(&name) {
                substitutions.extend(members);
            }
        }

        if substitutions.len() > 1 {
            let enum_name = escape_keyword(&name.local_name().to_camel_case());
            self.gen_substitution_enum(&enum_name, &substitutions);
            let head_local_name = format!("{}_head", name.local_name());
            let struct_name = escape_keyword(&head_local_name.to_camel_case());
            self.gen_element(&struct_name, name, element);
        }
        else {
            let struct_name = escape_keyword(&name.local_name().to_camel_case());
            self.gen_element(&struct_name, name, element);
        }
    }

    fn gen_substitution_enum(&mut self, enum_name: &str, substitutions: &Vec<FullName<'input>>) {
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_enum!({},", enum_name));
        let mut name_gen = NameGenerator::new();
        {
            let mut enum_ = cg::Enum::new(&enum_name);
            enum_.vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for &substitution in substitutions {
                let variant_name = escape_keyword(&name_gen.gen_name(substitution.local_name().to_camel_case()));
                let type_mod_name = escape_keyword(&self.get_module_name(substitution).to_snake_case());
                let type_name = escape_keyword(&substitution.local_name().to_camel_case());
                let mut variant = enum_.new_variant(&variant_name);
                variant.tuple(&format!("Box<super::{}::{}<'input>>", escape_keyword(&type_mod_name), escape_keyword(&type_name)));
                impl_code.push(format!("    impl_singleton_variant!({}, {}, Box<{}>),", variant_name, escape_keyword(&type_mod_name), escape_keyword(&type_name)));
            }
            impl_code.push(");".to_string());
            self.scope.push_enum(enum_);
        }
        self.scope.raw(&impl_code.join("\n"));
    }

    fn gen_attrs(&mut self, struct_: &mut cg::Struct, impl_code: &mut Vec<String>, name_gen: &mut NameGenerator, attrs: &Attrs<'input>, seen_attrs: &mut HashMap<FullName<'input>, AttrUse>, generated_attrs: &mut HashSet<FullName<'input>>, inherited: bool) {
        for (attr_name, use_, attr_type) in &attrs.named {
            if generated_attrs.contains(attr_name) {
                continue;
            }
            let default_type = SimpleType::Alias(FullName::new(Some(SCHEMA_URI), "AnySimpleType"));
            let type_ = attr_type.as_ref().unwrap_or(&default_type);
            let (type_mod_name, type_name) = self.gen_simple_type(&RichAstNode::new(type_.clone(), Documentation::new())).unwrap(); // FIXME: ugly
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
            let mut found = None;
            for processor in self.processors.iter() {
                if let Some(attrs) = processor.attribute_groups.get(group_name) {
                    found = Some(attrs.clone()); // TODO: do not clone
                    break;
                }
            }
            if let Some(attrs) = found {
                self.gen_attrs(struct_, impl_code, name_gen, &attrs, seen_attrs, generated_attrs, false);
            }
            else {
                panic!("unknown attribute group: {:?}", group_name);
            }
        }
    }

    fn gen_element(&mut self, struct_name: &str, tag_name: FullName<'input>, type_: &InlineComplexType<'input, ()>) {
        let mod_name = self.get_module_name(tag_name);
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_element!({}, {:?}, \"{}\", attributes = {{",
            struct_name, tag_name.namespace().unwrap_or(""), tag_name.local_name()));
        let InlineComplexType { min_occurs, max_occurs, type_ } = type_;

        let mut struct_ = cg::Struct::new(&struct_name);
        struct_.vis("pub").derive("Debug").derive("PartialEq").generic("'input");
        let mut empty_struct = false;
        struct_.field("pub attrs", "HashMap<FullName<'input>, &'input str>");
        let mut name_gen = NameGenerator::new();
        let mut generated_attrs = HashSet::new();
        let mut seen_attrs = HashMap::new();
        
        println!("{:?}", type_);
        let attrs = self.compute_attrs(&type_.type_, &type_.attrs);
        self.gen_attrs(&mut struct_, &mut impl_code, &mut name_gen, &attrs, &mut seen_attrs, &mut generated_attrs, false);
        /* WTF was this supposed to do?
         * {
            let field_writer = &mut |_, _, _, _, _| ();
            let doc_writer = &mut |_| ();
            self.write_type_in_struct_def(field_writer, &mut Some(doc_writer), &type_);
        } */
        impl_code.push(format!("}}, fields = {{"));
        if let Some((type_mod_name, type_name)) = self.gen_complex_type(type_) {
            self.gen_field(&mut empty_struct, &mut struct_, &mut impl_code, &type_.doc, &mut name_gen, *min_occurs, *max_occurs, (type_mod_name, type_name));
        }
        struct_.doc(&type_.doc.to_string());

        impl_code.push(format!("}});"));
        let mut module = self.scope.get_module_mut(&mod_name).expect(&mod_name);
        module.push_struct(struct_);
        module.scope().raw(&impl_code.join("\n"));
    }

    /*
    fn write_type_in_struct_def<'a, F, H>(&'a self,
            field_writer: &mut F,
            doc_writer: &mut Option<&mut H>,
            type_: &'a InlineComplexType<'input, ()>,
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
    }*/

    fn compute_attrs(&self, type_: &Type<'input>, own_attrs: &Attrs<'input>) -> Attrs<'input> {
        match type_ {
            Type::Alias(name) => {
                println!("{:?} {:?}", type_, name);
                if let Some(target_type) = self.get_type(*name) {
                    let target_attrs = self.compute_attrs(&target_type.type_, &target_type.attrs);
                    self.extend_attrs(&target_attrs, own_attrs)
                }
                else {
                    // TODO: check that simple type exists
                    own_attrs.clone()
                }
            },
            Type::Sequence(_) |
            Type::Element(_) |
            Type::Group(_) |
            Type::ElementRef(_) |
            Type::Choice(_) |
            Type::Empty |
            Type::Any => {
                own_attrs.clone()
            }
            Type::Extension(base, ext_type) => {
                let base_type = &self.get_type(*base).unwrap();
                let base_attrs = self.compute_attrs(&base_type.type_, &base_type.attrs);
                let InlineComplexType { min_occurs, max_occurs, type_: ref ext_type } = **ext_type;
                assert_eq!(min_occurs, 1); // FIXME: type of 'ext_type'
                assert_eq!(max_occurs, 1); // FIXME: type of 'ext_type'
                let own_attrs = self.extend_attrs(own_attrs, &ext_type.attrs); // XXX
                self.extend_attrs(&base_attrs, &own_attrs)
            },
            Type::Restriction(base, ext_type) => {
                let base_type = &self.get_type(*base).unwrap();
                let base_attrs = self.compute_attrs(&base_type.type_, &base_type.attrs);
                let InlineComplexType { min_occurs, max_occurs, type_: ref ext_type } = **ext_type;
                assert_eq!(min_occurs, 1); // FIXME: type of 'ext_type'
                assert_eq!(max_occurs, 1); // FIXME: type of 'ext_type'
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
