use std::fmt;

use xmlparser::Error as XmlParserError;
use xmlparser::Token as XmlParserToken;

#[derive(Debug, PartialEq, Default)]
pub struct token<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct NMTOKEN<'input>(&'input str);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct QName<'input>(pub Option<&'input str>, pub &'input str);

impl<'input> From<&'input str> for QName<'input> {
    fn from(s: &'input str) -> QName<'input> {
        let mut splitted = s.split(":");
        let v1 = splitted.next().expect(&format!("Empty qname"));
        let v2 = splitted.next();
        assert_eq!(splitted.next(), None);
        match v2 {
            None => QName(None, v1),
            Some(v2) => QName(Some(v1), v2),
        }
    }
}

impl<'input> fmt::Display for QName<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(prefix) => write!(f, "{}:{}", prefix, self.1),
            None => write!(f, "{}", self.1),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct anyURI<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct nonNegativeInteger<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct SUPPORT_ANY<'input>(&'input str);

type Stream<'input> = Iterator<Item=Result<XmlParserToken<'input>, XmlParserError>>;
pub trait ParseContext {
}
pub trait ParseXml {
    type Context: ParseContext;
    fn parse_xml(stream: &mut Stream, context: &mut Self::Context, closingtag: &QName) -> Self;
}
