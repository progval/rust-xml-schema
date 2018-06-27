use std::marker::PhantomData;
use std::collections::HashMap;
use std::num::ParseIntError;

use codegen;
use xmlparser::{Token, Tokenizer, Error, StrSpan, ElementEnd};

use support::QName;

#[derive(Debug, PartialEq)]
pub struct Document<'a> {
    pub version: Option<&'a str>,
    pub encoding: Option<&'a str>,
    pub standalone: Option<&'a str>,
    pub schema: Option<Schema<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Schema<'a> {
    pub target_namespace: &'a str,
    pub namespaces: HashMap<String, &'a str>,
    pub elements: Vec<Element<'a>>,
    pub types: HashMap<String, (Option<usize>, Option<usize>, Vec<Attribute<'a>>, bool, ElementType<'a>)>, // (min_occurs, max_occurs, attrs, mixed, type_)
    pub groups: HashMap<String, (Option<usize>, Option<usize>, Vec<Attribute<'a>>, Option<ElementType<'a>>)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Element<'a> {
    pub name: QName<'a>,
    pub attrs: Vec<Attribute<'a>>,
    pub mixed: bool,
    pub type_: Option<ElementType<'a>>, // XXX is sometimes None, something about abstract elements facets blah blah blah?
    pub min_occurs: Option<usize>,
    pub max_occurs: Option<usize>,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ElementType<'a> {
    String,
    Date,
    Sequence(Vec<(Option<usize>, Option<usize>, ElementType<'a>)>),
    Ref(QName<'a>),
    Custom(QName<'a>),
    Extension(QName<'a>, Vec<(Option<usize>, Option<usize>, ElementType<'a>)>),
    GroupRef(QName<'a>),
    Choice(Vec<(Option<usize>, Option<usize>, ElementType<'a>)>),
    Union(Option<Vec<QName<'a>>>, Option<Vec<(Option<usize>, Option<usize>, ElementType<'a>)>>),
    List(List<'a>),
    Element(Box<Element<'a>>),
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum List<'a> {
    ComplexList(bool, Box<ElementType<'a>>), // (mixed, inner_type)
    SimpleList(QName<'a>),
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Attribute<'a> {
    Def {
        name: &'a str,
        type_: Option<ElementType<'a>>,
        default: Option<&'a str>,
    },
    Ref(QName<'a>),
    GroupRef(QName<'a>),
    Any,
}

fn parse_max_occurs(s: &str) -> Result<usize, ParseIntError> {
    if s == "unbounded" {
        Ok(usize::max_value())
    }
    else {
        s.parse()
    }
}

struct SchemaParseContext<'input> {
    main_namespace: &'input str
}

pub(crate) struct Parser<S>(PhantomData<S>);

impl<'a, S: Iterator<Item=Result<Token<'a>, Error>>> Parser<S> { // To avoid that boilerplate on each function

fn parse_attributes<E, P>(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str), mut predicate: P)
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
                        let mut context = SchemaParseContext { main_namespace };
                        root.schema = Some(Self::parse_schema(stream, &mut context, (prefix.to_str(), local.to_str())));
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

fn parse_annotation(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) {
    Self::eat_block(stream, context, closing_tag) // TODO
}
fn eat_block(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) {
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

fn parse_schema(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> Schema<'a> {
    let mut namespaces = HashMap::new();
    let mut target_namespace = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value: &str| {
        match (prefix, local) {
            ("xmlns", local) => {
                namespaces.insert(local.to_string(), value);
                Ok(())
            },
            ("", "elementFormDefault") => Ok(()), // TODO
            ("", "targetNamespace") => {
                assert_eq!(target_namespace, None);
                target_namespace = Some(value);
                Ok(())
            },
            ("", "version") => Ok(()), // TODO
            ("xml", "lang") => Ok(()), // TODO
            _ => Err(format!("Unexpected token while parsing attribute in <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value))
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));

    let mut schema = Schema {
        namespaces: namespaces,
        elements: Vec::new(),
        types: HashMap::new(),
        groups: HashMap::new(),
        target_namespace: target_namespace.expect("Missing targetNamespace"),
    };

    let main_namespace_uri = schema.namespaces.get(context.main_namespace).unwrap().clone();
    assert_eq!(main_namespace_uri, "http://www.w3.org/2001/XMLSchema");

    Self::parse_children(stream, context, closing_tag, |stream2, context, prefix, local| {
        match local {
            "element" if prefix == context.main_namespace => {
                schema.elements.push(Self::parse_root_element(stream2, context, (prefix, local)));
                Ok(())
            },
            "annotation" if prefix == context.main_namespace => {
                Self::parse_annotation(stream2, context, (prefix, local));
                Ok(())
            }
            "complexType" if prefix == context.main_namespace => {
                let (min_occurs, max_occurs, name, attrs, mixed, def) = Self::parse_complex_type(stream2, context, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), (min_occurs, max_occurs, attrs, mixed, def));
                Ok(())
            },
            "simpleType" if prefix == context.main_namespace => {
                let (min_occurs, max_occurs, name, attrs, def) = Self::parse_simple_type(stream2, context, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), (min_occurs, max_occurs, attrs, false, def));
                Ok(())
            },
            "group" if prefix == context.main_namespace => {
                let (min_occurs, max_occurs, name, attrs, def) = Self::parse_group_def(stream2, context, (prefix, local));
                assert_eq!(schema.groups.get(name), None);
                schema.groups.insert(name.to_string(), (min_occurs, max_occurs, attrs, Some(def)));
                Ok(())
            },
            "attributeGroup" if prefix == context.main_namespace => {
                let (min_occurs, max_occurs, name, attrs) = Self::parse_attribute_group_def(stream2, context, (prefix, local));
                assert_eq!(schema.groups.get(name), None);
                schema.groups.insert(name.to_string(), (min_occurs, max_occurs, attrs, None));
                Ok(())
            },
            "import" if prefix == context.main_namespace => {
                Self::eat_block(stream2, context, (prefix, local)); // TODO
                Ok(())
            }
            "notation" if prefix == context.main_namespace => {
                Self::eat_block(stream2, context, (prefix, local)); // TODO
                Ok(())
            }
            _ => Err(format!("Unexpected tag while parsing schema elements: <{}:{}", prefix, local)),
        }
    }).unwrap();

    schema
}
fn parse_children<E, P>(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str), mut predicate: P)
        -> Result<(), E>
        where P: FnMut(&mut S, &mut SchemaParseContext, &'a str, &'a str) -> Result<(), E> {
    loop {
        let token = stream.next().expect("Unexpected end while parsing attributes");
        match token {
            Ok(Token::Whitespaces(_)) => (),
            Ok(Token::Comment(_)) => (),
            Ok(Token::ElementStart(prefix, local)) => predicate(stream, context, prefix.to_str(), local.to_str())?,
            Ok(Token::ElementEnd(ElementEnd::Close(prefix, local))) if (prefix.to_str(), local.to_str()) == closing_tag => return Ok(()),
            _ => panic!(format!("Unexpected token while parsing <{}:{}'s children: {:?}", closing_tag.0, closing_tag.1, token)),
        }
    }
}

fn parse_root_element(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> Element<'a> {
    let mut name = None;
    let mut type_ = None;
    let mut attrs = None;
    let mut min_occurs = None;
    let mut max_occurs = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value: &str|
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(QName::from(value));
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                let id = QName::from(value);
                // TODO: handle namespace
                type_ = Some(match id.1 {
                    "string" => ElementType::String,
                    "date" => ElementType::Date,
                    _ => ElementType::Custom(id),
                });
                Ok(())
            },
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            ("", "id") => Ok(()), // TODO
            ("", "abstract") => Ok(()), // TODO
            ("", "substitutionGroup") => Ok(()), // TODO
            _ => Err(format!("Unexpected attribute while parsing element: <{}:{}: {}:{}={:?}", closing_tag.0, closing_tag.1, prefix, local, value))
        }
    ).unwrap();

    let (type_, mixed, attrs) = if let ElementEnd::Open = element_end {
        let (newtype, mixed, newattrs) = Self::parse_subelement(stream, context, closing_tag);
        let type_ = match (type_, newtype) {
            (Some(t), None) => Some(t),
            (None, Some(t)) => Some(t),
            (None, None) => None,
            (t1, t2) => panic!(format!("Conflict {:?} {:?} {:?}", name, t1, t2)),
        };
        let attrs = match (attrs, newattrs) {
            (Some(a), None) => a,
            (None, Some(a)) => a,
            (None, None) => Vec::new(),
            _ => panic!("Conflict"),
        };
        (type_, mixed, attrs)
    }
    else {
        let type_ = type_.expect(&format!("Element {:?} has no type", name));
        (Some(type_), false, Vec::new())
    };

    Element { name: name.expect(&format!("{:?}", type_)), attrs, mixed, type_, min_occurs, max_occurs }
}

fn parse_element(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, ElementType<'a>) {
    let mut name = None;
    let mut type_ = None;
    let mut attrs = None;
    let mut min_occurs = None;
    let mut max_occurs = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value: &str|
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(QName::from(value));
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                let id = QName::from(value);
                // TODO: handle namespace
                type_ = Some(match id.1 {
                    "string" => ElementType::String,
                    "date" => ElementType::Date,
                    _ => ElementType::Custom(id),
                });
                Ok(())
            },
            ("", "ref") => {
                assert_eq!(type_, None);
                type_ = Some(ElementType::Ref(QName::from(value)));
                assert_eq!(name, None);
                Ok(())
            }
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            ("", "id") => Ok(()), // TODO
            ("", "abstract") => Ok(()), // TODO
            ("", "substitutionGroup") => Ok(()), // TODO
            _ => Err(format!("Unexpected attribute while parsing element: <{}:{}: {:?}", closing_tag.0, closing_tag.1, local))
        }
    ).unwrap();

    let (type_, mixed, attrs) = if let ElementEnd::Open = element_end {
        let (newtype, mixed, newattrs) = Self::parse_subelement(stream, context, closing_tag);
        let type_ = match (type_, newtype) {
            (Some(t), None) => Some(t),
            (None, Some(t)) => Some(t),
            (None, None) => None,
            (t1, t2) => panic!(format!("Conflict {:?} {:?} {:?}", name, t1, t2)),
        };
        let attrs = match (attrs, newattrs) {
            (Some(a), None) => a,
            (None, Some(a)) => a,
            (None, None) => Vec::new(),
            _ => panic!("Conflict"),
        };
        (type_, mixed, attrs)
    }
    else {
        let type_ = type_.expect(&format!("Element {:?} has no type", name));
        (Some(type_), false, Vec::new())
    };

    match name {
        Some(name) => (min_occurs, max_occurs, ElementType::Element(Box::new(Element { name, attrs, mixed, type_, min_occurs, max_occurs }))),
        None => (min_occurs, max_occurs, type_.unwrap()),
    }
}

fn parse_subelement(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<ElementType<'a>>, bool, Option<Vec<Attribute<'a>>>) {
    let mut type_ = None;
    let mut attrs = None;
    let mut mixed = None;
    let mut min_occurs = None;
    let mut max_occurs = None;
    Self::parse_children(stream, context, closing_tag, |stream2, context, prefix, local| {
        assert_eq!(prefix, context.main_namespace);
        match local {
            "complexType" => {
                assert_eq!(type_, None);
                assert_eq!(attrs, None);
                assert_eq!(mixed, None);
                assert_eq!(min_occurs, None);
                assert_eq!(max_occurs, None);
                let (min, max, name, attrs_def, newmixed, def) = Self::parse_complex_type(stream2, context, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                attrs = Some(attrs_def);
                mixed = Some(newmixed);
                min_occurs = Some(min);
                max_occurs = Some(max);
                Ok(())
            },
            "simpleType" => {
                assert_eq!(type_, None);
                assert_eq!(attrs, None);
                assert_eq!(min_occurs, None);
                assert_eq!(max_occurs, None);
                let (min, max, name, attrs_def, def) = Self::parse_simple_type(stream2, context, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                attrs = Some(attrs_def);
                min_occurs = Some(min);
                max_occurs = Some(max);
                Ok(())
            },
            "annotation" => {
                Self::parse_annotation(stream2, context, (prefix, local));
                Ok(())
            },
            "key" => {
                Self::eat_block(stream2, context, (prefix, local)); // TODO
                Ok(())
            },
            _ => Err(format!("Unknown element type: {}:{}", prefix, local)),
        }
    }).unwrap();

    (type_, mixed.unwrap_or(false), attrs)
}


fn parse_complex_type(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Option<&'a str>, Vec<Attribute<'a>>, bool, ElementType<'a>) {
    let mut name = None;
    let mut mixed = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            ("", "mixed") => {
                assert_eq!(mixed, None);
                mixed = Some(value);
                Ok(())
            }
            ("", "abstract") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for complexType: {:?}", (prefix, local, name))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    let (min_occurs, max_occurs, attributes, type_) = Self::parse_subtype(stream, context, closing_tag).unwrap();
    let mixed = match mixed { None | Some("false") => false, Some("true") => true, _ => panic!(format!("{:?}", mixed)) };
    (min_occurs, max_occurs, name, attributes, mixed, type_.expect("complexType has no subtype."))
}

fn parse_attribute_group_def(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, &'a str, Vec<Attribute<'a>>) {
    let mut name = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            _ => Err(format!("Unknown attribute for group definition: {:?}", (prefix, local, name))),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Open));
    let (min_occurs, max_occurs, attrs, items) = Self::parse_subtype(stream, context, closing_tag).unwrap();
    let name = name.expect("AttributeGroup def has no name");
    assert_eq!(items, None);
    (min_occurs, max_occurs, name, attrs)
}

fn parse_attribute_group_ref(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> QName<'a> {
    let mut ref_ = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "ref") => {
                assert_eq!(ref_, None);
                ref_ = Some(QName::from(value));
                Ok(())
            },
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for group reference: {:?}", (prefix, local, ref_))),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Empty));
    ref_.expect("AttributeGroup ref has no name")
}

fn parse_group_def(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, &'a str, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut name = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            _ => Err(format!("Unknown attribute for group definition: {:?}", (prefix, local, name))),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Open));
    let (min_occurs, max_occurs, attrs, items) = Self::parse_subtype(stream, context, closing_tag).unwrap();
    let name = name.expect("Group def has no name");
    (min_occurs, max_occurs, name, attrs, items.expect("Missing inner element type"))
}

fn parse_group_ref(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut ref_ = None;
    let mut min_occurs = None;
    let mut max_occurs = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "ref") => {
                assert_eq!(ref_, None);
                ref_ = Some(QName::from(value));
                Ok(())
            },
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            _ => Err(format!("Unknown attribute for group reference: {:?}", (prefix, local, ref_))),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Empty));
    let ref_ = ref_.expect("Group ref has no name");
    (min_occurs, max_occurs, Vec::new(), ElementType::GroupRef(ref_))
}

