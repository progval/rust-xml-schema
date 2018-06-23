use std::marker::PhantomData;
use std::collections::HashMap;

use codegen;
use xmlparser::{Token, Tokenizer, Error, StrSpan, ElementEnd};

#[derive(Debug, PartialEq, Eq)]
pub struct Document<'a> {
    pub version: Option<&'a str>,
    pub encoding: Option<&'a str>,
    pub standalone: Option<&'a str>,
    pub schema: Option<Schema<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Schema<'a> {
    pub namespaces: HashMap<String, &'a str>,
    pub elements: Vec<Element<'a>>,
    pub types: HashMap<String, ElementType<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element<'a> {
    pub name: &'a str,
    pub type_: ElementType<'a>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum ElementType<'a> {
    String,
    Date,
    Sequence(Vec<Element<'a>>),
    Custom(Option<&'a str>, &'a str),
}

pub(crate) struct Parser<S>(PhantomData<S>);

impl<'a, S: Iterator<Item=Result<Token<'a>, Error>>> Parser<S> { // To avoid that boilerplate on each function

fn parse_attributes<E, P>(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str), mut predicate: P)
        -> Result<ElementEnd<'a>, E>
        where P: FnMut(&'a str, &'a str, &'a str) -> Result<(), E> {
    loop {
        let token = stream.next().expect("Unexpected end while parsing attributes");
        match token {
            Ok(Token::Whitespaces(_)) => (),
            Ok(Token::Attribute((prefix, local), value)) => predicate(prefix.to_str(), local.to_str(), value.to_str())?,
            Ok(Token::ElementEnd(end)) => return Ok(end),
            _ => panic!(format!("Unexpected token while parsing attribute in <{}:{}: {:?}", closing_tag.0, closing_tag.1, token)),
        }
    }
}


pub(crate) fn parse_document(stream: &mut S) -> Document<'a> {
    let mut root = Document { version: None, encoding: None, standalone: None, schema: None };

    loop {
        let token = stream.next();
        match token {
            None => break,
            Some(token) => {
                match token {
                    Ok(Token::Whitespaces(_)) => (),
                    Ok(Token::Declaration(version, encoding, standalone)) => {
                        assert_eq!(root.version, None);
                        assert_eq!(version.to_str(), "1.0");
                        root.version = Some(version.to_str());
                        root.encoding = encoding.map(|s| s.to_str());
                        root.standalone = standalone.map(|s| s.to_str());
                    },
                    Ok(Token::ElementStart(prefix, local)) => {
                        assert_eq!(local.to_str(), "schema");
                        let main_namespace = prefix.to_str();
                        root.schema = Some(Self::parse_schema(stream, main_namespace, (prefix.to_str(), local.to_str())));
                    },
                    _ => panic!(format!("Unexpected token at root: {:?}", token)),
                }
            }
        }
    }

    root
}

fn parse_schema(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Schema<'a> {
    let mut schema = Schema { namespaces: HashMap::new(), elements: Vec::new(), types: HashMap::new() };

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value: &str| {
        if prefix == "xmlns" {
            schema.namespaces.insert(local.to_string(), value);
            Ok(())
        }
        else {
            Err(format!("Unexpected token while parsing attribute in <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value))
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));

    let main_namespace_uri = schema.namespaces.get(main_namespace).unwrap().clone();
    assert_eq!(main_namespace_uri, "http://www.w3.org/2001/XMLSchema");

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        match local {
            "element" if prefix == main_namespace => {
                schema.elements.push(Self::parse_element(stream2, &main_namespace, (prefix, local)));
                Ok(())
            },
            "annotation" if prefix == main_namespace => {
                Self::eat_annotation(stream2, &main_namespace, (prefix, local));
                Ok(())
            }
            "complexType" => {
                let (name, def) = Self::parse_complex_type(stream2, &main_namespace, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), def);
                Ok(())
            },
            _ => Err(format!("Unexpected tag while parsing schema elements: <{}:{}", prefix, local)),
        }
    }).unwrap();

    schema
}
fn parse_children<E, P>(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str), mut predicate: P)
        -> Result<(), E>
        where P: FnMut(&mut S, &'a str, &'a str) -> Result<(), E> {
    loop {
        let token = stream.next().expect("Unexpected end while parsing attributes");
        match token {
            Ok(Token::Whitespaces(_)) => (),
            Ok(Token::ElementStart(prefix, local)) => predicate(stream, prefix.to_str(), local.to_str())?,
            Ok(Token::ElementEnd(ElementEnd::Close(prefix, local))) if (prefix.to_str(), local.to_str()) == closing_tag => return Ok(()),
            _ => panic!(format!("Unexpected token while parsing <{}:{}'s children: {:?}", closing_tag.0, closing_tag.1, token)),
        }
    }
}

