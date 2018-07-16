use std::marker::PhantomData;
use std::collections::HashMap;
pub use std::str::FromStr;

pub use xmlparser::{Token as XmlToken, Tokenizer, ElementEnd};

pub use primitives::*; // TODO: remove the pub?
pub use names::FullName;

pub use bigfloat::BigFloatNotNaN;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Facets<'input> {
    pub min_exclusive: Option<BigFloatNotNaN>,
    pub min_inclusive: Option<BigFloatNotNaN>,
    pub max_exclusive: Option<BigFloatNotNaN>,
    pub max_inclusive: Option<BigFloatNotNaN>,
    pub total_digits: Option<u64>,
    pub fraction_digits: Option<u64>,
    pub length: Option<usize>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub enumeration: Option<Vec<&'input str>>,
    pub white_space: Option<&'input str>,
    pub pattern: Option<&'input str>,
    pub assertion: Option<&'input str>,
    pub explicit_timezone: Option<&'input str>,
}

#[derive(Debug,PartialEq)]
pub struct List<'input, Item>(Vec<Item>, PhantomData<&'input ()>);

pub type Stream<'input> = Box<InnerStream<'input>>;
pub struct InnerStream<'input> {
    pub(crate) index: usize,
    passed_prelude: bool,
    tokens: Vec<XmlToken<'input>>,
}

impl<'input> InnerStream<'input> {
    pub fn new(tokenizer: Tokenizer<'input>) -> InnerStream<'input> {
        InnerStream { index: 0, passed_prelude: false, tokens: tokenizer.into_iter().map(|o| o.unwrap()).collect() }
    }

    #[inline]
    pub fn transaction(&self) -> Transaction {
        Transaction { initial_index: self.index }
    }
}

#[must_use]
pub struct Transaction {
    initial_index: usize,
}

impl Transaction {
    #[inline]
    pub fn commit(self) {
    }

    #[inline]
    pub fn rollback(self, stream: &mut InnerStream) {
        //println!("// Rolling back {} tokens", stream.index - self.initial_index);
        stream.index = self.initial_index
    }
}

impl<'input> Iterator for InnerStream<'input> {
    type Item = XmlToken<'input>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.passed_prelude {
            self.passed_prelude = true;
            loop {
                let tok = self.next().unwrap();
                match tok {
                    XmlToken::EntityDeclaration(_, _) |
                    XmlToken::Declaration(_, _, _) |
                    XmlToken::DtdStart(_, _) |
                    XmlToken::Comment(_) => (),
                    XmlToken::DtdEnd => break,
                    _ => {
                        return Some(tok);
                    }
                }
            }
        }
        let tok = self.tokens.get(self.index);
        //println!("// Reading {:?}", tok);
        match tok {
            Some(res) => {
                self.index += 1;
                Some(res.clone())
            }
            None => None
        }
    }
}


#[derive(Clone)]
pub struct ParseContext<'input> {
    pub namespaces: HashMap<&'input str, &'input str>,
}
impl<'input> Default for ParseContext<'input> {
    fn default() -> ParseContext<'input> {
        let mut namespaces = HashMap::new();
        namespaces.insert("xmlns", "xmlns");
        namespaces.insert("xml", "xml");
        ParseContext { namespaces }
    }
}

pub trait ParseXml<'input>: Sized {
    const NODE_NAME: &'static str;

    fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self>;


    fn parse_empty<TParentContext>(_parse_context: &mut ParseContext<'input>, _parent_context: &TParentContext) -> Option<Self> {
        None
    }

    fn parse_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
        //println!("// Entering: {:?}", Self::NODE_NAME);
        let ret = Self::parse_self_xml(stream, parse_context, parent_context);
        /*
        match ret {
            Some(_) => println!("// Leaving: {:?} (succeeded)", Self::NODE_NAME),
            None => println!("// Leaving: {:?} (aborted)", Self::NODE_NAME),
        }*/
        ret
    }
}

pub trait ParseXmlStr<'input>: Sized {
    const NODE_NAME: &'static str;

    fn parse_self_xml_str<'a, TParentContext>(input: &'input str, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext, facets: &Facets<'a>) -> Option<(&'input str, Self)>;

    fn parse_xml_str<'a, TParentContext>(input: &'input str, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext, facets: &Facets<'a>) -> Option<(&'input str, Self)> {
        //println!("// Entering: {:?}", Self::NODE_NAME);
        let ret = Self::parse_self_xml_str(input, parse_context, parent_context, facets);
        /*
        match ret {
            Some(_) => println!("// Leaving: {:?} (succeeded)", Self::NODE_NAME),
            None => println!("// Leaving: {:?} (aborted)", Self::NODE_NAME),
        }*/
        ret
    }
}

impl<'input, T> ParseXml<'input> for T where T: ParseXmlStr<'input> {
    const NODE_NAME: &'static str = Self::NODE_NAME;
    fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
        match stream.next() {
            Some(XmlToken::Text(strspan)) => {
                match Self::parse_self_xml_str(strspan.to_str(), parse_context, parent_context, &Facets::default()) {
                    Some(("", out)) => Some(out),
                    Some((unparsed, _)) => None,
                    None => None,
                }
            }
            _ => None,
        }
    }
}
