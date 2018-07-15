use std::env::args_os;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

extern crate xmlparser;
extern crate xml_schema;
extern crate codegen;
use xml_schema::parser::xs;
use xml_schema::processor::*;
use xml_schema::parser_generator::*;
use xml_schema::support::*;

const RENAMES: &[(&'static str, &'static str)] = &[
    ("SequenceDefaultOpenContentAnnotation", "AnnotatedOpenContent"),
    ("sequence_default_open_content_annotation", "open_content"),
    ("ChoiceSimpleTypeComplexType", "Type"),
    ("choice_simple_type_complex_type", "type_"),
    ("SequenceOpenContentTypeDefParticleAttrDeclsAssertions", "CompleteContentModel"),
    ("ChoiceAttributeAttributeGroup", "AttrOrAttrGroup"),
    ("choice_attribute_attribute_group", "attribute"),
    ("SequenceSelectorField", "UniquenessSpec"),
    ("sequence_selector_field", "uniqueness_spec"),
    //("ChoiceRestrictionExtension", "ContentDef"),
    //("choice_restriction_extension", "content_def"),
    ("ChoiceAppinfoDocumentation", "AnnotationContent"),
    ("choice_appinfo_documentation", "annotation_content"),
    ];

fn main() {
    let mut inputs = Vec::new();
    for arg in args_os().skip(1) {
        let mut s = String::new();
        let mut file = File::open(&arg).expect(&format!("Could not open {:?}", arg));
        file.read_to_string(&mut s).expect(&format!("Could not read {:?}", arg));
        inputs.push((arg, s));
    }
    
    let inputs2 = inputs.iter().map(|(arg, s)| (arg, &s[..]));

    let mut documents = Vec::new();
    for (filename, input) in inputs2 {
        let tokenizer = xmlparser::Tokenizer::from(&input[..]);
        let mut stream = Box::new(InnerStream::new(tokenizer));
        loop {
            let tok = stream.next().unwrap();
            match tok {
                xmlparser::Token::EntityDeclaration(_, _) |
                xmlparser::Token::Declaration(_, _, _) |
                xmlparser::Token::DtdStart(_, _) |
                xmlparser::Token::Comment(_) => (),
                xmlparser::Token::DtdEnd => break,
                _ => panic!("Unexpected token {:?}", tok),
            }
        }
        documents.push((filename, xs::Schema::parse_xml(&mut stream, &mut ParseContext::default(), &()).unwrap()));
    }

    let mut processors = Vec::new();
    for (filename, document) in &documents {
        println!("// Input: {:?}", filename);
        let mut proc = Processor::new(document);
        proc.process_ast(document);
        processors.push(proc);
    }

    let mut renames = HashMap::new();
    for (from_, to_) in RENAMES {
        renames.insert(from_.to_string(), to_.to_string());
    }

    let mut gen = ParserGenerator::new(processors, renames);
    let scope = gen.gen_target_scope();
    //println!("#[allow(bad_style)]\nextern crate xml_schema;use xml_schema::support;\n{}\n{}", MACROS, scope.to_string());
    println!("#[allow(unused_imports)]\nuse support;\n{}", scope.to_string());
}
