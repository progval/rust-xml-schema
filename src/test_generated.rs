
use xmlparser;

use parse_xsd;
use parser::*;
use generated::UNQUAL;
use support::*;

const PERSON_XSD: &'static str = r#"
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
fn generated_parses_person_xsd() {
    let tokenizer = xmlparser::Tokenizer::from(PERSON_XSD);
    let mut stream = Box::new(InnerStream::new(tokenizer));
    let doc = UNQUAL::schema_e::parse_xml(&mut stream, &mut (), &());
    assert_ne!(doc, None);
    println!("{:#?}", doc);
    assert!(false);
}
