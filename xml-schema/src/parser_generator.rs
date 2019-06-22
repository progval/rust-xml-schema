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
    module_names: HashMap<Option<&'input str>, String>, // URI -> module name
    primitive_types: HashMap<&'static str, RichType<'input, Type<'input>>>,
    simple_restrictions: HashMap<(FullName<'input>, Facets<'input>), String>,
    renames: HashMap<String, String>,
    type_names: HashMap<TypeId, (String, String)>, // (mod_name, type_name)
    simple_type_names: HashMap<SimpleTypeId, (String, String)>, // (mod_name, type_name)
    self_gen: bool,
}

impl<'ast, 'input: 'ast> ParserGenerator<'ast, 'input> {
    pub fn new(processors: Vec<Processor<'ast, 'input>>, parse_context: &XsdParseContext<'input>, renames: HashMap<String, String>) -> ParserGenerator<'ast, 'input> {
        let mut module_names = HashMap::new();
        module_names.insert(None, "unqualified".to_string());
        let mut name_gen = NameGenerator::new();

        let mut primitive_types = HashMap::new();
        for (name, type_name) in PRIMITIVE_TYPES {
            primitive_types.insert(*name, RichType::new(NameHint::new(name), Type::Simple(SimpleType::Primitive(name, type_name)), Documentation::new()));
        }

        for (&uri, ns) in parse_context.namespaces.iter() {
            if Some(&ns.to_string()) != module_names.get(&Some(uri)) {
                module_names.insert(Some(uri), name_gen.gen_name(ns.to_string()));
            }
        }

        let mut self_gen = false;
        for proc in &processors {
            if proc.target_namespace == Some(&SCHEMA_URI) {
                self_gen = true;
            }
        }
        ParserGenerator {
            processors, renames, module_names, primitive_types, self_gen,
            simple_restrictions: HashMap::new(),
            type_names: HashMap::new(), simple_type_names: HashMap::new(),
        }
    }

    pub fn get_module_name(&self, qname: FullName<'input>) -> String {
        if qname.namespace() == Some(SCHEMA_URI) {
            for (name, _) in PRIMITIVE_TYPES {
                if *name == qname.local_name() {
                    return "support".to_string();
                }
            }
        }
        self.module_names.get(&qname.namespace()).expect(&format!("{:?}", qname.namespace())).clone()
    }

    pub fn gen_target_scope(&mut self) -> cg::Scope {
        let mut scope = cg::Scope::new();
        scope.raw("pub use std::collections::HashMap;");
        scope.raw("pub use std::marker::PhantomData;");
        self.create_modules(&mut scope);
        self.allocate_names();
        self.gen_anonymous_types(&mut scope);
        self.gen_sequences(&mut scope);
        self.gen_elements(&mut scope);
        self.gen_inline_elements(&mut scope);
        scope
    }

    fn create_modules(&mut self, scope: &mut cg::Scope) {
        let mut modules: Vec<_> = self.module_names.iter().collect();
        modules.sort();
        for (&uri, mod_name) in modules {
            if !self.self_gen && uri == Some(SCHEMA_URI) {
                scope.raw(&format!("#[allow(unused_imports)]\npub use xml_schema::parser::xs as {};", mod_name));
            }
            else {
                let mut module = scope.new_module(mod_name);
                module.vis("pub");
                module.scope().raw(&format!("//! {:?}", uri));
                module.scope().raw("#[allow(unused_imports)]\nuse super::*;");
            }
        }
    }

    fn allocate_names(&mut self) {
        let mut name_gen = NameGenerator::new();
        for proc in &self.processors {
            // Complex types
            let mut types: Vec<_> = proc.anonymous_types.iter().collect();
            types.sort_by_key(|&(id,_type)| id);
            for (&id, ref type_) in types {
                let name = name_from_hint(&type_.name_hint).unwrap();  // TODO: handle None
                let (mod_name, type_name) = match type_.type_ {
                    Type::Choice(_) => ("choices".to_string(), name.to_camel_case()),
                    _ => unimplemented!("{:?}", type_.type_),
                };
                let type_name = escape_keyword(&type_name);
                let type_name = name_gen.gen_name(type_name);
                let type_name = self.renames.get(&type_name).unwrap_or(&type_name);
                self.type_names.insert(id, (mod_name, type_name.clone()));
            }

            // Simple types
            let mut types: Vec<_> = proc.anonymous_simple_types.iter().collect();
            types.sort_by_key(|&(id,_type)| id);
            for (&id, ref type_) in types {
                let (mod_name, type_name) = match type_.type_ {
                    SimpleType::Alias(name) => {
                        if name.namespace() == Some(&SCHEMA_URI) {
                            let mut res = None;
                            for (prim_name, prim_type_name) in PRIMITIVE_TYPES {
                                if *prim_name == name.local_name() {
                                    res = Some(("support".to_string(), prim_type_name.to_string()))
                                }
                            }
                            res.expect(&format!("Unknown primitive type in schema {}: {:?}", SCHEMA_URI, name.local_name()))
                        }
                        else {
                            let type_mod_name = self.get_module_name(name);
                            let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                            let type_name = escape_keyword(&name.local_name().to_camel_case());
                            (type_mod_name, type_name)
                        }
                    },
                    SimpleType::Restriction(base_name, facets) => {
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
                        ("simple_restrictions".to_string(), name)
                    }
                    SimpleType::Primitive(field_name, type_name) => {
                        ("support".to_string(), type_name.to_string())
                    },
                    SimpleType::Union(type_names) => {
                        let name = name_from_hint(&type_.name_hint).unwrap();
                        ("unions".to_string(), name)
                    },
                    SimpleType::List(name) => {
                        let name = name_from_hint(&type_.name_hint).unwrap();
                        ("lists".to_string(), name)
                    },
                    SimpleType::Empty => {
                        ("fixme_empty".to_string(), "FixmeEmpty".to_string()) // FIXME
                    },
                };
                let type_name = escape_keyword(&type_name);
                let type_name = name_gen.gen_name(type_name);
                let type_name = self.renames.get(&type_name).unwrap_or(&type_name);
                self.simple_type_names.insert(id, (mod_name, type_name.clone()));
            }
        }
    }

    fn gen_anonymous_types(&self, scope: &mut cg::Scope) {
        let choices_module = scope.new_module("choices");
        choices_module.vis("pub");
        choices_module.scope().raw("#[allow(unused_imports)]\nuse super::*;");

        let unions_module = scope.new_module("unions");
        unions_module.vis("pub");
        unions_module.scope().raw("#[allow(unused_imports)]\nuse super::*;");

        let simple_restrictions_module = scope.new_module("simple_restrictions");
        simple_restrictions_module.vis("pub");
        simple_restrictions_module.scope().raw("#[allow(unused_imports)]\nuse super::*;");

        let lists_module = scope.new_module("lists");
        lists_module.vis("pub");
        lists_module.scope().raw("#[allow(unused_imports)]\nuse super::*;");

        for proc in &self.processors {
            // Complex types
            let mut types: Vec<_> = proc.anonymous_types.iter().collect();
            types.sort_by_key(|&(id,_type)| id);
            for (id, ref type_) in types {
                let (type_mod_name, type_name) = self.type_names.get(id).unwrap();
                match type_.type_ {
                    Type::Choice(items) => {
                        let items = items.iter().map(|id| proc.anonymous_types.get(id).unwrap()).collect();
                        self.gen_choice(choices_module.scope(), &type_name, &items, &Documentation::new());
                    },
                    Type::Sequence(items) => {
                        let items = items.iter().map(|id| proc.anonymous_types.get(id).unwrap()).collect();
                        self.gen_choice(choices_module.scope(), &type_name, &items, &Documentation::new());
                    },
                    _ => panic!("wat"),
                }
            }

            // Simple types
            let mut types: Vec<_> = proc.anonymous_simple_types.iter().collect();
            types.sort_by_key(|&(id,_type)| id);
            for (id, ref type_) in types {
                let type_name = self.simple_type_names.get(id).unwrap();
                match type_.type_ {
                    SimpleType::Alias(name) => {}, // nothing to generate
                    SimpleType::Restriction(ref base_name, ref facets) => {
                        let base_mod_name = self.module_names.get(&base_name.namespace()).unwrap();
                        let base_type_name = escape_keyword(base_name.local_name());
                        self.gen_simple_restriction(simple_restrictions_module.scope(), type_name.1, (base_mod_name, &base_type_name), facets, type_.doc);
                    }
                    SimpleType::Primitive(field_name, type_name) => {}, // Nothing to generate
                    SimpleType::Union(variants) => {
                        let mut name_gen = NameGenerator::new();
                        let variants: Vec<_> = variants.iter().map(|id| {
                            let variant_name = name_from_hint(&proc.anonymous_simple_types.get(id).unwrap().name_hint).unwrap_or("variant".to_string());
                            let variant_name = escape_keyword(&variant_name.to_camel_case());
                            let variant_name = name_gen.gen_name(variant_name);
                            (variant_name, self.simple_type_names.get(id).unwrap())
                        }).collect();
                        self.gen_union(unions_module.scope(), &type_name.1, &variants, &type_.doc);
                    },
                    SimpleType::List(item_type_id) => {
                        let (item_type_mod_name, item_type_name) = self.simple_type_names.get(&item_type_id).unwrap();
                        self.gen_list(unions_module.scope(), type_name.1, (&item_type_mod_name, &item_type_name), &type_.doc);
                    },
                    SimpleType::Empty => {}, // Nothing to generate
                }
            }
        }
    }

    fn gen_choice(&self, scope: &mut cg::Scope, enum_name: &String, items: &Vec<&RichType<'input, Type<'input>>>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_enum!({},", enum_name));
        {
            let enum_ = scope.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
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
                let variant_name = name_from_hint(&item.name_hint)
                    .unwrap_or(format!("{}{}", enum_name, i)).to_camel_case();
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
        scope.raw(&impl_code.join("\n"));
    }
    fn gen_union(&self, module: &mut cg::Scope, enum_name: &String, items: &Vec<(String, &(String, String))>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_union!({}, {{", enum_name));
        {
            let enum_ = module.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (variant_name, (variant_type_mod_name, variant_type_name)) in items.iter() {
                enum_.new_variant(&variant_name).tuple(&format!("{}::{}<'input>", variant_type_mod_name, variant_type_name));
                impl_code.push(format!("    impl_union_variant!({}),", variant_name));
            }
        }
        impl_code.push(format!("}});"));
        module.raw(&impl_code.join("\n"));
    }

    fn gen_list(&self, module: &mut cg::Scope, struct_name: String, item_type_name: (&str, &str), doc: &Documentation<'input>) {
        let (type_mod_name, type_name) = item_type_name;
        let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
        struct_.tuple_field(&format!("pub Vec<{}::{}<'input>>", type_mod_name, type_name));

        module.raw(&format!("impl_list!({}, {}::{});", struct_name, type_mod_name, type_name));
    }

    /*fn gen_simple_types(&mut self, scope: &mut cg::Scope) {
        let mut name_gen = NameGenerator::new();
        for proc in &self.processors {
            let mut types: Vec<_> = proc.simple_types.iter().collect();
            types.sort_by_key(|&(n,_)| n);
            for (&qname, (ref ty, ref doc)) in types {
                let mod_name = self.get_module_name(qname);
                if mod_name == "support" {
                    // Implemented as a primitive, skip it.
                    continue;
                }
                let scope = scope.get_module_mut(&mod_name)
                    .expect(&mod_name).scope();
                let name = escape_keyword(&qname.local_name().to_camel_case());
                let name = name_gen.gen_name(name);
                if let Some((type_mod_name, type_name)) = self.get_simple_type_name(&ty.type_) {
                    scope.raw(&format!("pub type {}<'input> = {}::{}<'input>;", name, type_mod_name, type_name));
                }
                else {
                    panic!("{:?}", ty)
                }
            }
        }
    }*/

    fn gen_simple_restriction(&mut self, module: &mut cg::Scope, struct_name: String, base_name: (&str, &str), facets: &Facets<'input>, doc: Documentation<'input>) {
        let (base_mod_name, base_type_name) = base_name;
        module.raw(&format!("#[derive(Debug, PartialEq)] pub struct {}<'input>(pub {}::{}<'input>);", struct_name, base_mod_name, base_type_name));
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
        module.raw(&format!("impl_simpletype_restriction!({}, Facets {{\n    {}\n}});", struct_name, s.join("\n    ")));
    }

    fn gen_sequences(&mut self, scope: &mut cg::Scope) {
        let module = scope.new_module("sequences");
        module.vis("pub");
        module.scope().raw("#[allow(unused_imports)]\nuse super::*;");

        for proc in &self.processors {
            let mut sequences: Vec<_> = proc.sequences.iter().collect();

            sequences.sort_by_key(|&(i,(n,_))| (n.iter().collect::<Vec<_>>(), i));
            for (sequence, (names, doc)) in sequences {
                for name in names.iter() {
                    self.gen_group_or_sequence(module, name, &sequence.iter().collect(), doc);
                }
            }
        }
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

    fn gen_group_or_sequence(&self, module: &mut cg::Module, struct_name: &'input str, items: &Vec<&RichType<'input, Type<'input>>>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        let struct_name = escape_keyword(&struct_name.to_camel_case());
        let struct_name = self.renames.get(&struct_name).unwrap_or(&struct_name);
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

    fn gen_inline_elements(&mut self, scope: &mut cg::Scope) {
        let module = scope.new_module("inline_elements");
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

    fn gen_elements(&mut self, scope: &mut cg::Scope) {
        for proc in &self.processors {
            let mut elements: Vec<_> = proc.elements.iter().collect();

            elements.sort_by_key(|&(n,_)| n);
            for (&name, element) in elements {
                let mod_name = self.get_module_name(name);
                let mut module = scope.get_module_mut(&mod_name).expect(&mod_name);
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

    fn gen_substitution_enum(&self, scope: &mut cg::Scope, enum_name: &str, substitutions: &Vec<FullName<'input>>) {
        let mut impl_code = Vec::new();
        impl_code.push(format!("impl_enum!({},", enum_name));
        let mut name_gen = NameGenerator::new();
        {
            let enum_ = scope.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
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
        scope.raw(&impl_code.join("\n"));
    }

    fn gen_attrs(&self, struct_: &mut cg::Struct, impl_code: &mut Vec<String>, name_gen: &mut NameGenerator, attrs: &Attrs<'input>, seen_attrs: &mut HashMap<FullName<'input>, AttrUse>, generated_attrs: &mut HashSet<FullName<'input>>, inherited: bool) {
        for (attr_name, use_, attr_type) in &attrs.named {
            if generated_attrs.contains(attr_name) {
                continue;
            }
            let default_type = ("support".to_string(), "AnySimpleType".to_string());
            let (type_mod_name, type_name) = attr_type
                .map(|id| self.simple_type_names.get(&id).unwrap())
                .unwrap_or(&default_type);
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
                    let field_name = name_gen.gen_name(format!("attr_{}", attr_name.local_name()).to_snake_case());
                    struct_.field(&format!("pub {}", field_name), &format!("Option<{}::{}<'input>>", type_mod_name, type_name));
                    impl_code.push(format!("    ({:?}, {:?}) => {}: optional,", attr_name.namespace().unwrap_or(""), attr_name.local_name(), field_name));
                },
                AttrUse::Required => {
                    let field_name = name_gen.gen_name(format!("attr_{}", attr_name.local_name()).to_snake_case());
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

    fn get_type(&self, name: &FullName<'input>) -> &RichType<'input, Type<'input>> {
        if name.namespace() == Some(SCHEMA_URI) {
            if let Some(type_) = self.primitive_types.get(name.local_name()) {
                return type_;
            }
        }
        let mut type_ = None;
        for proc in &self.processors {
            if proc.target_namespace != name.namespace() {
                continue;
            }
            type_ = proc.types.get(name).and_then(|id| proc.anonymous_types.get(id));
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
            Type::GroupRef(min_occurs, max_occurs, name) |
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
                let (type_mod_name, type_name) = self.simple_type_names.get(&type_).unwrap();
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
            Type::Sequence(_) |
            Type::Element(_) |
            Type::GroupRef(_) |
            Type::ElementRef(_) |
            Type::Choice(_) |
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
