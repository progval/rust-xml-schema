use std::fmt;
use std::marker::PhantomData;
use std::collections::HashMap;

use codegen;
use xmlparser::{Token, Tokenizer, Error, StrSpan, ElementEnd};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id<'a>(pub Option<&'a str>, pub &'a str);
impl<'a> fmt::Display for Id<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(prefix) => write!(f, "{}:{}", prefix, self.1),
            None => write!(f, "{}", self.1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document<'a> {
    pub version: Option<&'a str>,
    pub encoding: Option<&'a str>,
    pub standalone: Option<&'a str>,
    pub schema: Option<Schema<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Schema<'a> {
    pub target_namespace: &'a str,
    pub namespaces: HashMap<String, &'a str>,
    pub elements: Vec<Element<'a>>,
    pub types: HashMap<String, (Vec<Attribute<'a>>, bool, ElementType<'a>)>, // (attrs, mixed, type_)
    pub groups: HashMap<String, (Vec<Attribute<'a>>, Option<ElementType<'a>>)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element<'a> {
    pub name: Option<Id<'a>>,
    pub attrs: Vec<Attribute<'a>>,
    pub mixed: bool,
    pub type_: Option<ElementType<'a>>, // XXX is sometimes None, something about abstract elements facets blah blah blah?
}
#[derive(Debug, PartialEq, Eq)]
pub enum ElementType<'a> {
    String,
    Date,
    Any,
    Sequence(Vec<Element<'a>>),
    Ref(Id<'a>),
    Custom(Id<'a>),
    Extension(Id<'a>, Vec<Attribute<'a>>, Option<Box<ElementType<'a>>>),
    GroupRef(Id<'a>),
    Choice(Vec<Element<'a>>),
    Union(Option<Vec<Id<'a>>>, Option<Vec<Element<'a>>>),
    ComplexList(bool, Box<ElementType<'a>>), // (mixed, inner_type)
    SimpleList(Id<'a>),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Attribute<'a> {
    SmallDef {
        name: &'a str,
        type_: Option<Id<'a>>,
        default: Option<&'a str>,
    },
    LongDef {
        name: &'a str,
        default: Option<&'a str>,
        inner: Element<'a>,
    },
    Ref(Id<'a>),
    GroupRef(Id<'a>),
    Any,
}


fn split_id(id: &str) -> Id {
    let mut splitted_id = id.split(":");
    let v1 = splitted_id.next().expect(&format!("Empty id"));
    let v2 = splitted_id.next();
    match v2 {
        None => Id(None, v1),
        Some(v2) => Id(Some(v1), v2),
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
    let mut namespaces = HashMap::new();
    let mut target_namespace = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value: &str| {
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
                let (name, attrs, mixed, def) = Self::parse_complex_type(stream2, main_namespace, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), (attrs, mixed, def));
                Ok(())
            },
            "simpleType" if prefix == main_namespace => {
                let (name, attrs, def) = Self::parse_simple_type(stream2, main_namespace, (prefix, local));
                let name = name.unwrap();
                assert_eq!(schema.types.get(name), None);
                schema.types.insert(name.to_string(), (attrs, false, def));
                Ok(())
            },
            "group" if prefix == main_namespace => {
                let (name, attrs, def) = Self::parse_group_def(stream2, main_namespace, (prefix, local));
                assert_eq!(schema.groups.get(name), None);
                schema.groups.insert(name.to_string(), (attrs, Some(def)));
                Ok(())
            },
            "attributeGroup" if prefix == main_namespace => {
                let (name, attrs) = Self::parse_attribute_group_def(stream2, main_namespace, (prefix, local));
                assert_eq!(schema.groups.get(name), None);
                schema.groups.insert(name.to_string(), (attrs, None));
                Ok(())
            },
            "import" if prefix == main_namespace => {
                Self::eat_block(stream2, main_namespace, (prefix, local)); // TODO
                Ok(())
            }
            "notation" if prefix == main_namespace => {
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
                name = Some(split_id(value));
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                let id = split_id(value);
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
                type_ = Some(ElementType::Ref(split_id(value)));
                name = None;
                Ok(())
            }
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            ("", "id") => Ok(()), // TODO
            ("", "abstract") => Ok(()), // TODO
            ("", "substitutionGroup") => Ok(()), // TODO
            _ => Err(format!("Unexpected attribute while parsing element: <{}:{}: {:?}", closing_tag.0, closing_tag.1, local))
        }
    ).unwrap();

    let (type_, mixed, attrs) = if let ElementEnd::Open = element_end {
        let (newtype, mixed, newattrs) = Self::parse_subelement(stream, main_namespace, closing_tag);
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

    Element { name: name, attrs, mixed, type_ }
}

fn parse_subelement(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (Option<ElementType<'a>>, bool, Option<Vec<Attribute<'a>>>) {
    let mut type_ = None;
    let mut attrs = None;
    let mut mixed = None;
    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "complexType" => {
                assert_eq!(type_, None);
                assert_eq!(attrs, None);
                assert_eq!(mixed, None);
                let (name, attrs_def, newmixed, def) = Self::parse_complex_type(stream2, main_namespace, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                attrs = Some(attrs_def);
                mixed = Some(newmixed);
                Ok(())
            },
            "simpleType" => {
                assert_eq!(type_, None);
                assert_eq!(attrs, None);
                let (name, attrs_def, def) = Self::parse_simple_type(stream2, main_namespace, (prefix, local));
                assert_eq!(name, None);
                type_ = Some(def);
                attrs = Some(attrs_def);
                Ok(())
            },
            "annotation" => {
                Self::parse_annotation(stream2, main_namespace, (prefix, local));
                Ok(())
            },
            "key" => {
                Self::eat_block(stream2, main_namespace, (prefix, local)); // TODO
                Ok(())
            },
            _ => Err(format!("Unknown element type: {}:{}", prefix, local)),
        }
    }).unwrap();

    (type_, mixed.unwrap_or(false), attrs)
}


fn parse_complex_type(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (Option<&'a str>, Vec<Attribute<'a>>, bool, ElementType<'a>) {
    let mut name = None;
    let mut mixed = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
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
    let (attributes, type_) = Self::parse_subtype(stream, main_namespace, closing_tag).unwrap();
    let mixed = match mixed { None | Some("false") => false, Some("true") => true, _ => panic!(format!("{:?}", mixed)) };
    (name, attributes, mixed, type_.expect("complexType has no subtype."))
}

fn parse_attribute_group_def(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (&'a str, Vec<Attribute<'a>>) {
    let mut name = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
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
    let (attrs, items) = Self::parse_subtype(stream, main_namespace, closing_tag).unwrap();
    let name = name.expect("AttributeGroup def has no name");
    assert_eq!(items, None);
    (name, attrs)
}

fn parse_attribute_group_ref(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Id<'a> {
    let mut ref_ = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "ref") => {
                assert_eq!(ref_, None);
                ref_ = Some(split_id(value));
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

fn parse_group_def(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> (&'a str, Vec<Attribute<'a>>, ElementType<'a>) {
    let mut name = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
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
    let (attrs, items) = Self::parse_subtype(stream, main_namespace, closing_tag).unwrap();
    let name = name.expect("Group def has no name");
    (name, attrs, items.expect("Missing inner element type"))
}

fn parse_group_ref(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut ref_ = None;

    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "ref") => {
                assert_eq!(ref_, None);
                ref_ = Some(split_id(value));
                Ok(())
            },
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for group reference: {:?}", (prefix, local, ref_))),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Empty));
    let ref_ = ref_.expect("Group ref has no name");
    ElementType::GroupRef(ref_)
}

fn parse_subtype(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Result<(Vec<Attribute<'a>>, Option<ElementType<'a>>), String> {
    let mut inner = None;
    let mut attributes = Vec::new();

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "annotation" => {
                Self::parse_annotation(stream2, main_namespace, (prefix, local));
                Ok(())
            }
            "sequence" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_sequence(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "choice" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_choice(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "group" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_group_ref(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "attributeGroup" => {
                attributes.push(Attribute::GroupRef(Self::parse_attribute_group_ref(stream2, main_namespace, (prefix, local))));
                Ok(())
            }
            "complexContent" => {
                assert_eq!(inner, None);
                inner = Some(Self::parse_complex_content(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "attribute" => {
                attributes.push(Self::parse_attribute(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "anyAttribute" => {
                attributes.push(Self::parse_any_attribute(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            _ => Err(format!("Unknown subtype: {}:{}", prefix, local)),
        }
    })?;

    Ok((attributes, inner))
}

fn parse_elements(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Vec<Element<'a>> {
    let mut items = Vec::new();

    Self::parse_children(stream, main_namespace, closing_tag, |stream2, prefix, local| {
        assert_eq!(prefix, main_namespace);
        match local {
            "element" => {
                items.push(Self::parse_element(stream2, main_namespace, (prefix, local)));
                Ok(())
            },
            "group" => {
                let type_ = Self::parse_group_ref(stream2, main_namespace, (prefix, local));
                items.push(Element { name: None, attrs: Vec::new(), mixed: false, type_: Some(type_) });
                Ok(())
            },
            "simpleType" => {
                let (name, attrs, type_) = Self::parse_simple_type(stream2, main_namespace, (prefix, local));
                items.push(Element { name: name.map(split_id), attrs, mixed: false, type_: Some(type_) });
                Ok(())
            },
            "sequence" => {
                let type_ = Self::parse_sequence(stream2, main_namespace, (prefix, local));
                items.push(Element { name: None, attrs: Vec::new(), mixed: false, type_: Some(type_) });
                Ok(())
            },
            "choice" => {
                let type_ = Self::parse_choice(stream2, main_namespace, (prefix, local));
                items.push(Element { name: None, attrs: Vec::new(), mixed: false, type_: Some(type_) });
                Ok(())
            }
            "extension" => {
                let type_ = Self::parse_extension(stream2, main_namespace, (prefix, local));
                items.push(Element { name: None, attrs: Vec::new(), mixed: false, type_: Some(type_) });
                Ok(())
            },
            "any" => {
                let type_ = Self::parse_any(stream2, main_namespace, (prefix, local));
                items.push(Element { name: None, attrs: Vec::new(), mixed: false, type_: Some(type_) });
                Ok(())
            },
            "annotation" => {
                Self::parse_annotation(stream2, main_namespace, (prefix, local));
                Ok(())
            },
            _ => Err(format!("Unknown tag in sequence: {}:{}", prefix, local)),
        }
    }).unwrap();

    items
}

fn parse_sequence(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for sequence: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open));
    
    let items = Self::parse_elements(stream, main_namespace, closing_tag);

    ElementType::Sequence(items)
}

fn parse_choice(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for choice: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Open), "{:?}", closing_tag);

    let items = Self::parse_elements(stream, main_namespace, closing_tag);

    ElementType::Choice(items)
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
    let (attrs, inner) = Self::parse_subtype(stream, main_namespace, closing_tag).unwrap();
    ElementType::Extension(split_id(base.expect("Extension has no base.")), attrs, inner.map(Box::new))
}

fn parse_any(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "processContents") => Ok(()), // TODO
            ("", "namespace") => Ok(()), // TODO
            ("", "minOccurs") => Ok(()), // TODO
            ("", "maxOccurs") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for any: {:?}", (prefix, local, value))),
        }
    });
    assert_eq!(element_end, Ok(ElementEnd::Empty));
    ElementType::Any
}

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
    let mut default = None;
    let mut ref_ = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "name") => {
                assert_eq!(name, None);
                name = Some(value);
                Ok(())
            },
            ("", "type") => {
                assert_eq!(type_, None);
                type_ = Some(split_id(value));
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
                ref_ = Some(split_id(value));
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
            Attribute::SmallDef { name, type_, default }
        },
        (&Ok(ElementEnd::Open), None) => {
            let name = name.expect("Attribute has no name.");
            let (newtype, mixed, attrs) = Self::parse_subelement(stream, main_namespace, closing_tag);
            assert_eq!(mixed, false);
            let (attrs, type_) = match (type_, newtype) {
                (Some(t), None) => {
                    assert_eq!(attrs, None);
                    (Vec::new(), ElementType::Ref(t))
                },
                (None, Some(t)) => (attrs.unwrap(), t),
                _ => panic!("Conflict")
            };
            let inner = Element { name: None, attrs, mixed: false, type_: Some(type_) };
            Attribute::LongDef { name, default, inner }
        },
        _ => panic!(format!("<{}:{} did not expect: {:?} {:?}", closing_tag.0, closing_tag.1, element_end, ref_)),
    }
}

fn parse_any_attribute(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> Attribute<'a> {
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "namespace") => Ok(()), // TODO
            ("", "processContents") => Ok(()), // TODO
            _ => Err(format!("Unknown attribute for <{}:{}: {}:{}=\"{}\"", closing_tag.0, closing_tag.1, prefix, local, value)),
        }
    });

    assert_eq!(element_end, Ok(ElementEnd::Empty));

    Attribute::Any
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
            },
            "union" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_union(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "list" => {
                assert_eq!(type_, None);
                type_ = Some(Self::parse_list(stream2, main_namespace, (prefix, local)));
                Ok(())
            }
            "attribute" => {
                attributes.push(Self::parse_attribute(stream2, main_namespace, (prefix, local)));
                Ok(())
            },
            "annotation" => {
                Self::parse_annotation(stream2, main_namespace, (prefix, local));
                Ok(())
            },
            _ => Err(format!("Unknown simpleType type: {}:{}", prefix, local)),
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

    match element_end {
        Ok(ElementEnd::Open) => Self::eat_block(stream, main_namespace, closing_tag), // TODO
        Ok(ElementEnd::Empty) => (), // yeah, it happens sometimes
        _ => panic!(format!("{:?}", element_end)),
    }

    ElementType::Custom(split_id(name.unwrap()))
}

fn parse_union(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut member_types = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "memberTypes") => {
                assert_eq!(member_types, None);
                member_types = Some(value);
                Ok(())
            }
            _ => Err(format!("Unknown attribute for union: {:?}", (prefix, local, member_types))),
        }
    });

    let items = match element_end {
        Ok(ElementEnd::Empty) => None,
        Ok(ElementEnd::Open) => Some(Self::parse_elements(stream, main_namespace, closing_tag)),
        _ => panic!(format!("{:?}", element_end)),
    };

    let member_types = member_types.map(|s| s.split(" ").map(split_id).collect());
    ElementType::Union(member_types, items)
}

fn parse_list(stream: &mut S, main_namespace: &str, closing_tag: (&str, &str)) -> ElementType<'a> {
    let mut item_type = None;
    let element_end = Self::parse_attributes(stream, main_namespace, closing_tag, |prefix, local, value| {
        match (prefix, local) {
            ("", "itemType") => {
                assert_eq!(item_type, None);
                item_type = Some(split_id(value));
                Ok(())
            },
            _ => Err(format!("Unknown attribute for list: {:?}", (prefix, local, item_type))),
        }
    });
    if let Ok(ElementEnd::Open) = element_end {
        assert_eq!(item_type, None);
        let (newtype, mixed, newattrs) = Self::parse_subelement(stream, main_namespace, closing_tag);
        assert!(newattrs == None || newattrs == Some(Vec::new()));
        if let Some(type_) = newtype {
            return ElementType::ComplexList(mixed, Box::new(type_))
        }
    }
    else {
        assert_eq!(element_end, Ok(ElementEnd::Empty));
    }

    let item_type = item_type.unwrap();
    ElementType::SimpleList(item_type)
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
