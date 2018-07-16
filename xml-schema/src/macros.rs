#[macro_export]
macro_rules! use_xsd_types {
    () => {
        pub use $crate::parser::xs;
    }
}

#[macro_export]
macro_rules! try_rollback {
    ($stream:expr, $tx:expr, $e:expr) => {
        match $e {
            Some(i) => i,
            None => {
                $tx.rollback($stream);
                return None
            }
        }
    }
}

#[macro_export]
macro_rules! impl_enum {
    ( $name:ident, $($variant_macro:ident ! ( $($variant_args: tt )* ),  )* ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("enum ", stringify!($name));

            fn parse_empty<TParentContext>(_parse_context: &mut ParseContext<'input>, _parent_context: &TParentContext) -> Option<Self> {
                None
            }

            fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
                let tx = stream.transaction();
                $(
                    match $variant_macro!($name, stream, parse_context, parent_context, $($variant_args)*) {
                        Some(x) => return Some(x),
                        None => (), // TODO: should we rollback here?
                    }
                )*

                tx.rollback(stream);
                None
            }
        }
    }
}

macro_rules! impl_singleton_variant {
    ( $enum_name:ident, $stream:expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, $type_mod_name:ident, Box < $type_name:ident > ) => {
        super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context).map(Box::new).map($enum_name::$variant_name)
    };
    ( $enum_name:ident, $stream:expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, $type_mod_name:ident, Option < Box < $type_name:ident > > ) => {
        Some(super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context).map(Box::new).map($enum_name::$variant_name))
    };
    ( $enum_name:ident, $stream:expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, $type_mod_name:ident, Vec < $type_name:ident > ) => {{
        let mut items = Vec::new();
        while let Some(item) = super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context) {
            items.push(item);
        }
        Some($enum_name::$variant_name(items))
    }}
}

macro_rules! impl_struct_variant {
    ( $enum_name:ident, $stream: expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, ) => {{
        // empty variant
        None
    }};
    ( $enum_name:ident, $stream: expr, $parse_context:expr, $parent_context:expr, $variant_name:ident, $( ( $field_name:ident, $( $field_args:tt )* ), )* ) => {{
        let mut res = None;
        loop { // single run, used for breaking
            $(
                let $field_name = match impl_struct_variant_field!($stream, $parse_context, $parent_context, $( $field_args )* ) {
                    Some(e) => e,
                    None => break,
                };
            )*
            res = Some($enum_name::$variant_name {
                $(
                    $field_name,
                )*
            });
            break;
        }
        res
    }}
}

macro_rules! impl_struct_variant_field {
    ( $stream: expr, $parse_context:expr, $parent_context:expr,  $type_mod_name:ident, Box < $type_name:ident > ) => {
        super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context).map(Box::new)
    };
    ( $stream: expr, $parse_context:expr, $parent_context:expr,  $type_mod_name:ident, Option < Box < $type_name:ident > > ) => {
        Some(super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context).map(Box::new))
    };
    ( $stream: expr, $parse_context:expr, $parent_context:expr,  $type_mod_name:ident, Vec < $type_name:ident > ) => {{
        let mut items = Vec::new();
        while let Some(item) = super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context) {
            items.push(item);
        }
        Some(items)
    }}
}

#[macro_export]
macro_rules! impl_group_or_sequence {
    ( $name:ident, ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("empty group or sequence ", stringify!($name));

            fn parse_empty<TParentContext>(parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
                Some($name(Default::default()))
            }

            fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut ParseContext<'input>, _parent_context: &TParentContext) -> Option<Self> {
                None
            }
        }
    };
    ( $name:ident, $( ( $field_name:ident, $( $field_args:tt )* ), )* ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("group or sequence ", stringify!($name));

            #[allow(unused_variables)]
            fn parse_empty<TParentContext>(parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
                Some($name {
                    $(
                        $field_name: impl_empty_element_field!(parse_context, parent_context, $($field_args)*),
                    )*
                })
            }

            #[allow(unused_variables)]
            fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
                let tx = stream.transaction();
                Some($name {
                    $(
                        $field_name: impl_element_field!(stream, tx, parse_context, parent_context, $($field_args)*),
                    )*
                })
            }
        }
    }
}

