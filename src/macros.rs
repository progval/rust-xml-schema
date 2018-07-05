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
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("enum ", stringify!($name));

            fn parse_empty<TParseContext, TParentContext>(_parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Self> {
                None
            }

            fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
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
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("empty group or sequence ", stringify!($name));

            fn parse_empty<TParseContext, TParentContext>(parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
                Some($name(Default::default()))
            }

            fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Self> {
                None
            }
        }
    };
    ( $name:ident, $( ( $field_name:ident, $( $field_args:tt )* ), )* ) => {
        impl<'input> ParseXml<'input> for $name<'input> {
            const NODE_NAME: &'static str = concat!("group or sequence ", stringify!($name));

            #[allow(unused_variables)]
            fn parse_empty<TParseContext, TParentContext>(parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
                Some($name {
                    $(
                        $field_name: impl_empty_element_field!(parse_context, parent_context, $($field_args)*),
                    )*
                })
            }

            #[allow(unused_variables)]
            fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
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
    ( $struct_name:ident, $name:expr, { $( ( $field_name:ident, $( $field_args:tt )* ), )* } ) => {
        impl<'input> ParseXml<'input> for $struct_name<'input> {
            const NODE_NAME: &'static str = concat!("element ", stringify!($struct_name));

            fn parse_empty<TParseContext, TParentContext>(_parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Self> {
                None
            }

            #[allow(unused_variables)]
            fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
                let tx = stream.transaction();
                let mut tok = stream.next().unwrap();
                loop {
                    match tok {
                        Token::Whitespaces(_) => (),
                        Token::Comment(_) => (),
                        Token::Text(_) => (),
                        _ => break,
                    }
                    tok = stream.next().unwrap();
                }
                match tok {
                    Token::ElementStart(prefix, name) => {
                        if name.to_str() == $name {
                            let mut attrs = HashMap::new();
                            loop {
                                let tok = stream.next().unwrap();
                                match tok {
                                    Token::Whitespaces(_) => (),
                                    Token::Comment(_) => (),
                                    Token::Text(_) => (),
                                    Token::Attribute((key_prefix, key_local), value) => {
                                        let key = QName(match key_prefix.to_str() { "" => None, s => Some(s) }, key_local.to_str());
                                        let old = attrs.insert(key, value.to_str()); assert_eq!(old, None)
                                    },
                                    Token::ElementEnd(ElementEnd::Open) => {
                                        let ret = Some($struct_name {
                                            attrs,
                                            $(
                                                $field_name: impl_element_field!(stream, tx, parse_context, parent_context, $($field_args)*),
                                            )*
                                        });
                                        let mut next_tok;
                                        loop {
                                            next_tok = stream.next();
                                            match next_tok {
                                                Some(Token::Whitespaces(_)) => (),
                                                Some(Token::Comment(_)) => (),
                                                Some(Token::Text(_)) => (),
                                                Some(Token::ElementEnd(ElementEnd::Close(prefix2, name2))) => {
                                                    assert_eq!((prefix.to_str(), name.to_str()), (prefix2.to_str(), name2.to_str()));
                                                    return ret;
                                                }
                                                _ => panic!(format!("Expected closing tag for {}:{}, got {:?}", prefix, name, next_tok)),
                                            }
                                        }
                                    },
                                    Token::ElementEnd(ElementEnd::Empty) => {
                                        return Some($struct_name {
                                            attrs,
                                            $(
                                                $field_name: impl_empty_element_field!(parse_context, parent_context, $($field_args)*),
                                            )*
                                        });
                                    },
                                    Token::ElementEnd(ElementEnd::Close(_, _)) => {
                                        tx.rollback(stream);
                                        return None
                                    },
                                    _ => panic!(format!("Expected element end for {}:{}, got {:?}", prefix, name, tok)),
                                }
                            }
                        }
                        else {
                            tx.rollback(stream);
                            None
                        }
                    },
                    Token::ElementEnd(ElementEnd::Close(_, _)) => {
                        tx.rollback(stream);
                        return None
                    },
                    _ => panic!(format!("Expected element start for {}, got {:?}", Self::NODE_NAME, tok)),
                }
            }
        }
    }
}

macro_rules! impl_element_field {
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, $type_name:ident ) => {
        try_rollback!($stream, $tx, super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context))
    };
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Option < $type_name:ident > ) => {
        super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context)
    };
    ( $stream: expr, $tx: expr, $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Vec < $type_name:ident > ) => {{
        let mut items = Vec::new();
        while let Some(item) = super::$type_mod_name::$type_name::parse_xml($stream, $parse_context, $parent_context) {
            items.push(item);
        }
        items
    }}
}


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
    ( $parse_context:expr, $parent_context:expr, $type_mod_name:ident, Vec < $type_name:ident > ) => {{
        Vec::new()
    }}
}
