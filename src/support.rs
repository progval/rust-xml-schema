use std::fmt;

use xmlparser::Error as XmlParserError;
use xmlparser::{Token, ElementEnd, StrSpan};

#[derive(Debug, PartialEq)]
pub struct token<'input>(StrSpan<'input>);

impl<'input> ParseXml<'input> for token<'input> {
    fn parse_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<token<'input>> {
        match stream.next() {
            Some(Ok(Token::Text(strspan))) => Some(token(strspan)),
            _ => None,
        }
    }
}
impl<'input> Default for token<'input> {
    fn default() -> Self {
        token(StrSpan::from_substr("", 0, 0))
    }
}

#[derive(Debug, PartialEq)]
pub struct NMTOKEN<'input>(StrSpan<'input>);

impl<'input> ParseXml<'input> for NMTOKEN<'input> {
    fn parse_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<NMTOKEN<'input>> {
        match stream.next() {
            Some(Ok(Token::Text(strspan))) => Some(NMTOKEN(strspan)),
            _ => None,
        }
    }
}
impl<'input> Default for NMTOKEN<'input> {
    fn default() -> Self {
        NMTOKEN(StrSpan::from_substr("", 0, 0))
    }
}

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

impl <'input> QName<'input> {
    pub fn from_strspans(prefix: StrSpan<'input>, local: StrSpan<'input>) -> QName<'input> {
        match prefix.to_str() {
            "" => QName(None, local.to_str()),
            p => QName(Some(p), local.to_str()),
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
pub struct SUPPORT_ANY<'input>(Vec<Token<'input>>); // TODO: remove, temporary

#[derive(Debug, PartialEq, Default)]
pub struct any<'input>(Vec<Token<'input>>);
impl<'input> ParseXml<'input> for any<'input> {
    fn parse_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<any<'input>> {
        let mut tag_stack = Vec::new();
        let mut tokens = Vec::new();
        let tok = stream.next().unwrap().unwrap();
        tokens.push(tok);
        match tok {
            Token::ElementStart(prefix, name) => tag_stack.push(QName::from_strspans(prefix, name)),
            _ => return None, // TODO: put it back in the stream
        }
        while tag_stack.len() > 0 {
            let tok = stream.next().unwrap().unwrap();
            tokens.push(tok);
            match tok {
                Token::ElementStart(prefix, name) => tag_stack.push(QName::from_strspans(prefix, name)),
                Token::ElementEnd(end) => {
                    match end {
                        ElementEnd::Open => (),
                        ElementEnd::Close(prefix, name) => assert_eq!(QName::from_strspans(prefix, name), tag_stack.pop().unwrap()),
                        ElementEnd::Empty => { tag_stack.pop(); () },
                    }
                }
                _ => (),
            }
        }
        Some(any(tokens))
    }
}

#[derive(Debug, PartialEq)]
pub struct XmlString<'input>(StrSpan<'input>);

impl<'input> ParseXml<'input> for XmlString<'input> {
    fn parse_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<XmlString<'input>> {
        match stream.next() {
            Some(Ok(Token::Text(strspan))) => Some(XmlString(strspan)),
            _ => None, // TODO: put it back in the stream
        }
    }
}

impl<'input> Default for XmlString<'input> {
    fn default() -> Self {
        XmlString(StrSpan::from_substr("", 0, 0))
    }
}

pub type Stream<'input> = Iterator<Item=Result<Token<'input>, XmlParserError>>;
pub trait ParseContext {
} // TODO: remove this
pub trait ParseXml<'input>: Sized {
    fn parse_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self>;
}