fn parse_subtype(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> Result<(Option<usize>, Option<usize>, Vec<Attribute<'a>>, Option<ElementType<'a>>), String> {
    let mut inner: Option<(Option<usize>, Option<usize>, Vec<Attribute>, ElementType)> = None;
    let mut attributes = Vec::new();

    Self::parse_children(stream, context, closing_tag, |stream2, context, prefix, local| {
        assert_eq!(prefix, context.main_namespace);
        match local {
            "annotation" => {
                Self::parse_annotation(stream2, context, (prefix, local));
                Ok(())
            }
            "sequence" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_sequence(stream2, context, (prefix, local)));
                Ok(())
            }
            "choice" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_choice(stream2, context, (prefix, local)));
                Ok(())
            }
            "group" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_group_ref(stream2, context, (prefix, local)));
                Ok(())
            }
            "attributeGroup" => {
                attributes.push(Attribute::GroupRef(Self::parse_attribute_group_ref(stream2, context, (prefix, local))));
                Ok(())
            }
            "complexContent" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_complex_content(stream2, context, (prefix, local)));
                Ok(())
            }
            "attribute" => {
                attributes.push(Self::parse_attribute(stream2, context, (prefix, local)));
                Ok(())
            }
            "anyAttribute" => {
                attributes.push(Self::parse_any_attribute(stream2, context, (prefix, local)));
                Ok(())
            }
            _ => Err(format!("Unknown subtype: {}:{}", prefix, local)),
        }
    })?;

    match inner {
        Some((min_occurs, max_occurs, attrs, type_)) => {
            if attrs.len() > 0 {
                assert_eq!(attributes, Vec::new(), "{:?}", type_);
                Ok((min_occurs, max_occurs, attrs, Some(type_)))
            }
            else {
                Ok((min_occurs, max_occurs, attributes, Some(type_)))
            }
        },
        None => Ok((None, None, attributes, None)),
    }
}