fn parse_element(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Element<'a> {
    let mut name = None;
    let mut type_ = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value: &str|
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                let mut splitted_value = value.split(":");
                let v1 = splitted_value.next().expect(&format!("Empty value for <{}:{} type=", closing_tag.0, closing_tag.1));
                let v2 = splitted_value.next();
                let (value_prefix, value_name) = match v2 {
                    None => (None, v1),
                    Some(v2) => (Some(v1), v2),
                };
                type_ = Some(match value_name {
                    "string" => ElementType::String,
                    "date" => ElementType::Date,
                    _ => ElementType::Custom(value_prefix, value_name),
                });
                Ok(())
            },
            _ => Err(format!("Unexpected attribute while parsing schema elements: <{}:{}: {:?}", closing_tag.0, closing_tag.1, local))
        }
    ).unwrap();
 

    if let ElementEnd::Empty = element_end {
        let name = name.expect(&format!("Element has no name."));
        let type_ = type_.expect(&format!("Element '{}' has no type", name));

        return Element { name, type_, }
    }

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "complexType" => {
                assert_eq!(type_, None);
                let (name, def) = Self::parse_complex_type(stream2, &main_namespace, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                Ok(())
            },
            _ => Err(format!("Unknown element type: {}:{}", prefix, local)),
        }
    }).unwrap();

    let name = name.expect(&format!("Element has no name."));
    let type_ = type_.expect(&format!("Element '{}' has no type", name));
    Element { name, type_ }
}

fn parse_complex_type(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (Option<&'a str>, ElementType<'a>) {
    let mut type_ = None;
    let mut name = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                name = Some(value);
                Ok(())
            },
            _ => Err(format!("Unknown attribute for complexType: {:?}", (prefix, local, name))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "sequence" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_sequence(stream2, &main_namespace, (prefix, local)));
                Ok(())
            }
            _ => Err(format!("Unknown complexType type: {} {}", prefix, local)),
        }
    }).unwrap();

    (name, type_.expect("Missing type for complexType"))
}

fn parse_sequence(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut items = Vec::new();
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |_, _, _| Err(()));
    assert_eq!(element_end, Ok(ElementEnd::Open));

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "element" => {
                items.push(Self::parse_element(stream2, &main_namespace, (prefix, local)));
                Ok(())
            },
            _ => Err(format!("Unknown tag in sequence: {}:{}", prefix, local)),
        }
    }).unwrap();

    ElementType::Sequence(items)
}

fn eat_annotation(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) {
    let mut stack = vec![closing_tag];

    while stack.len() > 0 {
        let token = stream.next().unwrap();
        match token {
            Ok(Token::ElementStart(start, end)) => stack.push((start.to_str(), end.to_str())),
            Ok(Token::ElementEnd(ElementEnd::Empty)) => { stack.pop(); () },
            Ok(Token::ElementEnd(ElementEnd::Close(start, end))) => {
                let expected_tag = stack.pop().unwrap(); // unwrap can't panic, loop invariant
                assert_eq!((start.to_str(), end.to_str()), expected_tag);
                ()
            }
            Ok(_) => (),
            Err(e) => panic!(format!("{:?}", e)),
        }
    }
}

}
