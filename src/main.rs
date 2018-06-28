use std::io::stdin;
use std::io::Read;

extern crate xml_schema;
extern crate codegen;
use xml_schema::parse_xsd;
use xml_schema::parser::*;
use xml_schema::parser_generator::*;

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let doc = parse_xsd(&s);
    let mut gen = ParserGenerator::new(&doc);
    let mut scope = codegen::Scope::new();
    gen.gen_unqual_module();
    /*for (name, v) in doc.schema.as_ref().unwrap().types.iter() {
        println!("{} {:?}", name, v);
    }*/
    scope.push_module(gen.nsuri_to_module.remove(doc.schema.as_ref().unwrap().target_namespace).unwrap().1);
    println!("#[allow(bad_style)]\n{}\n{}", MACROS, scope.to_string());
    println!("// substs: {:?}", doc.schema.as_ref().unwrap().substitution_groups);
}
