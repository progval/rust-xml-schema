use std::io::stdin;
use std::io::Read;
use std::collections::HashMap;

extern crate xmlparser;
extern crate xml_schema;
extern crate codegen;
use xml_schema::generated::UNQUAL;
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
    ("ChoiceRestrictionExtension", "ContentDef"),
    ("choice_restriction_extension", "content_def"),
    ("ChoiceAppinfoDocumentation", "AnnotationContent"),
    ("choice_appinfo_documentation", "annotation_content"),
    ];

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

    let mut renames = HashMap::new();
    for (from_, to_) in RENAMES {
        renames.insert(from_.to_string(), to_.to_string());
    }

    let mut gen = ParserGenerator::new(&doc, renames);
    let scope = gen.gen(&doc);
    //println!("#[allow(bad_style)]\nextern crate xml_schema;use xml_schema::support;\n{}\n{}", MACROS, scope.to_string());
    println!("#[allow(bad_style)]\n#[macro_use] use support;\n{}", scope.to_string());
}
