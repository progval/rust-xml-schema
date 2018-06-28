use std::fs::File;
use std::io::Read;

use xmlparser;

use parse_xsd;
use round0_parser::*;
use generated::UNQUAL;
use support::*;

#[test]
fn generated_parses_person_xsd() {
    let mut f = File::open("XMLSchema.xsd").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let tokenizer = xmlparser::Tokenizer::from(&s[..]);
    let mut stream = Box::new(InnerStream::new(tokenizer));
    stream.next(); // Eat the declaration
    stream.next(); // Eat the DTD start
    stream.next(); // Eat comment
    stream.next(); // Eat comment
    stream.next(); // Eat the DTD end
    let doc = UNQUAL::schema_e::parse_xml(&mut stream, &mut (), &());
    assert_ne!(doc, None);
    println!("{:#?}", doc);
    assert!(false);
}
