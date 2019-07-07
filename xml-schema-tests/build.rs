extern crate xml_schema;

use std::env;
use std::fs::{File, read_dir};
use std::io::{Read, Write};
use std::path::Path;
use std::ffi::OsStr;
use std::env::current_dir;

use xml_schema::{Processor, ParserGenerator, parse_xsd};

fn main() {
    let mut in_dir = current_dir().unwrap();
    in_dir.push("src");
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    for entry in read_dir(in_dir.clone()).expect(&format!("Could not read dir {:?}", in_dir)) {
        let in_path = entry.unwrap().path();
        if in_path.extension() != Some(&OsStr::new("xsd")) {
            continue;
        }
        println!("cargo:rerun-if-changed={}", in_path.to_str().unwrap());

        let mut in_file =
            File::open(in_path.clone())
            .expect(&format!("Could not open {:?}", in_path));
        let mut in_xml = String::new();
        in_file
            .read_to_string(&mut in_xml)
            .expect(&format!("Could not read {:?}", in_path));
        let (document, parse_context) = parse_xsd(&in_xml);
        let document = document.expect(&format!("Could not parse {:?}", in_path));

        let mut proc = Processor::new(&document);
        proc.process_ast(&document);

        let renames = Default::default();
        let mut gen = ParserGenerator::new(vec![proc], &parse_context, renames);
        gen.gen_target_scope();

        let filename = in_path.file_name().unwrap();
        let out_path = 
            Path::new(&out_dir)
            .join(filename)
            .with_extension("rs");
        println!("cargo:warning=printing to: {}", out_path.to_str().unwrap());
        let mut out_file =
            File::create(out_path.clone())
            .expect(&format!("Could not create {:?}", out_path));
        out_file
            .write(b"#[allow(unused_imports)]\nuse xml_schema::support;\n")
            .expect(&format!("Could not write in {:?}", out_path));
        out_file
            .write(gen.scope.to_string().as_bytes())
            .expect(&format!("Could not write in {:?}", out_path));
    }
    println!("cargo:rerun-if-changed={}", in_dir.to_str().unwrap());
}