fn parse_elements(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Vec<Attribute<'a>>, Vec<(Option<usize>, Option<usize>, ElementType<'a>)>) {
    let mut items = Vec::new();
    let mut attrs = Vec::new();

    Self::parse_children(stream, context, closing_tag, |stream2, context, prefix, local| {
        assert_eq!(prefix, context.main_namespace);
        match local {
            "element" => {
                items.push(Self::parse_element(stream2, context, (prefix, local)));
                Ok(())
            },
            "group" => {
                let (min_occurs, max_occurs, a, type_) = Self::parse_group_ref(stream2, context, (prefix, local));
                items.push((min_occurs, max_occurs, type_));
                attrs.extend(a);
                Ok(())
            },
            "simpleType" => {
                let (min_occurs, max_occurs, name, attrs, type_) = Self::parse_simple_type(stream2, context, (prefix, local));
                match name {
                    Some(name) => items.push((None, None, ElementType::Element(Box::new(Element { name: QName::from(name), attrs, mixed: false, type_: Some(type_), min_occurs, max_occurs })))),
                    None => items.push((min_occurs, max_occurs, type_)),
                }
                Ok(())
            },
            "sequence" => {
                let (min_occurs, max_occurs, a, type_) = Self::parse_sequence(stream2, context, (prefix, local));
                items.push((min_occurs, max_occurs, type_));
                attrs.extend(a);
                Ok(())
            },
            "choice" => {
                let (min_occurs, max_occurs, a, type_) = Self::parse_choice(stream2, context, (prefix, local));
                items.push((min_occurs, max_occurs, type_));
                attrs.extend(a);
                Ok(())
            }
            "extension" => {
                let (min_occurs, max_occurs, a, type_) = Self::parse_extension(stream2, context, (prefix, local));
                items.push((min_occurs, max_occurs, type_));
                Ok(())
            },
            "any" => {
                let (min_occurs, max_occurs, a, type_) = Self::parse_any(stream2, context, (prefix, local));
                items.push((min_occurs, max_occurs, type_));
                attrs.extend(a);
                Ok(())
            },
            "attribute" => {
                attrs.push(Self::parse_attribute(stream2, context, (prefix, local)));
                Ok(())
            }
            "attributeGroup" => {
                attrs.push(Self::parse_attribute(stream2, context, (prefix, local)));
                Ok(())
            }
            "annotation" => {
                Self::parse_annotation(stream2, context, (prefix, local));
                Ok(())
            },
            _ => Err(format!("Unknown tag in sequence: {}:{}", prefix, local)),
        }
    }).unwrap();

    (attrs, items)
}

