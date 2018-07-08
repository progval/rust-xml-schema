use std::fmt;

use xmlparser::{Token as XmlToken, ElementEnd, StrSpan};

use support::{ParseXml, Stream};

pub const PRIMITIVE_TYPES: &[(&'static str, &'static str)] = &[
    ("token", "Token"),
    ("NMToken", "NMToken"),
    ("QName", "QName"),
    ("string", "XmlString"),
    ("positiveInteger", "PositiveInteger"),
    ("ID", "Id"),
    ("anyURI", "AnyUri"),
    ("boolean", "Boolean"),
    ("NCName", "NCName"),
    ("nonNegativeInteger", "NonNegativeInteger"),
    ];

#[derive(Debug, PartialEq)]
pub struct Token<'input>(StrSpan<'input>);

impl<'input> ParseXml<'input> for Token<'input> {
    const NODE_NAME: &'static str = "token";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Token<'input>> {
        match stream.next() {
            Some(XmlToken::Text(strspan)) => Some(Token(strspan)),
            _ => None,
        }
    }
}
impl<'input> Default for Token<'input> {
    fn default() -> Self {
        Token(StrSpan::from_substr("", 0, 0))
    }
}


pub type Nmtoken<'input> = NMToken<'input>; // TODO: remove this
#[derive(Debug, PartialEq)]
pub struct NMToken<'input>(StrSpan<'input>);

impl<'input> ParseXml<'input> for NMToken<'input> {
    const NODE_NAME: &'static str = "NMToken";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<NMToken<'input>> {
        match stream.next() {
            Some(XmlToken::Text(strspan)) => Some(NMToken(strspan)),
            _ => None,
        }
    }
}
impl<'input> Default for NMToken<'input> {
    fn default() -> Self {
        NMToken(StrSpan::from_substr("", 0, 0))
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
pub struct AnyUri<'input>(&'input str);

#[derive(Debug, PartialEq)]
pub struct AnyURIElement<'input>(StrSpan<'input>);
impl<'input> ParseXml<'input> for AnyURIElement<'input> {
    const NODE_NAME: &'static str = "AnyURIElement";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<AnyURIElement<'input>> {
        match stream.next() {
            Some(XmlToken::Text(strspan)) => Some(AnyURIElement(strspan)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct NonNegativeInteger<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct Any<'input>(pub Vec<XmlToken<'input>>);
impl<'input> ParseXml<'input> for Any<'input> {
    const NODE_NAME: &'static str = "Any";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Any<'input>> {
        let mut tag_stack = Vec::new();
        let mut tokens = Vec::new();
        loop {
            let tx = stream.transaction();
            let tok = stream.next()?;
            match tok {
                XmlToken::Whitespaces(_) => (),
                XmlToken::Comment(_) => (),
                XmlToken::Text(_) => (),
                XmlToken::ElementStart(prefix, name) => {
                    tag_stack.push(QName::from_strspans(prefix, name));
                    tokens.push(tok);
                    break
                },
                _ => {
                    tx.rollback(stream);
                    if tokens.len() > 0 {
                        return Some(Any(tokens));
                    }
                    else {
                        return None;
                    }
                }
            }
            tokens.push(tok);
        }
        while tag_stack.len() > 0 {
            let tok = stream.next().unwrap();
            tokens.push(tok);
            match tok {
                XmlToken::ElementStart(prefix, name) => tag_stack.push(QName::from_strspans(prefix, name)),
                XmlToken::ElementEnd(end) => {
                    match end {
                        ElementEnd::Open => (),
                        ElementEnd::Close(prefix, name) => assert_eq!(QName::from_strspans(prefix, name), tag_stack.pop().unwrap()),
                        ElementEnd::Empty => { tag_stack.pop(); () },
                    }
                }
                _ => (),
            }
        }
        Some(Any(tokens))
    }
}

#[derive(Debug, PartialEq)]
pub struct XmlString<'input>(StrSpan<'input>);

impl<'input> ParseXml<'input> for XmlString<'input> {
    const NODE_NAME: &'static str = "string";
    fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<XmlString<'input>> {
        match stream.next() {
            Some(XmlToken::Text(strspan)) => Some(XmlString(strspan)),
            _ => None, // TODO: put it back in the stream
        }
    }
}

impl<'input> Default for XmlString<'input> {
    fn default() -> Self {
        XmlString(StrSpan::from_substr("", 0, 0))
    }
}