#[macro_export]
macro_rules! impl_element {
    ( $struct_name:ident, $name:expr, attributes = { $( ($attr_prefix:expr, $attr_local:expr) => $attr_name:ident : $use:ident, )* }, fields = { $( ( $field_name:ident, $( $field_args:tt )* ), )* } ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXml<'input> for $struct_name<'input> {
            const NODE_NAME: &'static str = concat!("element ", stringify!($struct_name));

            fn parse_empty<TParentContext>(_parse_context: &mut ParseContext<'input>, _parent_context: &TParentContext) -> Option<Self> {
                None
            }

            #[allow(unused_variables)]
            fn parse_self_xml<TParentContext>(stream: &mut Stream<'input>, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext) -> Option<Self> {
                use $crate::support::{XmlToken,ElementEnd};
                let mut parse_context = parse_context.clone();
                let tx = stream.transaction();
                let mut tok = stream.next().unwrap();
                loop {
                    match tok {
                        XmlToken::Whitespaces(_) => (),
                        XmlToken::Comment(_) => (),
                        XmlToken::Text(_) => (),
                        _ => break,
                    }
                    tok = stream.next().unwrap();
                }
                match tok {
                    XmlToken::ElementStart(element_prefix, name) => {
                        if name.to_str() == $name {
                            let mut attrs = HashMap::new();
                            $(
                                let mut $attr_name = None;
                            )*
                            loop {
                                let tok = stream.next().unwrap();
                                match tok {
                                    XmlToken::Whitespaces(_) => (),
                                    XmlToken::Comment(_) => (),
                                    XmlToken::Text(_) => (),
                                    XmlToken::Attribute((key_prefix, key_local), value) => {
                                        let key_prefix = key_prefix.to_str();
                                        let key_local = key_local.to_str();
                                        let value = value.to_str();
                                        match (key_prefix, key_local) {
                                            ("xmlns", l) => {
                                                parse_context.namespaces.insert(l, value);
                                                //continue; // TODO: uncomment
                                            },
                                            ("", "xmlns") => {
                                                parse_context.namespaces.insert("", value);
                                                //continue; // TODO: uncomment
                                            }
                                            _ => (),
                                        }
                                        let key_namespace = match key_prefix {
                                            "" => parse_context.namespaces.get(element_prefix.to_str()).cloned(),
                                            _ => parse_context.namespaces.get(key_prefix).cloned(),
                                        };
                                        let key = FullName::new(key_namespace, key_local);
                                        let old = attrs.insert(key, value); assert_eq!(old, None);
                                        match (key_prefix, key_local) {
                                            $(
                                                (_, $attr_local) => { // TODO: match the namespace too
                                                    match ParseXmlStr::parse_xml_str(value, &mut parse_context, parent_context, &Facets::default()) {
                                                        Some(("", value)) => {
                                                            $attr_name = Some(value)
                                                            // TODO: check for duplicates
                                                        },
                                                        Some((out, _)) =>
                                                            panic!("Unmatched data at the end of {}={:?}: {:?}", $attr_local, value, out),
                                                        None => 
                                                            panic!("Could not parse {}={:?}.", $attr_local, value),
                                                    }
                                                },
                                            )*
                                            _ => (), // TODO: unknown attribute
                                        }
                                    },
                                    XmlToken::ElementEnd(ElementEnd::Open) => {
                                        let ret = Some($struct_name {
                                            attrs,
                                            $(
                                                $attr_name: extract_attribute!($attr_name, $attr_local, $use),
                                            )*
                                            $(
                                                $field_name: impl_element_field!(stream, tx, &mut parse_context, parent_context, $($field_args)*),
                                            )*
                                        });
                                        let mut next_tok;
                                        loop {
                                            next_tok = stream.next();
                                            match next_tok {
                                                Some(XmlToken::Whitespaces(_)) => (),
                                                Some(XmlToken::Comment(_)) => (),
                                                Some(XmlToken::Text(_)) => (),
                                                Some(XmlToken::ElementEnd(ElementEnd::Close(prefix2, name2))) => {
                                                    assert_eq!((element_prefix.to_str(), name.to_str()), (prefix2.to_str(), name2.to_str()));
                                                    return ret;
                                                }
                                                _ => panic!(format!("Expected closing tag for {}:{}, got {:?}", element_prefix, name, next_tok)),
                                            }
                                        }
                                    },
                                    XmlToken::ElementEnd(ElementEnd::Empty) => {
                                        return Some($struct_name {
                                            attrs,
                                            $(
                                                $attr_name: extract_attribute!($attr_name, $attr_local, $use),
                                            )*
                                            $(
                                                $field_name: impl_empty_element_field!(&mut parse_context, parent_context, $($field_args)*),
                                            )*
                                        });
                                    },
                                    XmlToken::ElementEnd(ElementEnd::Close(_, _)) => {
                                        tx.rollback(stream);
                                        return None
                                    },
                                    _ => panic!(format!("Expected element end for {}:{}, got {:?}", element_prefix, name, tok)),
                                }
                            }
                        }
                        else {
                            tx.rollback(stream);
                            None
                        }
                    },
                    XmlToken::ElementEnd(ElementEnd::Close(_, _)) => {
                        tx.rollback(stream);
                        return None
                    },
                    _ => panic!(format!("Expected element start for {}, got {:?}", Self::NODE_NAME, tok)),
                }
            }
        }
    }
}