fn parse_sequence(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut min_occurs = None;
    let mut max_occurs = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            _ => Err(format!("Unknown attribute for sequence: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    
    let (attrs, mut items) = Self::parse_elements(stream, context, closing_tag);

    match (min_occurs, max_occurs, items.len()) {
        (None, None, 1) => {
            let (min_occurs, max_occurs, item) = items.remove(0);
            (min_occurs, max_occurs, attrs, item)
        },
        _ => (min_occurs, max_occurs, attrs, ElementType::Sequence(items)),
    }
}

fn parse_choice(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut min_occurs = None;
    let mut max_occurs = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            _ => Err(format!("Unknown attribute for choice: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open), "{:?}", closing_tag);

    let (attrs, mut items) = Self::parse_elements(stream, context, closing_tag);

    match (min_occurs, max_occurs, items.len()) {
        (None, None, 1) => {
            let (min_occurs, max_occurs, item) = items.remove(0);
            (min_occurs, max_occurs, attrs, item)
        },
        _ => (min_occurs, max_occurs, attrs, ElementType::Choice(items)),
    }
}

fn parse_extension(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut base = None;
    let mut min_occurs: Option<usize> = None;
    let mut max_occurs: Option<usize> = None;

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "base") => {
                assert_eq!(base, None);
                base = Some(value);
                Ok(())
            },
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            _ => Err(format!("Unknown attribute for complexType: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    let (attrs, items) = Self::parse_elements(stream, context, closing_tag);
    (min_occurs, max_occurs, attrs, ElementType::Extension(QName::from(base.expect("Extension has no base.")), items))
}

fn parse_any(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut min_occurs = None;
    let mut max_occurs = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "processContents") => Ok(()), // TODO
            ("", "namespace") => Ok(()), // TODO
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            _ => Err(format!("Unknown attribute for any: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Empty));
    (min_occurs, max_occurs, Vec::new(), ElementType::Ref(QName(None, "any")))
}

fn parse_complex_content(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut type_ = None;
    let mut min_occurs = None;
    let mut max_occurs = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |_, _, _| Err(()));
    assert_eq!(element_end, Ok(ElementEnd::Open));

    Self::parse_children(stream, context, closing_tag, |stream2, context, prefix, local| {
        assert_eq!(prefix, context.main_namespace);
        match local {
            "restriction" => {
                assert_eq!(type_, None);
                let (min, max, t) = Self::parse_restriction(stream2, context, (prefix, local));
                min_occurs = min;
                max_occurs = max;
                type_ = Some(t);
                Ok(())
            },
            "extension" => {
                assert_eq!(type_, None);
                let (min, max, a, t) = Self::parse_extension(stream2, context, (prefix, local));
                min_occurs = min;
                max_occurs = max;
                type_ = Some(t);
                Ok(())
            },
            _ => Err(format!("Unknown tag in complexContent: {}:{}", prefix, local)),
        }
    }).unwrap();

    (min_occurs, max_occurs, Vec::new(), type_.expect("Empty complexContent"))
}

fn parse_attribute(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> Attribute<'a> {
    let mut name = None;
    let mut type_ = None;
    let mut default = None;
    let mut ref_ = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                type_ = Some(QName::from(value));
                Ok(())
            },
            ("", "fixed") => Ok(()), // TODO
            ("", "use") => Ok(()), // TODO
            ("", "default") => {
                assert_eq!(default, None);
                default = Some(value);
                Ok(())
            },
            ("", "ref") => {
                assert_eq!(ref_, None);
                ref_ = Some(QName::from(value));
                Ok(())
            },
            _ => Err(format!("Unknown attribute for <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value)),
        }
    });

    match (&element_end, ref_) {
        (&Ok(ElementEnd::Empty), Some(ref_)) => {
            assert_eq!(name, None);
            assert_eq!(type_, None);
            Attribute::Ref(ref_)
        },
        (&Ok(ElementEnd::Empty), None) => {
            let name = name.expect("Attribute has no name.");
            Attribute::Def { name, type_: type_.map(ElementType::Ref), default }
        },
        (&Ok(ElementEnd::Open), None) => {
            let name = name.expect("Attribute has no name.");
            let (newtype, mixed, attrs) = Self::parse_subelement(stream, context, closing_tag);
            assert_eq!(mixed, false);
            let (attrs, type_) = match (type_, newtype) {
                (Some(t), None) => {
                    assert_eq!(attrs, None);
                    (Vec::new(), ElementType::Ref(t))
                },
                (None, Some(t)) => (attrs.unwrap(), t),
                _ => panic!("Conflict")
            };
            Attribute::Def { name, type_: Some(type_), default }
        },
        _ => panic!(format!("<{}:{} did not expect: {:?} {:?}", closing_tag.0, closing_tag.1, element_end, ref_)),
    }
}

fn parse_any_attribute(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> Attribute<'a> {
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "namespace") => Ok(()), // TODO
            ("", "processContents") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value)),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Empty));

    Attribute::Any
}

