use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use xmlparser;

use parse_xsd;
use parser::xs;
use parser_generator::ParserGenerator;
use support::*;

#[test]
fn generated_parses_person_xsd() {
    let mut f = File::open("XMLSchema.xsd").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let (doc, _) = parse_xsd(&s);
    assert_ne!(doc, None);
}

#[test]
fn round1_parser_person_xsd() {
    let mut f = File::open("XMLSchema.xsd").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let (doc, _) = parse_xsd(&s);
    assert_ne!(doc, None);
    //let mut parser_generator = ParserGenerator::new(doc.as_ref().unwrap(), HashMap::new());
    //parser_generator.gen(doc.as_ref().unwrap()).to_string();
}

