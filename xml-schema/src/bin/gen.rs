use std::env::args_os;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

extern crate xmlparser;
extern crate xml_schema;
extern crate codegen;
use xml_schema::processor::*;
use xml_schema::parser_generator::*;
use xml_schema::parse_xsd;

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
        documents.push((filename, parse_xsd(input)));
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
    println!("#[allow(unused_imports)]\nuse support;\n{}", scope.to_string());
}