fn parse_simple_type(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, Option<&'a str>, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut type_ = None;
    let mut name = None;
    let mut min_occurs = None;
    let mut max_occurs = None;
    let mut attributes = Vec::new();

    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                name = Some(value);
                Ok(())
            },
            _ => Err(format!("Unknown attribute for complexType: {:?}", (prefix, local, name))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    Self::parse_children(stream, context, closing_tag, |stream2, context, prefix, local| {
        assert_eq!(prefix, context.main_namespace);
        match local {
            "restriction" => {
                assert_eq!(type_, None);
                let (min, max, t) = Self::parse_restriction(stream2, context, (prefix, local));
                min_occurs = min;
                max_occurs = max;
                type_ = Some(t);
                Ok(())
            },
            "union" => {
                assert_eq!(type_, None);
                let (min, max, t) = Self::parse_union(stream2, context, (prefix, local));
                min_occurs = min;
                max_occurs = max;
                type_ = Some(t);
                Ok(())
            }
            "list" => {
                assert_eq!(type_, None);
                let (min, max, t) = Self::parse_list(stream2, context, (prefix, local));
                min_occurs = min;
                max_occurs = max;
                type_ = Some(t);
                Ok(())
            }
            "attribute" => {
                attributes.push(Self::parse_attribute(stream2, context, (prefix, local)));
                Ok(())
            },
            "annotation" => {
                Self::parse_annotation(stream2, context, (prefix, local));
                Ok(())
            },
            _ => Err(format!("Unknown simpleType type: {}:{}", prefix, local)),
        }
    }).unwrap();

    (min_occurs, max_occurs, name, attributes, type_.expect("Missing type for complexType"))
}

