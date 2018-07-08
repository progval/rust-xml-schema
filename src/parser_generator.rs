use std::collections::HashMap;

use codegen as cg;
use heck::{SnakeCase, CamelCase};

//use support::*;
use primitives::PRIMITIVE_TYPES;
use processor::*;
use names::*;

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
    module_names: HashMap<&'input str, String>, // URI -> module name
    primitive_types: HashMap<&'static str, (RichType<'input, SimpleType<'input>>, Documentation<'input>)>,
    renames: HashMap<String, String>,
}

impl<'ast, 'input: 'ast> ParserGenerator<'ast, 'input> {
    pub fn new(processors: Vec<Processor<'ast, 'input>>, renames: HashMap<String, String>) -> ParserGenerator<'ast, 'input> {
        let mut module_names: HashMap<&'input str, String> = HashMap::new();
        let mut name_gen = NameGenerator::new();

        let mut primitive_types = HashMap::new();
        for (name, type_name) in PRIMITIVE_TYPES {
            primitive_types.insert(*name, (RichType::new(NameHint::new(name), SimpleType::Primitive(name, type_name), Documentation::new()), Documentation::new()));
        }

        for proc in &processors {
            for (ns, uri) in proc.namespaces.namespaces.iter() {
                if Some(&ns.to_string()) != module_names.get(uri) {
                    module_names.insert(*uri, name_gen.gen_name(ns.to_string()));
                }
            }
        }
        ParserGenerator { processors, renames, module_names, primitive_types }
    }

    pub fn get_module_name(&self, qname: FullName<'input>) -> String {
        let (prefix, _) = qname.as_tuple();
        self.module_names.get(prefix).cloned().unwrap_or("UNQUAL".to_string())
    }

    pub fn gen_target_scope(&mut self) -> cg::Scope {
        let mut scope = cg::Scope::new();
        scope.raw("extern crate xmlparser;");
        scope.raw("pub use std::collections::HashMap;");
        scope.raw("pub use std::marker::PhantomData;");
        scope.raw("pub use support::*;");
        self.create_modules(&mut scope);
        self.gen_choices(&mut scope);
        self.gen_simple_types(&mut scope);
        self.gen_lists(&mut scope);
        self.gen_unions(&mut scope);
        self.gen_sequences(&mut scope);
        self.gen_elements(&mut scope);
        self.gen_inline_elements(&mut scope);
        self.gen_groups(&mut scope);
        scope
    }

    fn create_modules(&mut self, scope: &mut cg::Scope) {
        let mut modules: Vec<_> = self.module_names.iter().collect();
        modules.sort();
        for (uri, mod_name) in modules {
            let mut module = scope.new_module(mod_name);
            module.vis("pub");
            module.scope().raw(&format!("//! {}", uri));
            module.scope().raw("#[allow(unused_imports)]\nuse super::*;");
        }
        if let Some(mod_name) = self.module_names.get(SCHEMA_URI) {
            scope.get_module_mut(mod_name).unwrap().scope().raw("pub(crate) use primitives::*;");
        }
    }

    fn gen_choices(&self, scope: &mut cg::Scope) {
        let module = scope.new_module("enums");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut name_gen = NameGenerator::new();
        for proc in &self.processors {
            let mut choices: Vec<_> = proc.choices.iter().collect();
            choices.sort_by_key(|&(t,names)| (t, names.iter().collect::<Vec<_>>()));
            for (ref choice, ref names) in choices {
                for name in names.iter() {
                    let enum_name = escape_keyword(&name.to_camel_case());
                    let enum_name = name_gen.gen_name(enum_name);
                    self.gen_choice(module.scope(), &enum_name, choice, &Documentation::new());
                }
            }
        }
    }
    fn gen_choice(&self, scope: &mut cg::Scope, enum_name: &String, items: &Vec<RichType<'input, Type<'input>>>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        let enum_name = self.renames.get(enum_name).unwrap_or(enum_name);
        impl_code.push(format!("impl_enum!({},", enum_name));
        {
            let enum_ = scope.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            for (i, item) in items.iter().enumerate() {
                let mut fields = Vec::new();
                let mut doc = doc.clone();
                {
                    let mut name_gen = NameGenerator::new();
                    let field_writer = &mut |variant_name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
                        let variant_name = escape_keyword(&name_gen.gen_name(variant_name.to_snake_case()));
                        let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                        let type_name = escape_keyword(&type_name.to_camel_case());
                        fields.push((variant_name, type_mod_name, min_occurs, max_occurs, type_name));
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
    fn gen_unions(&self, scope: &mut cg::Scope) {
        let module = scope.new_module("unions");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut name_gen = NameGenerator::new();
        for proc in &self.processors {
            let mut unions: Vec<_> = proc.unions.iter().collect();
            unions.sort_by_key(|&(items, names)| (items, names.iter().collect::<Vec<_>>()));
            for (ref items, ref names) in unions {
                for name in names.iter() {
                    let enum_name = escape_keyword(&name.to_camel_case());
                    let enum_name = name_gen.gen_name(enum_name);
                    let enum_ = module.new_enum(&enum_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
                    for item in items.iter() {
                        let RichType { name_hint, type_, doc } = item;
                        let variant_name = name_from_hint(&item.name_hint).unwrap();
                        // TODO: deduplicate this match{} with gen_lists
                        let (type_mod_name, type_name) = match type_ {
                            SimpleType::Alias(name) => {
                                let (type_mod_name, type_name) = name.as_tuple();
                                let type_mod_name = self.module_names.get(type_mod_name).expect(type_mod_name);
                                let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                                let type_name = escape_keyword(&type_name.to_camel_case());
                                (type_mod_name, type_name)
                            },
                            SimpleType::Primitive(field_name, type_name) => {
                                ("support".to_string(), type_name.to_string())
                            },
                            SimpleType::Union(type_name) => {
                                let type_name = escape_keyword(&type_name.to_camel_case());
                                ("unions".to_string(), type_name)
                            },
                            SimpleType::List(name) => {
                                let type_name = escape_keyword(&name.to_camel_case());
                                ("lists".to_string(), type_name)
                            },
                            SimpleType::Empty => {
                                continue; // TODO?
                            },
                            _ => unimplemented!("Union of {:?}", type_),
                        };
                        enum_.new_variant(&variant_name).tuple(&format!("{}::{}<'input>", type_mod_name, type_name));
                    }
                }
            }
        }
    }

    fn gen_lists(&self, scope: &mut cg::Scope) {
        let module = scope.new_module("lists");
        module.vis("pub");
        module.scope().raw("use super::*;");
        let mut name_gen = NameGenerator::new();
        for proc in &self.processors {
            let mut lists: Vec<_> = proc.lists.iter().collect();
            lists.sort_by_key(|&(item_type, names)| (item_type, names.iter().collect::<Vec<_>>()));
            for (ref item_type, ref names) in lists {
                for name in names.iter() {
                    let name = escape_keyword(&name.to_camel_case());
                    let name = name_gen.gen_name(name);
                    let type_ = &item_type.type_;
                    let (type_mod_name, type_name) = match &item_type.type_ {
                        SimpleType::Alias(name) => {
                            let (type_mod_name, type_name) = name.as_tuple();
                            let type_mod_name = self.module_names.get(type_mod_name).expect(type_mod_name);
                            let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                            let type_name = escape_keyword(&type_name.to_camel_case());
                            (type_mod_name, type_name)
                        },
                        SimpleType::Primitive(field_name, type_name) => {
                            ("support".to_string(), type_name.to_string())
                        },
                        SimpleType::Union(type_name) => {
                            let type_name = escape_keyword(&type_name.to_camel_case());
                            ("unions".to_string(), type_name)
                        },
                        SimpleType::List(name) => {
                            let type_name = escape_keyword(&name.to_camel_case());
                            ("lists".to_string(), type_name)
                        },
                        SimpleType::Empty => {
                            continue; // TODO?
                        },
                        _ => unimplemented!("List of {:?}", item_type),
                    };
                    module.scope().raw(&format!("pub type {}<'input> = support::List<'input, {}::{}<'input>>;", name, type_mod_name, type_name));
                }
            }
        }
    }
    fn gen_simple_types(&self, scope: &mut cg::Scope) {
        let mut name_gen = NameGenerator::new();
        for proc in &self.processors {
            let mut types: Vec<_> = proc.simple_types.iter().collect();
            types.sort_by_key(|&(n,_)| n);
            for (ref name, (ref ty, ref doc)) in types {
                let (mod_name, name) = name.as_tuple();
                let name = escape_keyword(&name.to_camel_case());
                let name = name_gen.gen_name(name);
                let (type_mod_name, type_name) = match &ty.type_ {
                    SimpleType::Alias(name) => {
                        let (type_mod_name, type_name) = name.as_tuple();
                        let type_mod_name = self.module_names.get(type_mod_name).expect(type_mod_name);
                        let type_mod_name = escape_keyword(&type_mod_name.to_snake_case());
                        let type_name = escape_keyword(&type_name.to_camel_case());
                        (type_mod_name, type_name)
                    },
                    SimpleType::Primitive(field_name, type_name) => {
                        ("support".to_string(), type_name.to_string())
                    },
                    SimpleType::Union(type_name) => {
                        let type_name = escape_keyword(&type_name.to_camel_case());
                        ("unions".to_string(), type_name)
                    },
                    SimpleType::List(name) => {
                        let type_name = escape_keyword(&name.to_camel_case());
                        ("lists".to_string(), type_name)
                    },
                    SimpleType::Empty => {
                        continue; // TODO?
                    },
                    _ => unimplemented!("Simple type of {:?}", ty),
                };
                scope.get_module_mut(self.module_names.get(mod_name).expect(mod_name))
                    .unwrap().scope()
                    .raw(&format!("pub type {}<'input> = {}::{}<'input>;", name, type_mod_name, type_name));
            }
        }
    }

    fn gen_sequences(&mut self, scope: &mut cg::Scope) {
        let module = scope.new_module("sequences");
        module.vis("pub");
        module.scope().raw("use super::*;");

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

    fn gen_groups(&mut self, scope: &mut cg::Scope) {
        for proc in &self.processors {
            let mut groups: Vec<_> = proc.groups.iter().collect();

            groups.sort_by_key(|&(n,_)| n);
            for (&name, group) in groups {
                let mod_name = self.get_module_name(name);
                let mut module = scope.get_module_mut(&mod_name).unwrap();
                let (mod_name, struct_name) = name.as_tuple();
                if let Type::InlineChoice(ref items) = group.type_ {
                    self.gen_choice(module.scope(), &struct_name.to_string().to_camel_case(), items, &group.doc);
                }
                else if let Type::InlineSequence(ref items) = group.type_ {
                    self.gen_group_or_sequence(module, struct_name, &items.iter().collect(), &group.doc);
                }
                else {
                    self.gen_group_or_sequence(module, struct_name, &vec![group], &group.doc);
                }
            }
        }
    }

    fn gen_fields(&self, empty_struct: &mut bool, struct_: &mut cg::Struct, impl_code: &mut Vec<String>, doc: &mut Documentation<'input>, name_gen: &mut NameGenerator, type_: &Type<'input>, override_name: Option<&'input str>) {
        let field_writer = &mut |name: &str, type_mod_name: &str, min_occurs, max_occurs, type_name: &str| {
            *empty_struct = false;
            let name = override_name.unwrap_or(name);
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
                    self.gen_fields(&mut empty_struct, struct_, &mut impl_code, &mut doc, &mut name_gen, &item.type_, None);
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
        module.scope().raw("use super::*;");
        for proc in &self.processors {
            let mut elements: Vec<_> = proc.inline_elements.iter().collect();

            elements.sort_by_key(|&((n,_,_),(n2,_))| (n, n2.iter().collect::<Vec<_>>()));
            for ((tag_name, attrs, element), (struct_names, doc)) in elements {
                // struct_names is always non-empty.

                let mut struct_names: Vec<_> = struct_names.iter().map(|s| s.to_camel_case()).collect();

                // Sort them to get the shortest one, and alias all the others to this one
                struct_names.sort_by_key(|n| n.len()); // TODO: just use a min_by_key
                for alias in &struct_names[1..] {
                    module.scope().raw(&format!("pub type {}<'input> = {}<'input>;", alias, struct_names[0]));
                }

                self.gen_element(module, &struct_names[0], tag_name, attrs, element, doc);
            }
        }
    }

    fn gen_elements(&mut self, scope: &mut cg::Scope) {
        for proc in &self.processors {
            let mut elements: Vec<_> = proc.elements.iter().collect();

            elements.sort_by_key(|&(n,_)| n);
            for (&name, (attrs, element)) in elements {
                let mod_name = self.get_module_name(name);
                let mut module = scope.get_module_mut(&mod_name).expect(&mod_name);
                let (prefix, local) = name.as_tuple();
                let struct_name = escape_keyword(&local.to_camel_case());
                self.gen_element(module, &struct_name, &name, attrs, &element.type_, &element.doc);
            }
        }
    }

    fn gen_element(&self, module: &mut cg::Module, struct_name: &str, tag_name: &FullName<'input>, attrs: &Attrs<'input>, type_: &Type<'input>, doc: &Documentation<'input>) {
        let mut impl_code = Vec::new();
        let (_, tag_name) = tag_name.as_tuple();
        impl_code.push(format!("impl_element!({}, \"{}\", attributes = {{", struct_name, tag_name));
        {
            let struct_ = module.new_struct(&struct_name).vis("pub").derive("Debug").derive("PartialEq").generic("'input");
            let mut empty_struct = false;
            struct_.field("pub attrs", "HashMap<QName<'input>, &'input str>");
            let mut name_gen = NameGenerator::new();
            let mut doc = doc.clone();
            for (attr_name, attr_type) in &attrs.named {
                () // TODO
            }
            impl_code.push(format!("}}, fields = {{"));
            self.gen_fields(&mut empty_struct, struct_, &mut impl_code, &mut doc, &mut name_gen, type_, None);
            struct_.doc(&doc.to_string());
        }
        impl_code.push(format!("}});"));
        module.scope().raw(&impl_code.join("\n"));
    }

    fn get_type(&self, name: &FullName<'input>) -> (&RichType<'input, Type<'input>>, &Documentation<'input>) {
        let mut type_ = None;
        let mut doc = None;
        let (prefix, local) = name.as_tuple();
        for proc in &self.processors {
            if proc.namespaces.target_namespace != prefix {
                continue;
            }
            if let Some(ref t) = proc.types.get(name) {
                type_ = Some(&t.0);
                doc = Some(&t.1);
                break;
            }
        }
        let type_ = type_.expect(&format!("Unknown type name: {:?}", name));
        let doc = doc.unwrap();
        (type_, doc) // TODO: why is doc returned both as .1 and in type_ (a RichType)?
    }

    fn write_type_in_struct_def<'a, F, G>(&'a self,
            field_writer: &mut F,
            doc_writer: &mut Option<&mut G>,
            type_: &'a Type<'input>,
            ) where
            F: FnMut(&'a str, &'a str, usize, usize, &'a str),
            G: FnMut(&'a Documentation<'input>),
            'ast: 'a {
        let mut doc_non_writer: Option<&mut G> = None;
        match &type_ {
            Type::Alias(name) => {
                let (target_type, doc) = self.get_type(name);
                if let Some(ref mut f) = doc_writer {
                    f(&doc);
                }
                self.write_type_in_struct_def(field_writer, doc_writer, &target_type.type_);
            },
            Type::InlineSequence(items) => {
                for item in items {
                    self.write_type_in_struct_def(field_writer, &mut doc_non_writer, &item.type_);
                }
            }
            Type::Sequence(min_occurs, max_occurs, name) => {
                field_writer(name, "sequences", *min_occurs, *max_occurs, name);
            }
            Type::Element(min_occurs, max_occurs, name) => {
                field_writer(name, "inline_elements", *min_occurs, *max_occurs, name);
            }
            Type::Group(min_occurs, max_occurs, name) |
            Type::ElementRef(min_occurs, max_occurs, name) => {
                let (mod_name, type_name) = name.as_tuple();
                let field_name = type_name;
                let mod_name = self.module_names.get(mod_name).expect(mod_name);
                field_writer(field_name, mod_name, *min_occurs, *max_occurs, type_name);
            },
            Type::Choice(min_occurs, max_occurs, ref name) => {
                field_writer(name, "enums", *min_occurs, *max_occurs, name);
            },
            Type::Extension(base, ext_type) => {
                let (base_type, _doc) = &self.get_type(base);
                self.write_type_in_struct_def(field_writer, &mut doc_non_writer, &base_type.type_);
                self.write_type_in_struct_def(field_writer, doc_writer, &ext_type.type_);
            },
            Type::Restriction(base, ext_type) => {
                if let Some(ref mut f) = doc_writer {
                    f(&ext_type.doc);
                }
                let (base_type, _doc) = &self.get_type(base);
                // TODO: do something with the base
                self.write_type_in_struct_def(field_writer, doc_writer, &ext_type.type_);
            },
            Type::Empty => (), // TODO ?
            Type::Any => {
                field_writer("any", "support", 1, 1, "Any")
            },
            _ => unimplemented!("writing {:?}", type_),
        }
    }
}
