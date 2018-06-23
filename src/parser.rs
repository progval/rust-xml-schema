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
    pub types: HashMap<String, (Vec<Attribute<'a>>, ElementType<'a>)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element<'a> {
    pub name: Option<&'a str>,
    pub attrs: Vec<Attribute<'a>>,
    pub type_: ElementType<'a>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum ElementType<'a> {
    String,
    Date,
    Sequence(Vec<Element<'a>>),
    Ref(&'a str),
    Custom(Option<&'a str>, &'a str),
    Extension((Option<&'a str>, &'a str), Vec<Attribute<'a>>, Box<ElementType<'a>>),
}
#[derive(Debug, PartialEq, Eq)]
pub struct Attribute<'a> {
    name: &'a str,
    type_: &'a str,
}


fn split_id(id: &str) -> (Option<&str>, &str) {
    let mut splitted_id = id.split(":");
    let v1 = splitted_id.next().expect(&format!("Empty id"));
    let v2 = splitted_id.next();
    match v2 {
        None => (None, v1),
        Some(v2) => (Some(v1), v2),
    }
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
            Ok(Token::Comment(_)) => (),
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
                    Ok(Token::Comment(_)) => (),
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
                    Ok(Token::DtdStart(_, _)) => (),
                    Ok(Token::DtdEnd) => (),
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
        match (prefix, local) {
            ("xmlns", local) => {
                schema.namespaces.insert(local.to_string(), value);
                Ok(())
            },
            ("", "elementFormDefault") => Ok(()), // TODO
            ("", "targetNamespace") => Ok(()), // TODO
            ("", "version") => Ok(()), // TODO
            ("xml", "lang") => Ok(()), // TODO
            _ => Err(format!("Unexpected token while parsing attribute in <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value))
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));

    let main_namespace_uri = schema.namespaces.get(main_namespace).unwrap().clone();
    assert_eq!(main_namespace_uri, "http://www.w3.org/2001/XMLSchema");

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        match local {
            "element" if prefix == main_namespace => {
                schema.elements.push(Self::parse_element(stream2, main_namespace, (prefix, local)));
                Ok(())
            },
            "annotation" if prefix == main_namespace => {
                Self::parse_annotation(stream2, &main_namespace, (prefix, local));
                Ok(())
            }
            "complexType" if prefix == main_namespace => {
                let (name, attrs, def) = Self::parse_complex_type(stream2, main_namespace, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), (attrs, def));
                Ok(())
            },
            "simpleType" if prefix == main_namespace => {
                let (name, attrs, def) = Self::parse_simple_type(stream2, main_namespace, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), (attrs, def));
                Ok(())
            },
            "import" if prefix == main_namespace => {
                Self::eat_block(stream2, main_namespace, (prefix, local)); // TODO
                Ok(())
            }
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
            Ok(Token::Comment(_)) => (),
            Ok(Token::ElementStart(prefix, local)) => predicate(stream, prefix.to_str(), local.to_str())?,
            Ok(Token::ElementEnd(ElementEnd::Close(prefix, local))) if (prefix.to_str(), local.to_str()) == closing_tag => return Ok(()),
            _ => panic!(format!("Unexpected token while parsing <{}:{}'s children: {:?}", closing_tag.0, closing_tag.1, token)),
        }
    }
}

fn parse_element(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Element<'a> {
    let mut name = None;
    let mut type_ = None;
    let mut attrs = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value: &str|
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                let (value_prefix, value_local) = split_id(value);
                type_ = Some(match value_local {
                    "string" => ElementType::String,
                    "date" => ElementType::Date,
                    _ => ElementType::Custom(value_prefix, value_local),
                });
                Ok(())
            },
            ("", "ref") => {
                assert_eq!(type_, None);
                type_ = Some(ElementType::Ref(value));
                name = Some(value); // XXX is this correct?
                Ok(())
            }
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            _ => Err(format!("Unexpected attribute while parsing element: <{}:{}: {:?}", closing_tag.0, closing_tag.1, local))
        }
    ).unwrap();
 

    if let ElementEnd::Empty = element_end {
        let name = name.expect(&format!("Element has no name (type: {:?}).", type_));
        let type_ = type_.expect(&format!("Element '{}' has no type", name));

        return Element { name: Some(name), attrs: vec![], type_, }
    }

    assert_eq!(type_, None, "element {} with type={:?} has children.", name.unwrap(), type_);

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "complexType" => {
                assert_eq!(type_, None);
                assert_eq!(attrs, None);
                let (name, attrs_def, def) = Self::parse_complex_type(stream2, &main_namespace, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                attrs = Some(attrs_def);
                Ok(())
            },
            "simpleType" => {
                assert_eq!(type_, None);
                assert_eq!(attrs, None);
                let (name, attrs_def, def) = Self::parse_simple_type(stream2, &main_namespace, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                attrs = Some(attrs_def);
                Ok(())
            },
            _ => Err(format!("Unknown element type: {}:{}", prefix, local)),
        }
    }).unwrap();

    let name = name.expect(&format!("Element has no name."));
    let type_ = type_.expect(&format!("Element '{}' has no type", name));
    let attrs = attrs.unwrap(); // Always set to Some at the same time as type_
    Element { name: Some(name), attrs, type_ }
}