fn parse_restriction(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, ElementType<'a>) {
    let mut name = None;
    let mut min_occurs = None;
    let mut max_occurs = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "base") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            }
            ("", "minOccurs") => {
                assert_eq!(min_occurs, None);
                min_occurs = Some(value.parse().unwrap());
                Ok(())
            }
            ("", "maxOccurs") => {
                assert_eq!(max_occurs, None);
                max_occurs = Some(parse_max_occurs(value).unwrap());
                Ok(())
            }
            _ => Err(format!("Unknown attribute for restriction: {:?}", (prefix, local, name))),
        }
    });

    match element_end {
        Ok(ElementEnd::Open) => Self::eat_block(stream, context, closing_tag), // TODO
        Ok(ElementEnd::Empty) => (), // yeah, it happens sometimes
        _ => panic!(format!("{:?}", element_end)),
    }

    (min_occurs, max_occurs, ElementType::Custom(QName::from(name.unwrap())))
}

fn parse_union(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, ElementType<'a>) {
    let mut member_types = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "memberTypes") => {
                assert_eq!(member_types, None);
                member_types = Some(value);
                Ok(())
            }
            _ => Err(format!("Unknown attribute for union: {:?}", (prefix, local, member_types))),
        }
    });

    let (attrs, items) = match element_end {
        Ok(ElementEnd::Empty) => (Vec::new(), None),
        Ok(ElementEnd::Open) => {
            let (attrs, items) = Self::parse_elements(stream, context, closing_tag);
            (attrs, Some(items))
        },
        _ => panic!(format!("{:?}", element_end)),
    };

    let member_types = member_types.map(|s| s.split(" ").map(QName::from).collect());
    (None, None, ElementType::Union(member_types, items))
}

fn parse_list(stream: &mut S, context: &mut SchemaParseContext, closing_tag: (&str, &str)) -> (Option<usize>, Option<usize>, ElementType<'a>) {
    let mut item_type = None;
    let element_end = Self::parse_attributes(stream, context, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "itemType") => {
                assert_eq!(item_type, None);
                item_type = Some(QName::from(value));
                Ok(())
            },
            _ => Err(format!("Unknown attribute for list: {:?}", (prefix, local, item_type))),
        }
    });
    if let Ok(ElementEnd::Open) = element_end {
        assert_eq!(item_type, None);
        let (newtype, mixed, newattrs) = Self::parse_subelement(stream, context, closing_tag);
        assert!(newattrs == None || newattrs == Some(Vec::new()));
        if let Some(type_) = newtype {
            return (None, None, ElementType::List(List::ComplexList(mixed, Box::new(type_))))
        }
    }
    else {
        assert_eq!(element_end, Ok(ElementEnd::Empty));
    }

    let item_type = item_type.unwrap();
    (None, None, ElementType::List(List::SimpleList(item_type)))
}


}
