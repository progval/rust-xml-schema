#![recursion_limit="80"]

pub extern crate xmlparser;
extern crate codegen;
extern crate heck;
extern crate num_traits;
extern crate bigdecimal;

#[macro_use] pub mod macros;
pub mod xml_utils;
pub mod names;
pub mod support;
pub mod primitives;
pub mod bigfloat;

pub mod utils;

pub mod asts;
pub mod toplevel;
pub mod processor2;
pub mod attrs_bubble_up;
pub mod name_allocator;
pub mod ungroup;
//pub mod parser_generator2;

pub mod parser;
pub mod processor;
pub mod parser_generator;

#[cfg(test)]
mod test_parser;
#[cfg(test)]
mod test_parser_schema;

use support::{ParseXml, InnerStream, ParseContext, ParentContext};

pub use processor::Processor;
pub use parser_generator::{XsdParseContext, ParserGenerator};

pub fn parse_xsd<'input>(xsd: &'input str) -> (Option<parser::xs::Schema<'input>>, XsdParseContext) {
    let mut visitor = XsdParseContext::default();
    let ast = parse_xsd_with_visitor(xsd, &mut visitor);
    (ast, visitor)
}

pub fn parse_xsd_with_visitor<'input, TParseContext: ParseContext<'input>>(xsd: &'input str, visitor: &mut TParseContext) -> Option<parser::xs::Schema<'input>> {
    let tokenizer = xmlparser::Tokenizer::from(xsd);
    let mut stream = Box::new(InnerStream::new(tokenizer));
    parser::xs::Schema::parse_xml(&mut stream, visitor, &ParentContext::default())
}

