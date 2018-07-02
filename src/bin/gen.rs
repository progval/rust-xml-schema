use std::io::stdin;
use std::io::Read;

extern crate xmlparser;
extern crate xml_schema;
extern crate codegen;
use xml_schema::generated::UNQUAL;
use xml_schema::parser_generator::*;
use xml_schema::support::*;

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let tokenizer = xmlparser::Tokenizer::from(&s[..]);
    let mut stream = Box::new(InnerStream::new(tokenizer));
    stream.next(); // Eat the declaration
    stream.next(); // Eat the DTD start
    stream.next(); // Eat comment
    stream.next(); // Eat comment
    stream.next(); // Eat the DTD end
    let doc = UNQUAL::schema_e::parse_xml(&mut stream, &mut (), &()).unwrap();
    let mut gen = ParserGenerator::new(&doc);
    let scope = gen.gen(&doc);
    //println!("#[allow(bad_style)]\nextern crate xml_schema;use xml_schema::support;\n{}\n{}", MACROS, scope.to_string());
    println!("#[allow(bad_style)]\n#[macro_use] use support;\n{}", scope.to_string());
}