fn parse_complex_type(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (Option<&'a str>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut name = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            _ => Err(format!("Unknown attribute for complexType: {:?}", (prefix, local, name))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    let (attributes, type_) = Self::parse_complex_type_children(stream, main_namespace, closing_tag).unwrap();
    (name, attributes, type_)
}

fn parse_complex_type_children(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Result<(Vec<Attribute<'a>>, ElementType<'a>), String> {
    let mut type_ = None;
    let mut attributes = Vec::new();

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "annotation" => {
                Self::parse_annotation(stream2, main_namespace, (prefix, local));
                Ok(())
            }
            "sequence" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_sequence(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "complexContent" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_complex_content(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "attribute" => {
                attributes.push(Self::parse_attribute(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            _ => Err(format!("Unknown complexType type: {}:{}", prefix, local)),
        }
    })?;

    Ok((attributes, type_.expect("Missing type for complexType")))
}

fn parse_sequence(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut items = Vec::new();
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |_, _, _| Err(()));
    assert_eq!(element_end, Ok(ElementEnd::Open));

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "element" => {
                items.push(Self::parse_element(stream2, main_namespace, (prefix, local)));
                Ok(())
            },
            "extension" => {
                let type_ = Self::parse_extension(stream2, main_namespace, (prefix, local));
                items.push(Element { name: None, attrs: Vec::new(), type_ });
                Ok(())
            },
            _ => Err(format!("Unknown tag in sequence: {}:{}", prefix, local)),
        }
    }).unwrap();

    ElementType::Sequence(items)
}

fn parse_extension(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut base = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "base") => {
                assert_eq!(base, None);
                base = Some(value);
                Ok(())
            },
            _ => Err(format!("Unknown attribute for complexType: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    let (attrs, inner) = Self::parse_complex_type_children(stream, main_namespace, closing_tag).unwrap();
    ElementType::Extension(split_id(base.expect("Extension has no base.")), attrs, Box::new(inner))}

fn parse_complex_content(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut type_ = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |_, _, _| Err(()));
    assert_eq!(element_end, Ok(ElementEnd::Open));

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "restriction" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_restriction(stream2, &main_namespace, (prefix, local)));
                Ok(())
            },
            "extension" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_extension(stream2, main_namespace, (prefix, local)));
                Ok(())
            },
            _ => Err(format!("Unknown tag in complexContent: {}:{}", prefix, local)),
        }
    }).unwrap();

    type_.expect("Empty complexContent")
}

fn parse_attribute(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Attribute<'a> {
    let mut name = None;
    let mut type_ = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                type_ = Some(value);
                Ok(())
            },
            ("", "fixed") => Ok(()), // TODO
            ("", "use") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value)),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Empty));
    let name = name.expect("Attribute has no name.");
    let type_ = type_.expect("Attribute has no type.");
    Attribute { name, type_, }
}

fn parse_simple_type(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (Option<&'a str>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut type_ = None;
    let mut name = None;
    let mut attributes = Vec::new();

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
            "restriction" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_restriction(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "attribute" => {
                attributes.push(Self::parse_attribute(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            _ => Err(format!("Unknown complexType type: {}:{}", prefix, local)),
        }
    }).unwrap();

    (name, attributes, type_.expect("Missing type for complexType"))
}

fn parse_restriction(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut name = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "base") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            }
            _ => Err(format!("Unknown attribute for restriction: {:?}", (prefix, local, name))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));

    Self::eat_block(stream, main_namespace, closing_tag); // TODO

    let (prefix, local) = split_id(name.unwrap());
    ElementType::Custom(prefix, local)
}

fn parse_annotation(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) {
    Self::eat_block(stream, main_namespace, closing_tag) // TODO
}
fn eat_block(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) {
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
