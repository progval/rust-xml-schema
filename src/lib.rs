extern crate xmlparser;
extern crate codegen;

mod parser;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use parser::*;

	const PERSON_XSD: &'static str = r#"
	  <?xml version="1.0" encoding="UTF-8"?>
	  <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
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
        let mut stream = xmlparser::Tokenizer::from(PERSON_XSD);
        let doc = parser::Parser::parse_document(&mut stream);

        let mut namespaces = HashMap::new();
        namespaces.insert("xs".to_string(), "http://www.w3.org/2001/XMLSchema");
        assert_eq!(doc, Document {
            version: Some("1.0"),
            encoding: Some("UTF-8"),
            standalone: None,
            schema: Some(Schema {
                namespaces: namespaces,
                elements: vec![
                ],
            }),
        });
    }
}