#[macro_export]
macro_rules! extract_attribute {
    ( $attr_name:ident, $attr_local:expr, required ) => {
        $attr_name.expect(&format!("Missing attribute {}", $attr_local))
    };
    ( $attr_name:ident, $attr_local:expr, optional ) => {
        $attr_name
    };
}

#[macro_export]
macro_rules! impl_element_field {
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, $type_name:ident ) => {
        try_rollback!($stream, $tx, super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context))
    };
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Option < $type_name:ident > ) => {
        super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context)
    };
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Vec < $type_name:ident ; min=$min:expr ; max=$max:expr ; > ) => {{
        let mut items = Vec::new();
        let min: usize = $min;
        let max: usize = $max;
        while let Some(item) = super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context) {
            items.push(item);
            if items.len() > max {
                return None;
            }
        }
        if items.len() < min {
            return None;
        }
        items
    }};
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Vec < $type_name:ident ; min=$min:expr ; > ) => {{
        let mut items = Vec::new();
        let min: usize = $min;
        while let Some(item) = super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context) {
            items.push(item);
        }
        if items.len() < min {
            return None;
        }
        items
    }};
}


#[macro_export]
macro_rules! impl_empty_element_field {
    ( $parse_context:expr, $parent_context:expr, $type_mod_name:ident, $type_name:ident ) => {
        match super::$type_mod_name::$type_name::parse_empty($parse_context, $parent_context) {
            Some(default) => default,
            None => return None,
        }
    };
    ( $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Option < $type_name:ident > ) => {
        None
    };
    ( $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Vec < $type_name:ident ; min=$min:expr ; max=$max:expr ; > ) => {{
        Vec::new()
    }};
    ( $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Vec < $type_name:ident ; min=$min:expr ; > ) => {{
        Vec::new()
    }};
}

#[macro_export]
macro_rules! impl_union {
    ( $name:ident, { $($variant_macro:ident ! ( $($variant_args: tt )* ), )* } ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXmlStr<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("union ", stringify!($name));

            fn parse_self_xml_str<'a, TParentContext>(input: &'input str, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext, facets: &Facets<'a>) -> Option<(&'input str, Self)> {
                $(
                    match $variant_macro!($name, input, parse_context, parent_context, facets, $($variant_args)*) {
                        Some((o, x)) => return Some((o, x)),
                        None => (),
                    }
                )*

                None
            }
        }
    }
}

macro_rules! impl_union_variant {
    ( $name:ident, $input:expr, $parse_context:expr, $parent_context:expr, $facets:expr, $variant_name:ident) => {
        ParseXmlStr::parse_xml_str($input, $parse_context, $parent_context, $facets)
            .map(|(o, x)| (o, $name::$variant_name(x)))
    }
}

#[macro_export]
macro_rules! impl_list {
    ( $name:ident, $item_type_mod_name:ident :: $item_type:ident ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXmlStr<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("list ", stringify!($name));

            #[allow(unused_variables)]
            fn parse_self_xml_str<'a, TParentContext>(input: &'input str, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext, facets: &Facets<'a>) -> Option<(&'input str, Self)> {
                let mut input = input;
                let mut items = Vec::new();
                while let Some((output, item)) = ParseXmlStr::parse_xml_str(input, parse_context, parent_context, facets) {
                    items.push(item);
                    if output.len() == 0 {
                        return Some(("", $name(items)));
                    }
                    if &output[0..1] != " " {
                        return None;
                    }
                    input = &output[1..];
                }
                None
            }
        }
    }
}

#[macro_export]
macro_rules! impl_simpletype_restriction {
    ( $name:ident, Facets { $( $facet_name:ident : $facet_value:expr , )* } ) => {
        #[allow(unused_imports)]
        use $crate::support::*;
        impl<'input> ParseXmlStr<'input> for $name<'input> {
            const NODE_NAME: &'static str = stringify!($name);

            #[allow(unused_variables)]
            fn parse_self_xml_str<'a, TParentContext>(input: &'input str, parse_context: &mut ParseContext<'input>, parent_context: &TParentContext, facets: &Facets<'a>) -> Option<(&'input str, Self)> {
                let mut facets = facets.clone();
                $(
                    facets.$facet_name =  $facet_value.or(facets.$facet_name);
                )*
                let (output, v) = ParseXmlStr::parse_xml_str(input, parse_context, parent_context, &facets)?;
                Some((output, $name(v)))
            }
        }
    }
}
