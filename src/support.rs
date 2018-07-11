use std::marker::PhantomData;

use xmlparser::{Token as XmlToken, Tokenizer};

pub use primitives::*; // TODO: remove the pub?

#[derive(Debug,PartialEq)]
pub struct List<'input, Item>(Vec<Item>, PhantomData<&'input ()>);

pub type Stream<'input> = Box<InnerStream<'input>>;
pub struct InnerStream<'input> {
    pub(crate) index: usize,
    tokens: Vec<XmlToken<'input>>,
}

impl<'input> InnerStream<'input> {
    pub fn new(tokenizer: Tokenizer<'input>) -> InnerStream<'input> {
        InnerStream { index: 0, tokens: tokenizer.into_iter().map(|o| o.unwrap()).collect() }
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


#[derive(Default)]
pub struct ParseContext {
}

pub trait ParseXml<'input>: Sized {
    const NODE_NAME: &'static str;

    fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext, parent_context: &TParentContext) -> Option<Self>;


    fn parse_empty<TParentContext>(_parse_context: &mut ParseContext, _parent_context: &TParentContext) -> Option<Self> {
        None
    }

    fn parse_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext, parent_context: &TParentContext) -> Option<Self> {
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

    fn parse_self_xml_str<TParentContext>(input: &'input str, parse_context: &mut ParseContext, parent_context: &TParentContext) -> Option<(&'input str, Self)>;

    fn parse_xml_str<TParentContext>(input: &'input str, parse_context: &mut ParseContext, parent_context: &TParentContext) -> Option<(&'input str, Self)> {
        //println!("// Entering: {:?}", Self::NODE_NAME);
        let ret = Self::parse_self_xml_str(input, parse_context, parent_context);
        /*
        match ret {
            Some(_) => println!("// Leaving: {:?} (succeeded)", Self::NODE_NAME),
            None => println!("// Leaving: {:?} (aborted)", Self::NODE_NAME),
        }*/
        ret
    }
}
