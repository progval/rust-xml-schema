#![recursion_limit="80"]

extern crate xmlparser;
extern crate codegen;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod round0_parser;
pub mod support;
pub mod round0_parser_generator;

#[cfg(not(feature="bootstrap"))]
pub mod generated;
#[cfg(not(feature="bootstrap"))]
pub mod parser_generator;
#[cfg(test)]
mod test_generated;
#[cfg(test)]
mod test_generated_schema;

pub fn parse_xsd(xsd: &str) -> round0_parser::Document {
    let mut stream = xmlparser::Tokenizer::from(xsd);
    round0_parser::Parser::parse_document(&mut stream)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use round0_parser::*;

	const PERSON_XSD: &'static str = r#"
	  <?xml version="1.0" encoding="UTF-8"?>
	  <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="foo">
		<xs:element name="person">
		  <xs:complexType>
			<xs:sequence>
			  <xs:element name="name" type="xs:string" />
			  <xs:element name="firstname" type="xs:string" />
			  <xs:element name="birthdate" type="xs:date" />
			</xs:sequence>
		  </xs:complexType>
		</xs:element>
	  </xs:schema>"#;
    #[test]
    fn parse_person_schema() {
        let doc = parse_xsd(PERSON_XSD);

        let mut namespaces = HashMap::new();
        namespaces.insert("xs".to_string(), "http://www.w3.org/2001/XMLSchema");

        let mut elements = HashMap::new();
        elements.insert("person".into(), Element {
            min_occurs: None,
            max_occurs: None,
            name: "person".into(),
            attrs: vec![],
            mixed: false,
            abstract_: false,
            type_: Some(ElementType::Sequence(vec![
                (None, None, ElementType::Element(Box::new(Element {
                    min_occurs: None,
                    max_occurs: None,
                    name: "name".into(),
                    attrs: vec![],
                    mixed: false,
                    abstract_: false,
                    type_: Some(ElementType::String),
                }))),
                (None, None, ElementType::Element(Box::new(Element {
                    min_occurs: None,
                    max_occurs: None,
                    name: "firstname".into(),
                    attrs: vec![],
                    mixed: false,
                    abstract_: false,
                    type_: Some(ElementType::String),
                }))),
                (None, None, ElementType::Element(Box::new(Element {
                    min_occurs: None,
                    max_occurs: None,
                    name: "birthdate".into(),
                    attrs: vec![],
                    mixed: false,
                    abstract_: false,
                    type_: Some(ElementType::Date),
                }))),
            ]))
        });

        assert_eq!(doc, Document {
            version: Some("1.0"),
            encoding: Some("UTF-8"),
            standalone: None,
            schema: Some(Schema {
                namespaces: namespaces,
                target_namespace: "foo",
                groups: HashMap::new(),
                elements: elements,
                types: HashMap::new(),
                substitution_groups: HashMap::new(),
            }),
        });
    }
}
