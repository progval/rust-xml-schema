#[allow(bad_style)]
pub mod UNQUAL {
    use std::marker::PhantomData;

    use support::*;

    use xmlparser::{Token, ElementEnd};

    macro_rules! try_rollback { ($stream:expr, $tx:expr, $e:expr) => { match $e { Some(i) => i, None => { $tx.rollback($stream); return None } } } }


    /////////// types


    #[derive(Debug, PartialEq, Default)]
    pub struct all<'input>(super::UNQUAL::explicitGroup<'input>);


    impl<'input> ParseXml<'input> for all<'input> {
        const NODE_NAME: &'static str = "custom all";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::explicitGroup::parse_xml(stream, parse_context, parent_context).map(all)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allNNI__item0<'input>(super::UNQUAL::NMTOKEN<'input>);


    impl<'input> ParseXml<'input> for allNNI__item0<'input> {
        const NODE_NAME: &'static str = "custom allNNI__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::NMTOKEN::parse_xml(stream, parse_context, parent_context).map(allNNI__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allNNI<'input> {
        member0: super::UNQUAL::nonNegativeInteger<'input>,
        item0: allNNI__item0<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "nonNegativeInteger")]), Some([(None, None, Custom(QName(Some("xs"), "NMTOKEN")))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0_item__choicevariant0__simpleType_e_inner<'input>(super::UNQUAL::localSimpleType<'input>);


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item__choicevariant0__simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom altType__extension__seqfield0_item__choicevariant0__simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(altType__extension__seqfield0_item__choicevariant0__simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0_item__choicevariant0__simpleType_e<'input> {
        child: altType__extension__seqfield0_item__choicevariant0__simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item__choicevariant0__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) altType__extension__seqfield0_item__choicevariant0__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(altType__extension__seqfield0_item__choicevariant0__simpleType_e {
                                        child: try_rollback!(stream, tx, altType__extension__seqfield0_item__choicevariant0__simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0_item__choicevariant0<'input>(altType__extension__seqfield0_item__choicevariant0__simpleType_e<'input>);


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element altType__extension__seqfield0_item__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            altType__extension__seqfield0_item__choicevariant0__simpleType_e::parse_xml(stream, parse_context, parent_context).map(altType__extension__seqfield0_item__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0_item__choicevariant1__complexType_e_inner<'input>(super::UNQUAL::localComplexType<'input>);


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item__choicevariant1__complexType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom altType__extension__seqfield0_item__choicevariant1__complexType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localComplexType::parse_xml(stream, parse_context, parent_context).map(altType__extension__seqfield0_item__choicevariant1__complexType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0_item__choicevariant1__complexType_e<'input> {
        child: altType__extension__seqfield0_item__choicevariant1__complexType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item__choicevariant1__complexType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) altType__extension__seqfield0_item__choicevariant1__complexType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "complexType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(altType__extension__seqfield0_item__choicevariant1__complexType_e {
                                        child: try_rollback!(stream, tx, altType__extension__seqfield0_item__choicevariant1__complexType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0_item__choicevariant1<'input>(altType__extension__seqfield0_item__choicevariant1__complexType_e<'input>);


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element altType__extension__seqfield0_item__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            altType__extension__seqfield0_item__choicevariant1__complexType_e::parse_xml(stream, parse_context, parent_context).map(altType__extension__seqfield0_item__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum altType__extension__seqfield0_item<'input> {
        choicevariant0(Box<altType__extension__seqfield0_item__choicevariant0<'input>>),
        choicevariant1(Box<altType__extension__seqfield0_item__choicevariant1<'input>>),
    }

    impl<'input> Default for altType__extension__seqfield0_item<'input> { fn default() -> altType__extension__seqfield0_item<'input> { altType__extension__seqfield0_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for altType__extension__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "choice altType__extension__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match altType__extension__seqfield0_item__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(altType__extension__seqfield0_item::choicevariant0(Box::new(r))), None => () }



            match altType__extension__seqfield0_item__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(altType__extension__seqfield0_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension__seqfield0<'input>(Option<altType__extension__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for altType__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "option altType__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(altType__extension__seqfield0(altType__extension__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType__extension<'input> {
        seqfield0: altType__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for altType__extension<'input> {
        const NODE_NAME: &'static str = "sequence altType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(altType__extension {



                seqfield0: try_rollback!(stream, tx, altType__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: altType__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(Some(0), None, Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for altType<'input> {
        const NODE_NAME: &'static str = "extension altType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(altType {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, altType__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension__seqfield0__seqfield0<'input>(Option<super::UNQUAL::annotation_e<'input>>);


    impl<'input> ParseXml<'input> for annotated__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option annotated__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(annotated__extension__seqfield0__seqfield0(super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension__seqfield0<'input> {
        seqfield0: annotated__extension__seqfield0__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation")))])


    impl<'input> ParseXml<'input> for annotated__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence annotated__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(annotated__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, annotated__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension<'input> {
        seqfield0: annotated__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation")))]))])


    impl<'input> ParseXml<'input> for annotated__extension<'input> {
        const NODE_NAME: &'static str = "sequence annotated__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(annotated__extension {



                seqfield0: try_rollback!(stream, tx, annotated__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotated__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "openAttrs"), Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation")))]))]))


    impl<'input> ParseXml<'input> for annotated<'input> {
        const NODE_NAME: &'static str = "extension annotated";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(annotated {
                BASE: try_rollback!(stream, tx, super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, annotated__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct anyType__seqfield0<'input>(Vec<any_e<'input>>);


    impl<'input> ParseXml<'input> for anyType__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec anyType__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = any_e::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(anyType__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyType<'input> {
        seqfield0: anyType__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), Ref(QName(None, "any")))])


    impl<'input> ParseXml<'input> for anyType<'input> {
        const NODE_NAME: &'static str = "sequence anyType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(anyType {



                seqfield0: try_rollback!(stream, tx, anyType__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for assertion__extension<'input> {
        const NODE_NAME: &'static str = "sequence assertion__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(assertion__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: assertion__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for assertion<'input> {
        const NODE_NAME: &'static str = "extension assertion";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(assertion {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, assertion__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input>(super::UNQUAL::localSimpleType<'input>);


    impl<'input> ParseXml<'input> for attribute__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom attribute__extension__seqfield0__seqfield0_item__simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(attribute__extension__seqfield0__seqfield0_item__simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__seqfield0__seqfield0_item__simpleType_e<'input> {
        child: attribute__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for attribute__extension__seqfield0__seqfield0_item__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attribute__extension__seqfield0__seqfield0_item__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(attribute__extension__seqfield0__seqfield0_item__simpleType_e {
                                        child: try_rollback!(stream, tx, attribute__extension__seqfield0__seqfield0_item__simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__seqfield0__seqfield0_item<'input>(attribute__extension__seqfield0__seqfield0_item__simpleType_e<'input>);


    impl<'input> ParseXml<'input> for attribute__extension__seqfield0__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "elementtype element attribute__extension__seqfield0__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            attribute__extension__seqfield0__seqfield0_item__simpleType_e::parse_xml(stream, parse_context, parent_context).map(attribute__extension__seqfield0__seqfield0_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__seqfield0__seqfield0<'input>(Option<attribute__extension__seqfield0__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for attribute__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option attribute__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(attribute__extension__seqfield0__seqfield0(attribute__extension__seqfield0__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__seqfield0<'input> {
        seqfield0: attribute__extension__seqfield0__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))])


    impl<'input> ParseXml<'input> for attribute__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence attribute__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(attribute__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, attribute__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension<'input> {
        seqfield0: attribute__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for attribute__extension<'input> {
        const NODE_NAME: &'static str = "sequence attribute__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(attribute__extension {



                seqfield0: try_rollback!(stream, tx, attribute__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: attribute__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for attribute<'input> {
        const NODE_NAME: &'static str = "extension attribute";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(attribute {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, attribute__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup__extension<'input> {
        seqfield0: super::UNQUAL::attrDecls<'input>,
    }

    // ^-- from Sequence([(None, None, GroupRef(QName(Some("xs"), "attrDecls")))])


    impl<'input> ParseXml<'input> for attributeGroup__extension<'input> {
        const NODE_NAME: &'static str = "sequence attributeGroup__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(attributeGroup__extension {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: attributeGroup__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, GroupRef(QName(Some("xs"), "attrDecls")))]))


    impl<'input> ParseXml<'input> for attributeGroup<'input> {
        const NODE_NAME: &'static str = "extension attributeGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(attributeGroup {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, attributeGroup__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroupRef<'input>(super::UNQUAL::attributeGroup<'input>);


    impl<'input> ParseXml<'input> for attributeGroupRef<'input> {
        const NODE_NAME: &'static str = "custom attributeGroupRef";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::attributeGroup::parse_xml(stream, parse_context, parent_context).map(attributeGroupRef)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList__valuetype__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for basicNamespaceList__valuetype__item0<'input> {
        const NODE_NAME: &'static str = "custom basicNamespaceList__valuetype__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(basicNamespaceList__valuetype__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList__valuetype<'input> {
        member0: super::UNQUAL::anyURI<'input>,
        item0: basicNamespaceList__valuetype__item0<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "anyURI")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList<'input>(Vec<basicNamespaceList__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Union(Some([QName(Some("xs"), "anyURI")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))))

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for blockSet__item0<'input> {
        const NODE_NAME: &'static str = "custom blockSet__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(blockSet__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1__valuetype<'input>(super::UNQUAL::derivationControl<'input>);


    impl<'input> ParseXml<'input> for blockSet__item1__valuetype<'input> {
        const NODE_NAME: &'static str = "custom blockSet__item1__valuetype";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::derivationControl::parse_xml(stream, parse_context, parent_context).map(blockSet__item1__valuetype)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1<'input>(Vec<blockSet__item1__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Custom(QName(Some("xs"), "derivationControl"))))

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet<'input> {
        item0: blockSet__item0<'input>,
        item1: blockSet__item1<'input>,
    }

    // ^-- from Union(None, Some([(None, None, Custom(QName(Some("xs"), "token"))), (None, None, List(ComplexList(false, Custom(QName(Some("xs"), "derivationControl")))))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct complexRestrictionType<'input>(super::UNQUAL::restrictionType<'input>);


    impl<'input> ParseXml<'input> for complexRestrictionType<'input> {
        const NODE_NAME: &'static str = "custom complexRestrictionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::restrictionType::parse_xml(stream, parse_context, parent_context).map(complexRestrictionType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType__extension<'input> {
        seqfield0: super::UNQUAL::complexTypeModel<'input>,
    }

    // ^-- from Sequence([(None, None, GroupRef(QName(Some("xs"), "complexTypeModel")))])


    impl<'input> ParseXml<'input> for complexType__extension<'input> {
        const NODE_NAME: &'static str = "sequence complexType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(complexType__extension {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::complexTypeModel::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: complexType__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, GroupRef(QName(Some("xs"), "complexTypeModel")))]))


    impl<'input> ParseXml<'input> for complexType<'input> {
        const NODE_NAME: &'static str = "extension complexType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(complexType {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, complexType__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct derivationControl<'input>(super::UNQUAL::NMTOKEN<'input>);


    impl<'input> ParseXml<'input> for derivationControl<'input> {
        const NODE_NAME: &'static str = "custom derivationControl";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::NMTOKEN::parse_xml(stream, parse_context, parent_context).map(derivationControl)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for derivationSet__item0<'input> {
        const NODE_NAME: &'static str = "custom derivationSet__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(derivationSet__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet__item1<'input>(Vec<super::UNQUAL::reducedDerivationControl<'input>>);

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet<'input> {
        item0: derivationSet__item0<'input>,
        item1: derivationSet__item1<'input>,
    }

    // ^-- from Union(None, Some([(None, None, Custom(QName(Some("xs"), "token"))), (None, None, List(SimpleList(QName(Some("xs"), "reducedDerivationControl"))))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e_inner<'input>(super::UNQUAL::localSimpleType<'input>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e<'input> {
        child: element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e {
                                        child: try_rollback!(stream, tx, element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0_item__choicevariant0<'input>(element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e<'input>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element element__extension__seqfield0__seqfield0_item__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            element__extension__seqfield0__seqfield0_item__choicevariant0__simpleType_e::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0__seqfield0_item__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e_inner<'input>(super::UNQUAL::localComplexType<'input>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localComplexType::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e<'input> {
        child: element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "complexType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e {
                                        child: try_rollback!(stream, tx, element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0_item__choicevariant1<'input>(element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e<'input>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element element__extension__seqfield0__seqfield0_item__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            element__extension__seqfield0__seqfield0_item__choicevariant1__complexType_e::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0__seqfield0_item__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum element__extension__seqfield0__seqfield0_item<'input> {
        choicevariant0(Box<element__extension__seqfield0__seqfield0_item__choicevariant0<'input>>),
        choicevariant1(Box<element__extension__seqfield0__seqfield0_item__choicevariant1<'input>>),
    }

    impl<'input> Default for element__extension__seqfield0__seqfield0_item<'input> { fn default() -> element__extension__seqfield0__seqfield0_item<'input> { element__extension__seqfield0__seqfield0_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "choice element__extension__seqfield0__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match element__extension__seqfield0__seqfield0_item__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(element__extension__seqfield0__seqfield0_item::choicevariant0(Box::new(r))), None => () }



            match element__extension__seqfield0__seqfield0_item__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(element__extension__seqfield0__seqfield0_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield0<'input>(Option<element__extension__seqfield0__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option element__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(element__extension__seqfield0__seqfield0(element__extension__seqfield0__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield1_item__alternative_e_inner<'input>(super::UNQUAL::altType<'input>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield1_item__alternative_e_inner<'input> {
        const NODE_NAME: &'static str = "custom element__extension__seqfield0__seqfield1_item__alternative_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::altType::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0__seqfield1_item__alternative_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield1_item__alternative_e<'input> {
        child: element__extension__seqfield0__seqfield1_item__alternative_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "alternative"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield1_item__alternative_e<'input> {
        const NODE_NAME: &'static str = "element (normal) element__extension__seqfield0__seqfield1_item__alternative_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "alternative" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(element__extension__seqfield0__seqfield1_item__alternative_e {
                                        child: try_rollback!(stream, tx, element__extension__seqfield0__seqfield1_item__alternative_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield1_item<'input>(element__extension__seqfield0__seqfield1_item__alternative_e<'input>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield1_item<'input> {
        const NODE_NAME: &'static str = "elementtype element element__extension__seqfield0__seqfield1_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            element__extension__seqfield0__seqfield1_item__alternative_e::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0__seqfield1_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield1<'input>(Vec<element__extension__seqfield0__seqfield1_item<'input>>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec element__extension__seqfield0__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = element__extension__seqfield0__seqfield1_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(element__extension__seqfield0__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0__seqfield2<'input>(Vec<super::UNQUAL::identityConstraint<'input>>);


    impl<'input> ParseXml<'input> for element__extension__seqfield0__seqfield2<'input> {
        const NODE_NAME: &'static str = "vec element__extension__seqfield0__seqfield2";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::identityConstraint::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(element__extension__seqfield0__seqfield2(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0<'input> {
        seqfield0: element__extension__seqfield0__seqfield0<'input>,
        seqfield1: element__extension__seqfield0__seqfield1<'input>,
        seqfield2: element__extension__seqfield0__seqfield2<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))])), (Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "alternative"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) })), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "identityConstraint")))])


    impl<'input> ParseXml<'input> for element__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence element__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(element__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, element__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, element__extension__seqfield0__seqfield1::parse_xml(stream, parse_context, parent_context)),



                seqfield2: try_rollback!(stream, tx, element__extension__seqfield0__seqfield2::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension<'input> {
        seqfield0: element__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))])), (Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "alternative"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) })), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "identityConstraint")))]))])


    impl<'input> ParseXml<'input> for element__extension<'input> {
        const NODE_NAME: &'static str = "sequence element__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(element__extension {



                seqfield0: try_rollback!(stream, tx, element__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: element__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Choice([(None, None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }))])), (Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "alternative"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) })), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "identityConstraint")))]))]))


    impl<'input> ParseXml<'input> for element<'input> {
        const NODE_NAME: &'static str = "extension element";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(element {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, element__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct explicitGroup<'input>(super::UNQUAL::group<'input>);


    impl<'input> ParseXml<'input> for explicitGroup<'input> {
        const NODE_NAME: &'static str = "custom explicitGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::group::parse_xml(stream, parse_context, parent_context).map(explicitGroup)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension__seqfield0__seqfield0<'input>(Option<super::UNQUAL::openContent_e<'input>>);


    impl<'input> ParseXml<'input> for extensionType__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option extensionType__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(extensionType__extension__seqfield0__seqfield0(super::UNQUAL::openContent_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension__seqfield0__seqfield1<'input>(Option<super::UNQUAL::typeDefParticle<'input>>);


    impl<'input> ParseXml<'input> for extensionType__extension__seqfield0__seqfield1<'input> {
        const NODE_NAME: &'static str = "option extensionType__extension__seqfield0__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(extensionType__extension__seqfield0__seqfield1(super::UNQUAL::typeDefParticle::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension__seqfield0<'input> {
        seqfield0: extensionType__extension__seqfield0__seqfield0<'input>,
        seqfield1: extensionType__extension__seqfield0__seqfield1<'input>,
        seqfield2: super::UNQUAL::attrDecls<'input>,
        seqfield3: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (Some(0), None, GroupRef(QName(Some("xs"), "typeDefParticle"))), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))])


    impl<'input> ParseXml<'input> for extensionType__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence extensionType__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(extensionType__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, extensionType__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, extensionType__extension__seqfield0__seqfield1::parse_xml(stream, parse_context, parent_context)),



                seqfield2: try_rollback!(stream, tx, super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)),



                seqfield3: try_rollback!(stream, tx, super::UNQUAL::assertions::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension<'input> {
        seqfield0: extensionType__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (Some(0), None, GroupRef(QName(Some("xs"), "typeDefParticle"))), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))]))])


    impl<'input> ParseXml<'input> for extensionType__extension<'input> {
        const NODE_NAME: &'static str = "sequence extensionType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(extensionType__extension {



                seqfield0: try_rollback!(stream, tx, extensionType__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: extensionType__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (Some(0), None, GroupRef(QName(Some("xs"), "typeDefParticle"))), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))]))]))


    impl<'input> ParseXml<'input> for extensionType<'input> {
        const NODE_NAME: &'static str = "extension extensionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(extensionType {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, extensionType__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct facet__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for facet__extension<'input> {
        const NODE_NAME: &'static str = "sequence facet__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(facet__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: facet__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for facet<'input> {
        const NODE_NAME: &'static str = "extension facet";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(facet {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, facet__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct formChoice<'input>(super::UNQUAL::NMTOKEN<'input>);


    impl<'input> ParseXml<'input> for formChoice<'input> {
        const NODE_NAME: &'static str = "custom formChoice";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::NMTOKEN::parse_xml(stream, parse_context, parent_context).map(formChoice)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for fullDerivationSet__item0<'input> {
        const NODE_NAME: &'static str = "custom fullDerivationSet__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(fullDerivationSet__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet__item1<'input>(Vec<super::UNQUAL::typeDerivationControl<'input>>);

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet<'input> {
        item0: fullDerivationSet__item0<'input>,
        item1: fullDerivationSet__item1<'input>,
    }

    // ^-- from Union(None, Some([(None, None, Custom(QName(Some("xs"), "token"))), (None, None, List(SimpleList(QName(Some("xs"), "typeDerivationControl"))))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct group__extension__seqfield0<'input>(Vec<super::UNQUAL::particle<'input>>);


    impl<'input> ParseXml<'input> for group__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec group__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::particle::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(group__extension__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct group__extension<'input> {
        seqfield0: group__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "particle")))])


    impl<'input> ParseXml<'input> for group__extension<'input> {
        const NODE_NAME: &'static str = "sequence group__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(group__extension {



                seqfield0: try_rollback!(stream, tx, group__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct group<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: group__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "particle")))]))


    impl<'input> ParseXml<'input> for group<'input> {
        const NODE_NAME: &'static str = "extension group";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(group {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, group__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct groupRef<'input>(super::UNQUAL::realGroup<'input>);


    impl<'input> ParseXml<'input> for groupRef<'input> {
        const NODE_NAME: &'static str = "custom groupRef";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::realGroup::parse_xml(stream, parse_context, parent_context).map(groupRef)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct intFacet<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for intFacet<'input> {
        const NODE_NAME: &'static str = "custom intFacet";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(intFacet)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase__extension__seqfield0_item__seqfield1<'input>(Vec<super::UNQUAL::field_e<'input>>);


    impl<'input> ParseXml<'input> for keybase__extension__seqfield0_item__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec keybase__extension__seqfield0_item__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::field_e::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(keybase__extension__seqfield0_item__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase__extension__seqfield0_item<'input> {
        seqfield0: super::UNQUAL::selector_e<'input>,
        seqfield1: keybase__extension__seqfield0_item__seqfield1<'input>,
    }

    // ^-- from Sequence([(None, None, Ref(QName(Some("xs"), "selector"))), (Some(1), Some(18446744073709551615), Ref(QName(Some("xs"), "field")))])


    impl<'input> ParseXml<'input> for keybase__extension__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "sequence keybase__extension__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(keybase__extension__seqfield0_item {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::selector_e::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, keybase__extension__seqfield0_item__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase__extension__seqfield0<'input>(Option<keybase__extension__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for keybase__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "option keybase__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(keybase__extension__seqfield0(keybase__extension__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase__extension<'input> {
        seqfield0: keybase__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Sequence([(None, None, Ref(QName(Some("xs"), "selector"))), (Some(1), Some(18446744073709551615), Ref(QName(Some("xs"), "field")))]))])


    impl<'input> ParseXml<'input> for keybase__extension<'input> {
        const NODE_NAME: &'static str = "sequence keybase__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(keybase__extension {



                seqfield0: try_rollback!(stream, tx, keybase__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: keybase__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(Some(0), None, Sequence([(None, None, Ref(QName(Some("xs"), "selector"))), (Some(1), Some(18446744073709551615), Ref(QName(Some("xs"), "field")))]))]))


    impl<'input> ParseXml<'input> for keybase<'input> {
        const NODE_NAME: &'static str = "extension keybase";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(keybase {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, keybase__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct localComplexType<'input>(super::UNQUAL::complexType<'input>);


    impl<'input> ParseXml<'input> for localComplexType<'input> {
        const NODE_NAME: &'static str = "custom localComplexType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::complexType::parse_xml(stream, parse_context, parent_context).map(localComplexType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct localElement<'input>(super::UNQUAL::element<'input>);


    impl<'input> ParseXml<'input> for localElement<'input> {
        const NODE_NAME: &'static str = "custom localElement";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::element::parse_xml(stream, parse_context, parent_context).map(localElement)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct localSimpleType<'input>(super::UNQUAL::simpleType<'input>);


    impl<'input> ParseXml<'input> for localSimpleType<'input> {
        const NODE_NAME: &'static str = "custom localSimpleType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::simpleType::parse_xml(stream, parse_context, parent_context).map(localSimpleType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct namedAttributeGroup<'input>(super::UNQUAL::attributeGroup<'input>);


    impl<'input> ParseXml<'input> for namedAttributeGroup<'input> {
        const NODE_NAME: &'static str = "custom namedAttributeGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::attributeGroup::parse_xml(stream, parse_context, parent_context).map(namedAttributeGroup)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct namedGroup<'input>(super::UNQUAL::realGroup<'input>);


    impl<'input> ParseXml<'input> for namedGroup<'input> {
        const NODE_NAME: &'static str = "custom namedGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::realGroup::parse_xml(stream, parse_context, parent_context).map(namedGroup)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct namespaceList<'input> {
        member0: super::UNQUAL::specialNamespaceList<'input>,
        member1: super::UNQUAL::basicNamespaceList<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "specialNamespaceList"), QName(Some("xs"), "basicNamespaceList")]), None)

    #[derive(Debug, PartialEq, Default)]
    pub struct noFixedFacet<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for noFixedFacet<'input> {
        const NODE_NAME: &'static str = "custom noFixedFacet";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(noFixedFacet)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct numFacet<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for numFacet<'input> {
        const NODE_NAME: &'static str = "custom numFacet";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(numFacet)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openAttrs<'input>(super::UNQUAL::anyType<'input>);


    impl<'input> ParseXml<'input> for openAttrs<'input> {
        const NODE_NAME: &'static str = "custom openAttrs";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::anyType::parse_xml(stream, parse_context, parent_context).map(openAttrs)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct public<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for public<'input> {
        const NODE_NAME: &'static str = "custom public";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(public)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList__valuetype__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for qnameList__valuetype__item0<'input> {
        const NODE_NAME: &'static str = "custom qnameList__valuetype__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(qnameList__valuetype__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList__valuetype<'input> {
        member0: super::UNQUAL::QName<'input>,
        item0: qnameList__valuetype__item0<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "QName")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList<'input>(Vec<qnameList__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Union(Some([QName(Some("xs"), "QName")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA__valuetype__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for qnameListA__valuetype__item0<'input> {
        const NODE_NAME: &'static str = "custom qnameListA__valuetype__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(qnameListA__valuetype__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA__valuetype<'input> {
        member0: super::UNQUAL::QName<'input>,
        item0: qnameListA__valuetype__item0<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "QName")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA<'input>(Vec<qnameListA__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Union(Some([QName(Some("xs"), "QName")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))))

    #[derive(Debug, PartialEq, Default)]
    pub struct realGroup<'input>(super::UNQUAL::group<'input>);


    impl<'input> ParseXml<'input> for realGroup<'input> {
        const NODE_NAME: &'static str = "custom realGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::group::parse_xml(stream, parse_context, parent_context).map(realGroup)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct reducedDerivationControl<'input>(super::UNQUAL::derivationControl<'input>);


    impl<'input> ParseXml<'input> for reducedDerivationControl<'input> {
        const NODE_NAME: &'static str = "custom reducedDerivationControl";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::derivationControl::parse_xml(stream, parse_context, parent_context).map(reducedDerivationControl)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0__seqfield0_item__choicevariant0__seqfield0<'input>(Option<super::UNQUAL::openContent_e<'input>>);


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0__seqfield0_item__choicevariant0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option restrictionType__extension__seqfield0__seqfield0_item__choicevariant0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(restrictionType__extension__seqfield0__seqfield0_item__choicevariant0__seqfield0(super::UNQUAL::openContent_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0__seqfield0_item__choicevariant0<'input> {
        seqfield0: restrictionType__extension__seqfield0__seqfield0_item__choicevariant0__seqfield0<'input>,
        seqfield1: super::UNQUAL::typeDefParticle<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (None, None, GroupRef(QName(Some("xs"), "typeDefParticle")))])


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0__seqfield0_item__choicevariant0<'input> {
        const NODE_NAME: &'static str = "sequence restrictionType__extension__seqfield0__seqfield0_item__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(restrictionType__extension__seqfield0__seqfield0_item__choicevariant0 {



                seqfield0: try_rollback!(stream, tx, restrictionType__extension__seqfield0__seqfield0_item__choicevariant0__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, super::UNQUAL::typeDefParticle::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum restrictionType__extension__seqfield0__seqfield0_item<'input> {
        choicevariant0(Box<restrictionType__extension__seqfield0__seqfield0_item__choicevariant0<'input>>),
        choicevariant1(Box<super::UNQUAL::simpleRestrictionModel<'input>>),
    }

    impl<'input> Default for restrictionType__extension__seqfield0__seqfield0_item<'input> { fn default() -> restrictionType__extension__seqfield0__seqfield0_item<'input> { restrictionType__extension__seqfield0__seqfield0_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (None, None, GroupRef(QName(Some("xs"), "typeDefParticle")))])), (None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))])


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "choice restrictionType__extension__seqfield0__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match restrictionType__extension__seqfield0__seqfield0_item__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(restrictionType__extension__seqfield0__seqfield0_item::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::simpleRestrictionModel::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(restrictionType__extension__seqfield0__seqfield0_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0__seqfield0<'input>(Option<restrictionType__extension__seqfield0__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option restrictionType__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(restrictionType__extension__seqfield0__seqfield0(restrictionType__extension__seqfield0__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0<'input> {
        seqfield0: restrictionType__extension__seqfield0__seqfield0<'input>,
        seqfield1: super::UNQUAL::attrDecls<'input>,
        seqfield2: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Choice([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (None, None, GroupRef(QName(Some("xs"), "typeDefParticle")))])), (None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))])), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))])


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence restrictionType__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(restrictionType__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, restrictionType__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)),



                seqfield2: try_rollback!(stream, tx, super::UNQUAL::assertions::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension<'input> {
        seqfield0: restrictionType__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Choice([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (None, None, GroupRef(QName(Some("xs"), "typeDefParticle")))])), (None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))])), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))]))])


    impl<'input> ParseXml<'input> for restrictionType__extension<'input> {
        const NODE_NAME: &'static str = "sequence restrictionType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(restrictionType__extension {



                seqfield0: try_rollback!(stream, tx, restrictionType__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: restrictionType__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Choice([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (None, None, GroupRef(QName(Some("xs"), "typeDefParticle")))])), (None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))])), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))]))]))


    impl<'input> ParseXml<'input> for restrictionType<'input> {
        const NODE_NAME: &'static str = "extension restrictionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(restrictionType {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, restrictionType__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for simpleDerivationSet__item0<'input> {
        const NODE_NAME: &'static str = "custom simpleDerivationSet__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(simpleDerivationSet__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item1__valuetype<'input>(super::UNQUAL::derivationControl<'input>);


    impl<'input> ParseXml<'input> for simpleDerivationSet__item1__valuetype<'input> {
        const NODE_NAME: &'static str = "custom simpleDerivationSet__item1__valuetype";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::derivationControl::parse_xml(stream, parse_context, parent_context).map(simpleDerivationSet__item1__valuetype)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item1<'input>(Vec<simpleDerivationSet__item1__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Custom(QName(Some("xs"), "derivationControl"))))

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet<'input> {
        item0: simpleDerivationSet__item0<'input>,
        item1: simpleDerivationSet__item1<'input>,
    }

    // ^-- from Union(None, Some([(None, None, Custom(QName(Some("xs"), "token"))), (None, None, List(ComplexList(false, Custom(QName(Some("xs"), "derivationControl")))))]))

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleExplicitGroup<'input>(super::UNQUAL::explicitGroup<'input>);


    impl<'input> ParseXml<'input> for simpleExplicitGroup<'input> {
        const NODE_NAME: &'static str = "custom simpleExplicitGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::explicitGroup::parse_xml(stream, parse_context, parent_context).map(simpleExplicitGroup)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleExtensionType<'input>(super::UNQUAL::extensionType<'input>);


    impl<'input> ParseXml<'input> for simpleExtensionType<'input> {
        const NODE_NAME: &'static str = "custom simpleExtensionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::extensionType::parse_xml(stream, parse_context, parent_context).map(simpleExtensionType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionType<'input>(super::UNQUAL::restrictionType<'input>);


    impl<'input> ParseXml<'input> for simpleRestrictionType<'input> {
        const NODE_NAME: &'static str = "custom simpleRestrictionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::restrictionType::parse_xml(stream, parse_context, parent_context).map(simpleRestrictionType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType__extension<'input> {
        seqfield0: super::UNQUAL::simpleDerivation<'input>,
    }

    // ^-- from Sequence([(None, None, GroupRef(QName(Some("xs"), "simpleDerivation")))])


    impl<'input> ParseXml<'input> for simpleType__extension<'input> {
        const NODE_NAME: &'static str = "sequence simpleType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(simpleType__extension {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::simpleDerivation::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: simpleType__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, GroupRef(QName(Some("xs"), "simpleDerivation")))]))


    impl<'input> ParseXml<'input> for simpleType<'input> {
        const NODE_NAME: &'static str = "extension simpleType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(simpleType {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, simpleType__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct specialNamespaceList<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for specialNamespaceList<'input> {
        const NODE_NAME: &'static str = "custom specialNamespaceList";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(specialNamespaceList)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelAttribute<'input>(super::UNQUAL::attribute<'input>);


    impl<'input> ParseXml<'input> for topLevelAttribute<'input> {
        const NODE_NAME: &'static str = "custom topLevelAttribute";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::attribute::parse_xml(stream, parse_context, parent_context).map(topLevelAttribute)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelComplexType<'input>(super::UNQUAL::complexType<'input>);


    impl<'input> ParseXml<'input> for topLevelComplexType<'input> {
        const NODE_NAME: &'static str = "custom topLevelComplexType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::complexType::parse_xml(stream, parse_context, parent_context).map(topLevelComplexType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelElement<'input>(super::UNQUAL::element<'input>);


    impl<'input> ParseXml<'input> for topLevelElement<'input> {
        const NODE_NAME: &'static str = "custom topLevelElement";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::element::parse_xml(stream, parse_context, parent_context).map(topLevelElement)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelSimpleType<'input>(super::UNQUAL::simpleType<'input>);


    impl<'input> ParseXml<'input> for topLevelSimpleType<'input> {
        const NODE_NAME: &'static str = "custom topLevelSimpleType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::simpleType::parse_xml(stream, parse_context, parent_context).map(topLevelSimpleType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct typeDerivationControl<'input>(super::UNQUAL::derivationControl<'input>);


    impl<'input> ParseXml<'input> for typeDerivationControl<'input> {
        const NODE_NAME: &'static str = "custom typeDerivationControl";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::derivationControl::parse_xml(stream, parse_context, parent_context).map(typeDerivationControl)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct wildcard__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for wildcard__extension<'input> {
        const NODE_NAME: &'static str = "sequence wildcard__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(wildcard__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct wildcard<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: wildcard__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for wildcard<'input> {
        const NODE_NAME: &'static str = "extension wildcard";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(wildcard {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, wildcard__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct xpathDefaultNamespace__item0<'input>(super::UNQUAL::token<'input>);


    impl<'input> ParseXml<'input> for xpathDefaultNamespace__item0<'input> {
        const NODE_NAME: &'static str = "custom xpathDefaultNamespace__item0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::token::parse_xml(stream, parse_context, parent_context).map(xpathDefaultNamespace__item0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct xpathDefaultNamespace<'input> {
        member0: super::UNQUAL::anyURI<'input>,
        item0: xpathDefaultNamespace__item0<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "anyURI")]), Some([(None, None, Custom(QName(Some("xs"), "token")))]))


    /////////// elements


    #[derive(Debug, PartialEq, Default)]
    pub struct all_e_inner<'input>(super::UNQUAL::all<'input>);


    impl<'input> ParseXml<'input> for all_e_inner<'input> {
        const NODE_NAME: &'static str = "custom all_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::all::parse_xml(stream, parse_context, parent_context).map(all_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct all_e<'input> {
        child: all_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "all"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "all"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for all_e<'input> {
        const NODE_NAME: &'static str = "element (normal) all_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "all" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(all_e {
                                        child: try_rollback!(stream, tx, all_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum annotation_e_inner__extension__seqfield0_item<'input> {
        choicevariant0(Box<super::UNQUAL::appinfo_e<'input>>),
        choicevariant1(Box<super::UNQUAL::documentation_e<'input>>),
    }

    impl<'input> Default for annotation_e_inner__extension__seqfield0_item<'input> { fn default() -> annotation_e_inner__extension__seqfield0_item<'input> { annotation_e_inner__extension__seqfield0_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "appinfo"))), (None, None, Ref(QName(Some("xs"), "documentation")))])


    impl<'input> ParseXml<'input> for annotation_e_inner__extension__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "choice annotation_e_inner__extension__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::appinfo_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(annotation_e_inner__extension__seqfield0_item::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::documentation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(annotation_e_inner__extension__seqfield0_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner__extension__seqfield0<'input>(Vec<annotation_e_inner__extension__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for annotation_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec annotation_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = annotation_e_inner__extension__seqfield0_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(annotation_e_inner__extension__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner__extension<'input> {
        seqfield0: annotation_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "appinfo"))), (None, None, Ref(QName(Some("xs"), "documentation")))]))])


    impl<'input> ParseXml<'input> for annotation_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence annotation_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(annotation_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, annotation_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotation_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "openAttrs"), Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "appinfo"))), (None, None, Ref(QName(Some("xs"), "documentation")))]))]))


    impl<'input> ParseXml<'input> for annotation_e_inner<'input> {
        const NODE_NAME: &'static str = "extension annotation_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(annotation_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, annotation_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e<'input> {
        child: annotation_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "annotation"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "appinfo"))), (None, None, Ref(QName(Some("xs"), "documentation")))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "annotation" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(annotation_e {
                                        child: try_rollback!(stream, tx, annotation_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for any_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence any_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(any_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
        EXTENSION: any_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "wildcard"), Sequence([]))


    impl<'input> ParseXml<'input> for any_e_inner<'input> {
        const NODE_NAME: &'static str = "extension any_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(any_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, any_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct any_e<'input> {
        child: any_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for any_e<'input> {
        const NODE_NAME: &'static str = "element (normal) any_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "any" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(any_e {
                                        child: try_rollback!(stream, tx, any_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for anyAttribute_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence anyAttribute_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(anyAttribute_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
        EXTENSION: anyAttribute_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "wildcard"), Sequence([]))


    impl<'input> ParseXml<'input> for anyAttribute_e_inner<'input> {
        const NODE_NAME: &'static str = "extension anyAttribute_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(anyAttribute_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, anyAttribute_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e<'input> {
        child: anyAttribute_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "anyAttribute"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for anyAttribute_e<'input> {
        const NODE_NAME: &'static str = "element (normal) anyAttribute_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "anyAttribute" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(anyAttribute_e {
                                        child: try_rollback!(stream, tx, anyAttribute_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e_inner<'input> {
        seqfield0: any_e<'input>,
    }

    // ^-- from Sequence([(None, None, Ref(QName(None, "any")))])


    impl<'input> ParseXml<'input> for appinfo_e_inner<'input> {
        const NODE_NAME: &'static str = "sequence appinfo_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(appinfo_e_inner {



                seqfield0: try_rollback!(stream, tx, any_e::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e<'input> {
        child: appinfo_e_inner<'input>,
        source: Option<super::UNQUAL::anyURI_e<'input>>,
    }

    // ^-- from Element { name: QName(None, "appinfo"), attrs: [Def { name: "source", type_: Some(Ref(QName(Some("xs"), "anyURI"))), default: None }, Any], mixed: true, type_: Some(Sequence([(None, None, Ref(QName(None, "any")))])), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for appinfo_e<'input> {
        const NODE_NAME: &'static str = "element (normal) appinfo_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "appinfo" {



                            let source: Option<super::UNQUAL::anyURI_e> = None;



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(appinfo_e {
                                        child: try_rollback!(stream, tx, appinfo_e_inner::parse_xml(stream, parse_context, parent_context)),



                                        source,



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion_e_inner<'input>(super::UNQUAL::assertion<'input>);


    impl<'input> ParseXml<'input> for assertion_e_inner<'input> {
        const NODE_NAME: &'static str = "custom assertion_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::assertion::parse_xml(stream, parse_context, parent_context).map(assertion_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion_e<'input> {
        child: assertion_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "assertion"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for assertion_e<'input> {
        const NODE_NAME: &'static str = "element (normal) assertion_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "assertion" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(assertion_e {
                                        child: try_rollback!(stream, tx, assertion_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute_e_inner<'input>(super::UNQUAL::topLevelAttribute<'input>);


    impl<'input> ParseXml<'input> for attribute_e_inner<'input> {
        const NODE_NAME: &'static str = "custom attribute_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelAttribute::parse_xml(stream, parse_context, parent_context).map(attribute_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute_e<'input> {
        child: attribute_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "attribute"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelAttribute"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for attribute_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attribute_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "attribute" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(attribute_e {
                                        child: try_rollback!(stream, tx, attribute_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup_e_inner<'input>(super::UNQUAL::namedAttributeGroup<'input>);


    impl<'input> ParseXml<'input> for attributeGroup_e_inner<'input> {
        const NODE_NAME: &'static str = "custom attributeGroup_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::namedAttributeGroup::parse_xml(stream, parse_context, parent_context).map(attributeGroup_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup_e<'input> {
        child: attributeGroup_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "attributeGroup"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedAttributeGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for attributeGroup_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attributeGroup_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "attributeGroup" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(attributeGroup_e {
                                        child: try_rollback!(stream, tx, attributeGroup_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct choice_e_inner<'input>(super::UNQUAL::explicitGroup<'input>);


    impl<'input> ParseXml<'input> for choice_e_inner<'input> {
        const NODE_NAME: &'static str = "custom choice_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::explicitGroup::parse_xml(stream, parse_context, parent_context).map(choice_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct choice_e<'input> {
        child: choice_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "choice"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for choice_e<'input> {
        const NODE_NAME: &'static str = "element (normal) choice_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "choice" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(choice_e {
                                        child: try_rollback!(stream, tx, choice_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner<'input>(super::UNQUAL::complexRestrictionType<'input>);


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner<'input> {
        const NODE_NAME: &'static str = "custom complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::complexRestrictionType::parse_xml(stream, parse_context, parent_context).map(complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e<'input> {
        child: complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "restriction" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e {
                                        child: try_rollback!(stream, tx, complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension__seqfield0__choicevariant0<'input>(complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e<'input>);


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element complexContent_e_inner__extension__seqfield0__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            complexContent_e_inner__extension__seqfield0__choicevariant0__restriction_e::parse_xml(stream, parse_context, parent_context).map(complexContent_e_inner__extension__seqfield0__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner<'input>(super::UNQUAL::extensionType<'input>);


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner<'input> {
        const NODE_NAME: &'static str = "custom complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::extensionType::parse_xml(stream, parse_context, parent_context).map(complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e<'input> {
        child: complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "extension" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e {
                                        child: try_rollback!(stream, tx, complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension__seqfield0__choicevariant1<'input>(complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e<'input>);


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element complexContent_e_inner__extension__seqfield0__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            complexContent_e_inner__extension__seqfield0__choicevariant1__extension_e::parse_xml(stream, parse_context, parent_context).map(complexContent_e_inner__extension__seqfield0__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum complexContent_e_inner__extension__seqfield0<'input> {
        choicevariant0(Box<complexContent_e_inner__extension__seqfield0__choicevariant0<'input>>),
        choicevariant1(Box<complexContent_e_inner__extension__seqfield0__choicevariant1<'input>>),
    }

    impl<'input> Default for complexContent_e_inner__extension__seqfield0<'input> { fn default() -> complexContent_e_inner__extension__seqfield0<'input> { complexContent_e_inner__extension__seqfield0::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "choice complexContent_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match complexContent_e_inner__extension__seqfield0__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexContent_e_inner__extension__seqfield0::choicevariant0(Box::new(r))), None => () }



            match complexContent_e_inner__extension__seqfield0__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexContent_e_inner__extension__seqfield0::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner__extension<'input> {
        seqfield0: complexContent_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence complexContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(complexContent_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, complexContent_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: complexContent_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for complexContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension complexContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(complexContent_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, complexContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e<'input> {
        child: complexContent_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "complexContent"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for complexContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "complexContent" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(complexContent_e {
                                        child: try_rollback!(stream, tx, complexContent_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType_e_inner<'input>(super::UNQUAL::topLevelComplexType<'input>);


    impl<'input> ParseXml<'input> for complexType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom complexType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelComplexType::parse_xml(stream, parse_context, parent_context).map(complexType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType_e<'input> {
        child: complexType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "complexType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelComplexType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for complexType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "complexType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(complexType_e {
                                        child: try_rollback!(stream, tx, complexType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e_inner<'input>(super::UNQUAL::wildcard<'input>);


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e_inner<'input> {
        const NODE_NAME: &'static str = "custom defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context).map(defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e<'input> {
        child: defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e<'input> {
        const NODE_NAME: &'static str = "element (normal) defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "any" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e {
                                        child: try_rollback!(stream, tx, defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension__seqfield0__seqfield0<'input>(defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e<'input>);


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "elementtype element defaultOpenContent_e_inner__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            defaultOpenContent_e_inner__extension__seqfield0__seqfield0__any_e::parse_xml(stream, parse_context, parent_context).map(defaultOpenContent_e_inner__extension__seqfield0__seqfield0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension__seqfield0<'input> {
        seqfield0: defaultOpenContent_e_inner__extension__seqfield0__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence defaultOpenContent_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(defaultOpenContent_e_inner__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, defaultOpenContent_e_inner__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension<'input> {
        seqfield0: defaultOpenContent_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(None, None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence defaultOpenContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(defaultOpenContent_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, defaultOpenContent_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: defaultOpenContent_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(None, None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension defaultOpenContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(defaultOpenContent_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, defaultOpenContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e<'input> {
        child: defaultOpenContent_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "defaultOpenContent"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(None, None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for defaultOpenContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) defaultOpenContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "defaultOpenContent" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(defaultOpenContent_e {
                                        child: try_rollback!(stream, tx, defaultOpenContent_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e_inner<'input> {
        seqfield0: any_e<'input>,
    }

    // ^-- from Sequence([(None, None, Ref(QName(None, "any")))])


    impl<'input> ParseXml<'input> for documentation_e_inner<'input> {
        const NODE_NAME: &'static str = "sequence documentation_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(documentation_e_inner {



                seqfield0: try_rollback!(stream, tx, any_e::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e<'input> {
        child: documentation_e_inner<'input>,
        source: Option<super::UNQUAL::anyURI_e<'input>>,
    }

    // ^-- from Element { name: QName(None, "documentation"), attrs: [Def { name: "source", type_: Some(Ref(QName(Some("xs"), "anyURI"))), default: None }, Ref(QName(Some("xml"), "lang")), Any], mixed: true, type_: Some(Sequence([(None, None, Ref(QName(None, "any")))])), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for documentation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) documentation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "documentation" {



                            let source: Option<super::UNQUAL::anyURI_e> = None;



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(documentation_e {
                                        child: try_rollback!(stream, tx, documentation_e_inner::parse_xml(stream, parse_context, parent_context)),



                                        source,



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element_e_inner<'input>(super::UNQUAL::topLevelElement<'input>);


    impl<'input> ParseXml<'input> for element_e_inner<'input> {
        const NODE_NAME: &'static str = "custom element_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelElement::parse_xml(stream, parse_context, parent_context).map(element_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element_e<'input> {
        child: element_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelElement"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for element_e<'input> {
        const NODE_NAME: &'static str = "element (normal) element_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "element" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(element_e {
                                        child: try_rollback!(stream, tx, element_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct enumeration_e_inner<'input>(super::UNQUAL::noFixedFacet<'input>);


    impl<'input> ParseXml<'input> for enumeration_e_inner<'input> {
        const NODE_NAME: &'static str = "custom enumeration_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::noFixedFacet::parse_xml(stream, parse_context, parent_context).map(enumeration_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct enumeration_e<'input> {
        child: enumeration_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "enumeration"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for enumeration_e<'input> {
        const NODE_NAME: &'static str = "element (normal) enumeration_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "enumeration" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(enumeration_e {
                                        child: try_rollback!(stream, tx, enumeration_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitTimezone_e_inner<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for explicitTimezone_e_inner<'input> {
        const NODE_NAME: &'static str = "custom explicitTimezone_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(explicitTimezone_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitTimezone_e<'input> {
        child: explicitTimezone_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "explicitTimezone"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for explicitTimezone_e<'input> {
        const NODE_NAME: &'static str = "element (normal) explicitTimezone_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "explicitTimezone" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(explicitTimezone_e {
                                        child: try_rollback!(stream, tx, explicitTimezone_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet_e<'input>(PhantomData<&'input ()>);


    impl<'input> ParseXml<'input> for facet_e<'input> {
        const NODE_NAME: &'static str = "element (empty) facet_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, _parse_context: &mut TParseContext, _parent_context: &TParentContext) -> Option<Self> {
            Some(facet_e(Default::default()))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for field_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence field_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(field_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: field_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for field_e_inner<'input> {
        const NODE_NAME: &'static str = "extension field_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(field_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, field_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct field_e<'input> {
        child: field_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "field"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for field_e<'input> {
        const NODE_NAME: &'static str = "element (normal) field_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "field" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(field_e {
                                        child: try_rollback!(stream, tx, field_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct fractionDigits_e_inner<'input>(super::UNQUAL::numFacet<'input>);


    impl<'input> ParseXml<'input> for fractionDigits_e_inner<'input> {
        const NODE_NAME: &'static str = "custom fractionDigits_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(fractionDigits_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct fractionDigits_e<'input> {
        child: fractionDigits_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "fractionDigits"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for fractionDigits_e<'input> {
        const NODE_NAME: &'static str = "element (normal) fractionDigits_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "fractionDigits" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(fractionDigits_e {
                                        child: try_rollback!(stream, tx, fractionDigits_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct group_e_inner<'input>(super::UNQUAL::namedGroup<'input>);


    impl<'input> ParseXml<'input> for group_e_inner<'input> {
        const NODE_NAME: &'static str = "custom group_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::namedGroup::parse_xml(stream, parse_context, parent_context).map(group_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct group_e<'input> {
        child: group_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for group_e<'input> {
        const NODE_NAME: &'static str = "element (normal) group_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "group" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(group_e {
                                        child: try_rollback!(stream, tx, group_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for import_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence import_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(import_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: import_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for import_e_inner<'input> {
        const NODE_NAME: &'static str = "extension import_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(import_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, import_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct import_e<'input> {
        child: import_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "import"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for import_e<'input> {
        const NODE_NAME: &'static str = "element (normal) import_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "import" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(import_e {
                                        child: try_rollback!(stream, tx, import_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for include_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence include_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(include_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: include_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for include_e_inner<'input> {
        const NODE_NAME: &'static str = "extension include_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(include_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, include_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct include_e<'input> {
        child: include_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "include"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for include_e<'input> {
        const NODE_NAME: &'static str = "element (normal) include_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "include" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(include_e {
                                        child: try_rollback!(stream, tx, include_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct key_e_inner<'input>(super::UNQUAL::keybase<'input>);


    impl<'input> ParseXml<'input> for key_e_inner<'input> {
        const NODE_NAME: &'static str = "custom key_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::keybase::parse_xml(stream, parse_context, parent_context).map(key_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct key_e<'input> {
        child: key_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "key"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for key_e<'input> {
        const NODE_NAME: &'static str = "element (normal) key_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "key" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(key_e {
                                        child: try_rollback!(stream, tx, key_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for keyref_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence keyref_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(keyref_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e_inner<'input> {
        BASE: super::UNQUAL::keybase<'input>,
        EXTENSION: keyref_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "keybase"), Sequence([]))


    impl<'input> ParseXml<'input> for keyref_e_inner<'input> {
        const NODE_NAME: &'static str = "extension keyref_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(keyref_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::keybase::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, keyref_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e<'input> {
        child: keyref_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "keyref"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "keybase"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for keyref_e<'input> {
        const NODE_NAME: &'static str = "element (normal) keyref_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "keyref" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(keyref_e {
                                        child: try_rollback!(stream, tx, keyref_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct length_e_inner<'input>(super::UNQUAL::numFacet<'input>);


    impl<'input> ParseXml<'input> for length_e_inner<'input> {
        const NODE_NAME: &'static str = "custom length_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(length_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct length_e<'input> {
        child: length_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "length"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for length_e<'input> {
        const NODE_NAME: &'static str = "element (normal) length_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "length" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(length_e {
                                        child: try_rollback!(stream, tx, length_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input>(super::UNQUAL::localSimpleType<'input>);


    impl<'input> ParseXml<'input> for list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e<'input> {
        child: list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e {
                                        child: try_rollback!(stream, tx, list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__seqfield0__seqfield0_item<'input>(list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e<'input>);


    impl<'input> ParseXml<'input> for list_e_inner__extension__seqfield0__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "elementtype element list_e_inner__extension__seqfield0__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            list_e_inner__extension__seqfield0__seqfield0_item__simpleType_e::parse_xml(stream, parse_context, parent_context).map(list_e_inner__extension__seqfield0__seqfield0_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__seqfield0__seqfield0<'input>(Option<list_e_inner__extension__seqfield0__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for list_e_inner__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option list_e_inner__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(list_e_inner__extension__seqfield0__seqfield0(list_e_inner__extension__seqfield0__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__seqfield0<'input> {
        seqfield0: list_e_inner__extension__seqfield0__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))])


    impl<'input> ParseXml<'input> for list_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence list_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(list_e_inner__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, list_e_inner__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension<'input> {
        seqfield0: list_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for list_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence list_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(list_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, list_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: list_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for list_e_inner<'input> {
        const NODE_NAME: &'static str = "extension list_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(list_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, list_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct list_e<'input> {
        child: list_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "list"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for list_e<'input> {
        const NODE_NAME: &'static str = "element (normal) list_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "list" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(list_e {
                                        child: try_rollback!(stream, tx, list_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxExclusive_e_inner<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for maxExclusive_e_inner<'input> {
        const NODE_NAME: &'static str = "custom maxExclusive_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(maxExclusive_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxExclusive_e<'input> {
        child: maxExclusive_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "maxExclusive"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for maxExclusive_e<'input> {
        const NODE_NAME: &'static str = "element (normal) maxExclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "maxExclusive" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(maxExclusive_e {
                                        child: try_rollback!(stream, tx, maxExclusive_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxInclusive_e_inner<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for maxInclusive_e_inner<'input> {
        const NODE_NAME: &'static str = "custom maxInclusive_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(maxInclusive_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxInclusive_e<'input> {
        child: maxInclusive_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "maxInclusive"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for maxInclusive_e<'input> {
        const NODE_NAME: &'static str = "element (normal) maxInclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "maxInclusive" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(maxInclusive_e {
                                        child: try_rollback!(stream, tx, maxInclusive_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxLength_e_inner<'input>(super::UNQUAL::numFacet<'input>);


    impl<'input> ParseXml<'input> for maxLength_e_inner<'input> {
        const NODE_NAME: &'static str = "custom maxLength_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(maxLength_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxLength_e<'input> {
        child: maxLength_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "maxLength"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for maxLength_e<'input> {
        const NODE_NAME: &'static str = "element (normal) maxLength_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "maxLength" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(maxLength_e {
                                        child: try_rollback!(stream, tx, maxLength_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minExclusive_e_inner<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for minExclusive_e_inner<'input> {
        const NODE_NAME: &'static str = "custom minExclusive_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(minExclusive_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minExclusive_e<'input> {
        child: minExclusive_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "minExclusive"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for minExclusive_e<'input> {
        const NODE_NAME: &'static str = "element (normal) minExclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "minExclusive" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(minExclusive_e {
                                        child: try_rollback!(stream, tx, minExclusive_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minInclusive_e_inner<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for minInclusive_e_inner<'input> {
        const NODE_NAME: &'static str = "custom minInclusive_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(minInclusive_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minInclusive_e<'input> {
        child: minInclusive_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "minInclusive"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for minInclusive_e<'input> {
        const NODE_NAME: &'static str = "element (normal) minInclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "minInclusive" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(minInclusive_e {
                                        child: try_rollback!(stream, tx, minInclusive_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minLength_e_inner<'input>(super::UNQUAL::numFacet<'input>);


    impl<'input> ParseXml<'input> for minLength_e_inner<'input> {
        const NODE_NAME: &'static str = "custom minLength_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(minLength_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minLength_e<'input> {
        child: minLength_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "minLength"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for minLength_e<'input> {
        const NODE_NAME: &'static str = "element (normal) minLength_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "minLength" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(minLength_e {
                                        child: try_rollback!(stream, tx, minLength_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for notation_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence notation_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(notation_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: notation_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for notation_e_inner<'input> {
        const NODE_NAME: &'static str = "extension notation_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(notation_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, notation_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e<'input> {
        child: notation_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "notation"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for notation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) notation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "notation" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(notation_e {
                                        child: try_rollback!(stream, tx, notation_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__seqfield0__seqfield0_item__any_e_inner<'input>(super::UNQUAL::wildcard<'input>);


    impl<'input> ParseXml<'input> for openContent_e_inner__extension__seqfield0__seqfield0_item__any_e_inner<'input> {
        const NODE_NAME: &'static str = "custom openContent_e_inner__extension__seqfield0__seqfield0_item__any_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context).map(openContent_e_inner__extension__seqfield0__seqfield0_item__any_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__seqfield0__seqfield0_item__any_e<'input> {
        child: openContent_e_inner__extension__seqfield0__seqfield0_item__any_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for openContent_e_inner__extension__seqfield0__seqfield0_item__any_e<'input> {
        const NODE_NAME: &'static str = "element (normal) openContent_e_inner__extension__seqfield0__seqfield0_item__any_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "any" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(openContent_e_inner__extension__seqfield0__seqfield0_item__any_e {
                                        child: try_rollback!(stream, tx, openContent_e_inner__extension__seqfield0__seqfield0_item__any_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__seqfield0__seqfield0_item<'input>(openContent_e_inner__extension__seqfield0__seqfield0_item__any_e<'input>);


    impl<'input> ParseXml<'input> for openContent_e_inner__extension__seqfield0__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "elementtype element openContent_e_inner__extension__seqfield0__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            openContent_e_inner__extension__seqfield0__seqfield0_item__any_e::parse_xml(stream, parse_context, parent_context).map(openContent_e_inner__extension__seqfield0__seqfield0_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__seqfield0__seqfield0<'input>(Option<openContent_e_inner__extension__seqfield0__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for openContent_e_inner__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option openContent_e_inner__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(openContent_e_inner__extension__seqfield0__seqfield0(openContent_e_inner__extension__seqfield0__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__seqfield0<'input> {
        seqfield0: openContent_e_inner__extension__seqfield0__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }))])


    impl<'input> ParseXml<'input> for openContent_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence openContent_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(openContent_e_inner__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, openContent_e_inner__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension<'input> {
        seqfield0: openContent_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for openContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence openContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(openContent_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, openContent_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: openContent_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for openContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension openContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(openContent_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, openContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e<'input> {
        child: openContent_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "openContent"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), None, Element(Element { name: QName(None, "any"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for openContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) openContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "openContent" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(openContent_e {
                                        child: try_rollback!(stream, tx, openContent_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__seqfield0__seqfield0<'input>(Option<super::UNQUAL::annotation_e<'input>>);


    impl<'input> ParseXml<'input> for override_e_inner__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "option override_e_inner__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(override_e_inner__extension__seqfield0__seqfield0(super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__seqfield0__seqfield1<'input>(Vec<super::UNQUAL::schemaTop<'input>>);


    impl<'input> ParseXml<'input> for override_e_inner__extension__seqfield0__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec override_e_inner__extension__seqfield0__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::schemaTop::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(override_e_inner__extension__seqfield0__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__seqfield0<'input> {
        seqfield0: override_e_inner__extension__seqfield0__seqfield0<'input>,
        seqfield1: override_e_inner__extension__seqfield0__seqfield1<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation"))), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "schemaTop")))])


    impl<'input> ParseXml<'input> for override_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence override_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(override_e_inner__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, override_e_inner__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, override_e_inner__extension__seqfield0__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension<'input> {
        seqfield0: override_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation"))), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "schemaTop")))]))])


    impl<'input> ParseXml<'input> for override_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence override_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(override_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, override_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: override_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "openAttrs"), Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation"))), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "schemaTop")))]))]))


    impl<'input> ParseXml<'input> for override_e_inner<'input> {
        const NODE_NAME: &'static str = "extension override_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(override_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, override_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct override_e<'input> {
        child: override_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "override"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), Sequence([(None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation"))), (Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "schemaTop")))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for override_e<'input> {
        const NODE_NAME: &'static str = "element (normal) override_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "override" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(override_e {
                                        child: try_rollback!(stream, tx, override_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct pattern_e_inner<'input>(super::UNQUAL::noFixedFacet<'input>);


    impl<'input> ParseXml<'input> for pattern_e_inner<'input> {
        const NODE_NAME: &'static str = "custom pattern_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::noFixedFacet::parse_xml(stream, parse_context, parent_context).map(pattern_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct pattern_e<'input> {
        child: pattern_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "pattern"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for pattern_e<'input> {
        const NODE_NAME: &'static str = "element (normal) pattern_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "pattern" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(pattern_e {
                                        child: try_rollback!(stream, tx, pattern_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum redefine_e_inner__extension__seqfield0_item<'input> {
        choicevariant0(Box<super::UNQUAL::annotation_e<'input>>),
        choicevariant1(Box<super::UNQUAL::redefinable<'input>>),
    }

    impl<'input> Default for redefine_e_inner__extension__seqfield0_item<'input> { fn default() -> redefine_e_inner__extension__seqfield0_item<'input> { redefine_e_inner__extension__seqfield0_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "annotation"))), (None, None, GroupRef(QName(Some("xs"), "redefinable")))])


    impl<'input> ParseXml<'input> for redefine_e_inner__extension__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "choice redefine_e_inner__extension__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefine_e_inner__extension__seqfield0_item::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::redefinable::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefine_e_inner__extension__seqfield0_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner__extension__seqfield0<'input>(Vec<redefine_e_inner__extension__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for redefine_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec redefine_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = redefine_e_inner__extension__seqfield0_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(redefine_e_inner__extension__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner__extension<'input> {
        seqfield0: redefine_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "annotation"))), (None, None, GroupRef(QName(Some("xs"), "redefinable")))]))])


    impl<'input> ParseXml<'input> for redefine_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence redefine_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(redefine_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, redefine_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: redefine_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "openAttrs"), Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "annotation"))), (None, None, GroupRef(QName(Some("xs"), "redefinable")))]))]))


    impl<'input> ParseXml<'input> for redefine_e_inner<'input> {
        const NODE_NAME: &'static str = "extension redefine_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(redefine_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, redefine_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e<'input> {
        child: redefine_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "redefine"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "annotation"))), (None, None, GroupRef(QName(Some("xs"), "redefinable")))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for redefine_e<'input> {
        const NODE_NAME: &'static str = "element (normal) redefine_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "redefine" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(redefine_e {
                                        child: try_rollback!(stream, tx, redefine_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e_inner__extension<'input> {
        seqfield0: super::UNQUAL::simpleRestrictionModel<'input>,
    }

    // ^-- from Sequence([(None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))])


    impl<'input> ParseXml<'input> for restriction_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence restriction_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(restriction_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::simpleRestrictionModel::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: restriction_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))]))


    impl<'input> ParseXml<'input> for restriction_e_inner<'input> {
        const NODE_NAME: &'static str = "extension restriction_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(restriction_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, restriction_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e<'input> {
        child: restriction_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, GroupRef(QName(Some("xs"), "simpleRestrictionModel")))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for restriction_e<'input> {
        const NODE_NAME: &'static str = "element (normal) restriction_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "restriction" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(restriction_e {
                                        child: try_rollback!(stream, tx, restriction_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield0<'input>(Vec<super::UNQUAL::composition<'input>>);


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec schema_e_inner__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::composition::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(schema_e_inner__extension__seqfield0__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield1_item__seqfield1<'input>(Vec<super::UNQUAL::annotation_e<'input>>);


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield1_item__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec schema_e_inner__extension__seqfield0__seqfield1_item__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(schema_e_inner__extension__seqfield0__seqfield1_item__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield1_item<'input> {
        seqfield0: super::UNQUAL::defaultOpenContent_e<'input>,
        seqfield1: schema_e_inner__extension__seqfield0__seqfield1_item__seqfield1<'input>,
    }

    // ^-- from Sequence([(None, None, Ref(QName(Some("xs"), "defaultOpenContent"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))])


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield1_item<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension__seqfield0__seqfield1_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(schema_e_inner__extension__seqfield0__seqfield1_item {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::defaultOpenContent_e::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, schema_e_inner__extension__seqfield0__seqfield1_item__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield1<'input>(Option<schema_e_inner__extension__seqfield0__seqfield1_item<'input>>);


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield1<'input> {
        const NODE_NAME: &'static str = "option schema_e_inner__extension__seqfield0__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(schema_e_inner__extension__seqfield0__seqfield1(schema_e_inner__extension__seqfield0__seqfield1_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield2_item__seqfield1<'input>(Vec<super::UNQUAL::annotation_e<'input>>);


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield2_item__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec schema_e_inner__extension__seqfield0__seqfield2_item__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(schema_e_inner__extension__seqfield0__seqfield2_item__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield2_item<'input> {
        seqfield0: super::UNQUAL::schemaTop<'input>,
        seqfield1: schema_e_inner__extension__seqfield0__seqfield2_item__seqfield1<'input>,
    }

    // ^-- from Sequence([(None, None, GroupRef(QName(Some("xs"), "schemaTop"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))])


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield2_item<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension__seqfield0__seqfield2_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(schema_e_inner__extension__seqfield0__seqfield2_item {



                seqfield0: try_rollback!(stream, tx, super::UNQUAL::schemaTop::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, schema_e_inner__extension__seqfield0__seqfield2_item__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0__seqfield2<'input>(Vec<schema_e_inner__extension__seqfield0__seqfield2_item<'input>>);


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0__seqfield2<'input> {
        const NODE_NAME: &'static str = "vec schema_e_inner__extension__seqfield0__seqfield2";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = schema_e_inner__extension__seqfield0__seqfield2_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(schema_e_inner__extension__seqfield0__seqfield2(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield0<'input> {
        seqfield0: schema_e_inner__extension__seqfield0__seqfield0<'input>,
        seqfield1: schema_e_inner__extension__seqfield0__seqfield1<'input>,
        seqfield2: schema_e_inner__extension__seqfield0__seqfield2<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "composition"))), (Some(0), None, Sequence([(None, None, Ref(QName(Some("xs"), "defaultOpenContent"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))])), (Some(0), Some(18446744073709551615), Sequence([(None, None, GroupRef(QName(Some("xs"), "schemaTop"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))]))])


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(schema_e_inner__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, schema_e_inner__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, schema_e_inner__extension__seqfield0__seqfield1::parse_xml(stream, parse_context, parent_context)),



                seqfield2: try_rollback!(stream, tx, schema_e_inner__extension__seqfield0__seqfield2::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension<'input> {
        seqfield0: schema_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "composition"))), (Some(0), None, Sequence([(None, None, Ref(QName(Some("xs"), "defaultOpenContent"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))])), (Some(0), Some(18446744073709551615), Sequence([(None, None, GroupRef(QName(Some("xs"), "schemaTop"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))]))]))])


    impl<'input> ParseXml<'input> for schema_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(schema_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, schema_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: schema_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "openAttrs"), Sequence([(None, None, Sequence([(Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "composition"))), (Some(0), None, Sequence([(None, None, Ref(QName(Some("xs"), "defaultOpenContent"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))])), (Some(0), Some(18446744073709551615), Sequence([(None, None, GroupRef(QName(Some("xs"), "schemaTop"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))]))]))]))


    impl<'input> ParseXml<'input> for schema_e_inner<'input> {
        const NODE_NAME: &'static str = "extension schema_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(schema_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, schema_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e<'input> {
        child: schema_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "schema"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), Sequence([(None, None, Sequence([(Some(0), Some(18446744073709551615), GroupRef(QName(Some("xs"), "composition"))), (Some(0), None, Sequence([(None, None, Ref(QName(Some("xs"), "defaultOpenContent"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))])), (Some(0), Some(18446744073709551615), Sequence([(None, None, GroupRef(QName(Some("xs"), "schemaTop"))), (Some(0), Some(18446744073709551615), Ref(QName(Some("xs"), "annotation")))]))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for schema_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "schema" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(schema_e {
                                        child: try_rollback!(stream, tx, schema_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e_inner__extension<'input>(PhantomData<&'input ()>);

    // ^-- from Sequence([])


    impl<'input> ParseXml<'input> for selector_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence selector_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(selector_e_inner__extension {



                0: Default::default()



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: selector_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([]))


    impl<'input> ParseXml<'input> for selector_e_inner<'input> {
        const NODE_NAME: &'static str = "extension selector_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(selector_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, selector_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e<'input> {
        child: selector_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "selector"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for selector_e<'input> {
        const NODE_NAME: &'static str = "element (normal) selector_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "selector" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(selector_e {
                                        child: try_rollback!(stream, tx, selector_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct sequence_e_inner<'input>(super::UNQUAL::explicitGroup<'input>);


    impl<'input> ParseXml<'input> for sequence_e_inner<'input> {
        const NODE_NAME: &'static str = "custom sequence_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::explicitGroup::parse_xml(stream, parse_context, parent_context).map(sequence_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct sequence_e<'input> {
        child: sequence_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "sequence"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for sequence_e<'input> {
        const NODE_NAME: &'static str = "element (normal) sequence_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "sequence" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(sequence_e {
                                        child: try_rollback!(stream, tx, sequence_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner<'input>(super::UNQUAL::simpleRestrictionType<'input>);


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner<'input> {
        const NODE_NAME: &'static str = "custom simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::simpleRestrictionType::parse_xml(stream, parse_context, parent_context).map(simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e<'input> {
        child: simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "restriction" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e {
                                        child: try_rollback!(stream, tx, simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension__seqfield0__choicevariant0<'input>(simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e<'input>);


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element simpleContent_e_inner__extension__seqfield0__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            simpleContent_e_inner__extension__seqfield0__choicevariant0__restriction_e::parse_xml(stream, parse_context, parent_context).map(simpleContent_e_inner__extension__seqfield0__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner<'input>(super::UNQUAL::simpleExtensionType<'input>);


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner<'input> {
        const NODE_NAME: &'static str = "custom simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::simpleExtensionType::parse_xml(stream, parse_context, parent_context).map(simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e<'input> {
        child: simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "extension" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e {
                                        child: try_rollback!(stream, tx, simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension__seqfield0__choicevariant1<'input>(simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e<'input>);


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element simpleContent_e_inner__extension__seqfield0__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            simpleContent_e_inner__extension__seqfield0__choicevariant1__extension_e::parse_xml(stream, parse_context, parent_context).map(simpleContent_e_inner__extension__seqfield0__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum simpleContent_e_inner__extension__seqfield0<'input> {
        choicevariant0(Box<simpleContent_e_inner__extension__seqfield0__choicevariant0<'input>>),
        choicevariant1(Box<simpleContent_e_inner__extension__seqfield0__choicevariant1<'input>>),
    }

    impl<'input> Default for simpleContent_e_inner__extension__seqfield0<'input> { fn default() -> simpleContent_e_inner__extension__seqfield0<'input> { simpleContent_e_inner__extension__seqfield0::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "choice simpleContent_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match simpleContent_e_inner__extension__seqfield0__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleContent_e_inner__extension__seqfield0::choicevariant0(Box::new(r))), None => () }



            match simpleContent_e_inner__extension__seqfield0__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleContent_e_inner__extension__seqfield0::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner__extension<'input> {
        seqfield0: simpleContent_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence simpleContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(simpleContent_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, simpleContent_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: simpleContent_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }))]))]))


    impl<'input> ParseXml<'input> for simpleContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension simpleContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(simpleContent_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, simpleContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e<'input> {
        child: simpleContent_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleContent"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Choice([(None, None, Element(Element { name: QName(None, "restriction"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "extension"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for simpleContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleContent" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(simpleContent_e {
                                        child: try_rollback!(stream, tx, simpleContent_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType_e_inner<'input>(super::UNQUAL::topLevelSimpleType<'input>);


    impl<'input> ParseXml<'input> for simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelSimpleType::parse_xml(stream, parse_context, parent_context).map(simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType_e<'input> {
        child: simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelSimpleType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(simpleType_e {
                                        child: try_rollback!(stream, tx, simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct totalDigits_e_inner<'input>(super::UNQUAL::numFacet<'input>);


    impl<'input> ParseXml<'input> for totalDigits_e_inner<'input> {
        const NODE_NAME: &'static str = "custom totalDigits_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(totalDigits_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct totalDigits_e<'input> {
        child: totalDigits_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "totalDigits"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for totalDigits_e<'input> {
        const NODE_NAME: &'static str = "element (normal) totalDigits_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "totalDigits" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(totalDigits_e {
                                        child: try_rollback!(stream, tx, totalDigits_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input>(super::UNQUAL::localSimpleType<'input>);


    impl<'input> ParseXml<'input> for union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e<'input> {
        child: union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e {
                                        child: try_rollback!(stream, tx, union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__seqfield0__seqfield0_item<'input>(union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e<'input>);


    impl<'input> ParseXml<'input> for union_e_inner__extension__seqfield0__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "elementtype element union_e_inner__extension__seqfield0__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            union_e_inner__extension__seqfield0__seqfield0_item__simpleType_e::parse_xml(stream, parse_context, parent_context).map(union_e_inner__extension__seqfield0__seqfield0_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__seqfield0__seqfield0<'input>(Vec<union_e_inner__extension__seqfield0__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for union_e_inner__extension__seqfield0__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec union_e_inner__extension__seqfield0__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = union_e_inner__extension__seqfield0__seqfield0_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(union_e_inner__extension__seqfield0__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__seqfield0<'input> {
        seqfield0: union_e_inner__extension__seqfield0__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }))])


    impl<'input> ParseXml<'input> for union_e_inner__extension__seqfield0<'input> {
        const NODE_NAME: &'static str = "sequence union_e_inner__extension__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(union_e_inner__extension__seqfield0 {



                seqfield0: try_rollback!(stream, tx, union_e_inner__extension__seqfield0__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension<'input> {
        seqfield0: union_e_inner__extension__seqfield0<'input>,
    }

    // ^-- from Sequence([(None, None, Sequence([(Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }))]))])


    impl<'input> ParseXml<'input> for union_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence union_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(union_e_inner__extension {



                seqfield0: try_rollback!(stream, tx, union_e_inner__extension__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: union_e_inner__extension<'input>,
    }

    // ^-- from Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }))]))]))


    impl<'input> ParseXml<'input> for union_e_inner<'input> {
        const NODE_NAME: &'static str = "extension union_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(union_e_inner {
                BASE: try_rollback!(stream, tx, super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)),



                EXTENSION: try_rollback!(stream, tx, union_e_inner__extension::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct union_e<'input> {
        child: union_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "union"), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), Sequence([(None, None, Sequence([(Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }))]))]))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for union_e<'input> {
        const NODE_NAME: &'static str = "element (normal) union_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "union" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(union_e {
                                        child: try_rollback!(stream, tx, union_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct unique_e_inner<'input>(super::UNQUAL::keybase<'input>);


    impl<'input> ParseXml<'input> for unique_e_inner<'input> {
        const NODE_NAME: &'static str = "custom unique_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::keybase::parse_xml(stream, parse_context, parent_context).map(unique_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct unique_e<'input> {
        child: unique_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "unique"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for unique_e<'input> {
        const NODE_NAME: &'static str = "element (normal) unique_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "unique" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(unique_e {
                                        child: try_rollback!(stream, tx, unique_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct whiteSpace_e_inner<'input>(super::UNQUAL::facet<'input>);


    impl<'input> ParseXml<'input> for whiteSpace_e_inner<'input> {
        const NODE_NAME: &'static str = "custom whiteSpace_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(whiteSpace_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct whiteSpace_e<'input> {
        child: whiteSpace_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "whiteSpace"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for whiteSpace_e<'input> {
        const NODE_NAME: &'static str = "element (normal) whiteSpace_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "whiteSpace" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(whiteSpace_e {
                                        child: try_rollback!(stream, tx, whiteSpace_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }


    /////////// groups


    #[derive(Debug, PartialEq, Default)]
    pub struct defRef<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttrGroup<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum composition<'input> {
        choicevariant0(Box<super::UNQUAL::include_e<'input>>),
        choicevariant1(Box<super::UNQUAL::import_e<'input>>),
        choicevariant2(Box<super::UNQUAL::redefine_e<'input>>),
        choicevariant3(Box<super::UNQUAL::override_e<'input>>),
        choicevariant4(Box<super::UNQUAL::annotation_e<'input>>),
    }

    impl<'input> Default for composition<'input> { fn default() -> composition<'input> { composition::choicevariant4(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "include"))), (None, None, Ref(QName(Some("xs"), "import"))), (None, None, Ref(QName(Some("xs"), "redefine"))), (None, None, Ref(QName(Some("xs"), "override"))), (None, None, Ref(QName(Some("xs"), "annotation")))])


    impl<'input> ParseXml<'input> for composition<'input> {
        const NODE_NAME: &'static str = "choice composition";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::include_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::import_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::redefine_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::choicevariant2(Box::new(r))), None => () }



            match super::UNQUAL::override_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::choicevariant3(Box::new(r))), None => () }



            match super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::choicevariant4(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum redefinable<'input> {
        choicevariant0(Box<super::UNQUAL::simpleType_e<'input>>),
        choicevariant1(Box<super::UNQUAL::complexType_e<'input>>),
        choicevariant2(Box<super::UNQUAL::group_e<'input>>),
        choicevariant3(Box<super::UNQUAL::attributeGroup_e<'input>>),
    }

    impl<'input> Default for redefinable<'input> { fn default() -> redefinable<'input> { redefinable::choicevariant3(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "simpleType"))), (None, None, Ref(QName(Some("xs"), "complexType"))), (None, None, Ref(QName(Some("xs"), "group"))), (None, None, Ref(QName(Some("xs"), "attributeGroup")))])


    impl<'input> ParseXml<'input> for redefinable<'input> {
        const NODE_NAME: &'static str = "choice redefinable";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::simpleType_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::complexType_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::group_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::choicevariant2(Box::new(r))), None => () }



            match super::UNQUAL::attributeGroup_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::choicevariant3(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct nestedParticle__choicevariant0__element_e_inner<'input>(super::UNQUAL::localElement<'input>);


    impl<'input> ParseXml<'input> for nestedParticle__choicevariant0__element_e_inner<'input> {
        const NODE_NAME: &'static str = "custom nestedParticle__choicevariant0__element_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localElement::parse_xml(stream, parse_context, parent_context).map(nestedParticle__choicevariant0__element_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct nestedParticle__choicevariant0__element_e<'input> {
        child: nestedParticle__choicevariant0__element_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for nestedParticle__choicevariant0__element_e<'input> {
        const NODE_NAME: &'static str = "element (normal) nestedParticle__choicevariant0__element_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "element" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(nestedParticle__choicevariant0__element_e {
                                        child: try_rollback!(stream, tx, nestedParticle__choicevariant0__element_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct nestedParticle__choicevariant0<'input>(nestedParticle__choicevariant0__element_e<'input>);


    impl<'input> ParseXml<'input> for nestedParticle__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element nestedParticle__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            nestedParticle__choicevariant0__element_e::parse_xml(stream, parse_context, parent_context).map(nestedParticle__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct nestedParticle__choicevariant1__group_e_inner<'input>(super::UNQUAL::groupRef<'input>);


    impl<'input> ParseXml<'input> for nestedParticle__choicevariant1__group_e_inner<'input> {
        const NODE_NAME: &'static str = "custom nestedParticle__choicevariant1__group_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context).map(nestedParticle__choicevariant1__group_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct nestedParticle__choicevariant1__group_e<'input> {
        child: nestedParticle__choicevariant1__group_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for nestedParticle__choicevariant1__group_e<'input> {
        const NODE_NAME: &'static str = "element (normal) nestedParticle__choicevariant1__group_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "group" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(nestedParticle__choicevariant1__group_e {
                                        child: try_rollback!(stream, tx, nestedParticle__choicevariant1__group_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct nestedParticle__choicevariant1<'input>(nestedParticle__choicevariant1__group_e<'input>);


    impl<'input> ParseXml<'input> for nestedParticle__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element nestedParticle__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            nestedParticle__choicevariant1__group_e::parse_xml(stream, parse_context, parent_context).map(nestedParticle__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum nestedParticle<'input> {
        choicevariant0(Box<nestedParticle__choicevariant0<'input>>),
        choicevariant1(Box<nestedParticle__choicevariant1<'input>>),
        choicevariant2(Box<super::UNQUAL::choice_e<'input>>),
        choicevariant3(Box<super::UNQUAL::sequence_e<'input>>),
        choicevariant4(Box<super::UNQUAL::any_e<'input>>),
    }

    impl<'input> Default for nestedParticle<'input> { fn default() -> nestedParticle<'input> { nestedParticle::choicevariant4(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None })), (None, None, Ref(QName(Some("xs"), "choice"))), (None, None, Ref(QName(Some("xs"), "sequence"))), (None, None, Ref(QName(Some("xs"), "any")))])


    impl<'input> ParseXml<'input> for nestedParticle<'input> {
        const NODE_NAME: &'static str = "choice nestedParticle";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match nestedParticle__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::choicevariant0(Box::new(r))), None => () }



            match nestedParticle__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::choice_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::choicevariant2(Box::new(r))), None => () }



            match super::UNQUAL::sequence_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::choicevariant3(Box::new(r))), None => () }



            match super::UNQUAL::any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::choicevariant4(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum simpleDerivation<'input> {
        choicevariant0(Box<super::UNQUAL::restriction_e<'input>>),
        choicevariant1(Box<super::UNQUAL::list_e<'input>>),
        choicevariant2(Box<super::UNQUAL::union_e<'input>>),
    }

    impl<'input> Default for simpleDerivation<'input> { fn default() -> simpleDerivation<'input> { simpleDerivation::choicevariant2(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "restriction"))), (None, None, Ref(QName(Some("xs"), "list"))), (None, None, Ref(QName(Some("xs"), "union")))])


    impl<'input> ParseXml<'input> for simpleDerivation<'input> {
        const NODE_NAME: &'static str = "choice simpleDerivation";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::restriction_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleDerivation::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::list_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleDerivation::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::union_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleDerivation::choicevariant2(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions__seqfield0_item__assert_e_inner<'input>(super::UNQUAL::assertion<'input>);


    impl<'input> ParseXml<'input> for assertions__seqfield0_item__assert_e_inner<'input> {
        const NODE_NAME: &'static str = "custom assertions__seqfield0_item__assert_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::assertion::parse_xml(stream, parse_context, parent_context).map(assertions__seqfield0_item__assert_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions__seqfield0_item__assert_e<'input> {
        child: assertions__seqfield0_item__assert_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "assert"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for assertions__seqfield0_item__assert_e<'input> {
        const NODE_NAME: &'static str = "element (normal) assertions__seqfield0_item__assert_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "assert" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(assertions__seqfield0_item__assert_e {
                                        child: try_rollback!(stream, tx, assertions__seqfield0_item__assert_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions__seqfield0_item<'input>(assertions__seqfield0_item__assert_e<'input>);


    impl<'input> ParseXml<'input> for assertions__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "elementtype element assertions__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            assertions__seqfield0_item__assert_e::parse_xml(stream, parse_context, parent_context).map(assertions__seqfield0_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions__seqfield0<'input>(Vec<assertions__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for assertions__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec assertions__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = assertions__seqfield0_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(assertions__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions<'input> {
        seqfield0: assertions__seqfield0<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), Element(Element { name: QName(None, "assert"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }))])


    impl<'input> ParseXml<'input> for assertions<'input> {
        const NODE_NAME: &'static str = "sequence assertions";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(assertions {



                seqfield0: try_rollback!(stream, tx, assertions__seqfield0::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2__seqfield0<'input>(Option<super::UNQUAL::openContent_e<'input>>);


    impl<'input> ParseXml<'input> for complexTypeModel__choicevariant2__seqfield0<'input> {
        const NODE_NAME: &'static str = "option complexTypeModel__choicevariant2__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(complexTypeModel__choicevariant2__seqfield0(super::UNQUAL::openContent_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2__seqfield1<'input>(Option<super::UNQUAL::typeDefParticle<'input>>);


    impl<'input> ParseXml<'input> for complexTypeModel__choicevariant2__seqfield1<'input> {
        const NODE_NAME: &'static str = "option complexTypeModel__choicevariant2__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(complexTypeModel__choicevariant2__seqfield1(super::UNQUAL::typeDefParticle::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2<'input> {
        seqfield0: complexTypeModel__choicevariant2__seqfield0<'input>,
        seqfield1: complexTypeModel__choicevariant2__seqfield1<'input>,
        seqfield2: super::UNQUAL::attrDecls<'input>,
        seqfield3: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (Some(0), None, GroupRef(QName(Some("xs"), "typeDefParticle"))), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))])


    impl<'input> ParseXml<'input> for complexTypeModel__choicevariant2<'input> {
        const NODE_NAME: &'static str = "sequence complexTypeModel__choicevariant2";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(complexTypeModel__choicevariant2 {



                seqfield0: try_rollback!(stream, tx, complexTypeModel__choicevariant2__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, complexTypeModel__choicevariant2__seqfield1::parse_xml(stream, parse_context, parent_context)),



                seqfield2: try_rollback!(stream, tx, super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)),



                seqfield3: try_rollback!(stream, tx, super::UNQUAL::assertions::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum complexTypeModel<'input> {
        choicevariant0(Box<super::UNQUAL::simpleContent_e<'input>>),
        choicevariant1(Box<super::UNQUAL::complexContent_e<'input>>),
        choicevariant2(Box<complexTypeModel__choicevariant2<'input>>),
    }

    impl<'input> Default for complexTypeModel<'input> { fn default() -> complexTypeModel<'input> { complexTypeModel::choicevariant2(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "simpleContent"))), (None, None, Ref(QName(Some("xs"), "complexContent"))), (None, None, Sequence([(Some(0), None, Ref(QName(Some("xs"), "openContent"))), (Some(0), None, GroupRef(QName(Some("xs"), "typeDefParticle"))), (None, None, GroupRef(QName(Some("xs"), "attrDecls"))), (None, None, GroupRef(QName(Some("xs"), "assertions")))]))])


    impl<'input> ParseXml<'input> for complexTypeModel<'input> {
        const NODE_NAME: &'static str = "choice complexTypeModel";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::simpleContent_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexTypeModel::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::complexContent_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexTypeModel::choicevariant1(Box::new(r))), None => () }



            match complexTypeModel__choicevariant2::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexTypeModel::choicevariant2(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_item__choicevariant0__attribute_e_inner<'input>(super::UNQUAL::attribute<'input>);


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item__choicevariant0__attribute_e_inner<'input> {
        const NODE_NAME: &'static str = "custom attrDecls__seqfield0_item__choicevariant0__attribute_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::attribute::parse_xml(stream, parse_context, parent_context).map(attrDecls__seqfield0_item__choicevariant0__attribute_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_item__choicevariant0__attribute_e<'input> {
        child: attrDecls__seqfield0_item__choicevariant0__attribute_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "attribute"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item__choicevariant0__attribute_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attrDecls__seqfield0_item__choicevariant0__attribute_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "attribute" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(attrDecls__seqfield0_item__choicevariant0__attribute_e {
                                        child: try_rollback!(stream, tx, attrDecls__seqfield0_item__choicevariant0__attribute_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_item__choicevariant0<'input>(attrDecls__seqfield0_item__choicevariant0__attribute_e<'input>);


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element attrDecls__seqfield0_item__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            attrDecls__seqfield0_item__choicevariant0__attribute_e::parse_xml(stream, parse_context, parent_context).map(attrDecls__seqfield0_item__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_item__choicevariant1__attributeGroup_e_inner<'input>(super::UNQUAL::attributeGroupRef<'input>);


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item__choicevariant1__attributeGroup_e_inner<'input> {
        const NODE_NAME: &'static str = "custom attrDecls__seqfield0_item__choicevariant1__attributeGroup_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::attributeGroupRef::parse_xml(stream, parse_context, parent_context).map(attrDecls__seqfield0_item__choicevariant1__attributeGroup_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_item__choicevariant1__attributeGroup_e<'input> {
        child: attrDecls__seqfield0_item__choicevariant1__attributeGroup_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "attributeGroup"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item__choicevariant1__attributeGroup_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attrDecls__seqfield0_item__choicevariant1__attributeGroup_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "attributeGroup" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(attrDecls__seqfield0_item__choicevariant1__attributeGroup_e {
                                        child: try_rollback!(stream, tx, attrDecls__seqfield0_item__choicevariant1__attributeGroup_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_item__choicevariant1<'input>(attrDecls__seqfield0_item__choicevariant1__attributeGroup_e<'input>);


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element attrDecls__seqfield0_item__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            attrDecls__seqfield0_item__choicevariant1__attributeGroup_e::parse_xml(stream, parse_context, parent_context).map(attrDecls__seqfield0_item__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum attrDecls__seqfield0_item<'input> {
        choicevariant0(Box<attrDecls__seqfield0_item__choicevariant0<'input>>),
        choicevariant1(Box<attrDecls__seqfield0_item__choicevariant1<'input>>),
    }

    impl<'input> Default for attrDecls__seqfield0_item<'input> { fn default() -> attrDecls__seqfield0_item<'input> { attrDecls__seqfield0_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "attribute"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "attributeGroup"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "choice attrDecls__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match attrDecls__seqfield0_item__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(attrDecls__seqfield0_item::choicevariant0(Box::new(r))), None => () }



            match attrDecls__seqfield0_item__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(attrDecls__seqfield0_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0<'input>(Vec<attrDecls__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for attrDecls__seqfield0<'input> {
        const NODE_NAME: &'static str = "vec attrDecls__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = attrDecls__seqfield0_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(attrDecls__seqfield0(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield1<'input>(Option<super::UNQUAL::anyAttribute_e<'input>>);


    impl<'input> ParseXml<'input> for attrDecls__seqfield1<'input> {
        const NODE_NAME: &'static str = "option attrDecls__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(attrDecls__seqfield1(super::UNQUAL::anyAttribute_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls<'input> {
        seqfield0: attrDecls__seqfield0<'input>,
        seqfield1: attrDecls__seqfield1<'input>,
    }

    // ^-- from Sequence([(Some(0), Some(18446744073709551615), Choice([(None, None, Element(Element { name: QName(None, "attribute"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "attributeGroup"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }))])), (Some(0), None, Ref(QName(Some("xs"), "anyAttribute")))])


    impl<'input> ParseXml<'input> for attrDecls<'input> {
        const NODE_NAME: &'static str = "sequence attrDecls";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(attrDecls {



                seqfield0: try_rollback!(stream, tx, attrDecls__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, attrDecls__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct typeDefParticle__choicevariant0__group_e_inner<'input>(super::UNQUAL::groupRef<'input>);


    impl<'input> ParseXml<'input> for typeDefParticle__choicevariant0__group_e_inner<'input> {
        const NODE_NAME: &'static str = "custom typeDefParticle__choicevariant0__group_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context).map(typeDefParticle__choicevariant0__group_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct typeDefParticle__choicevariant0__group_e<'input> {
        child: typeDefParticle__choicevariant0__group_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for typeDefParticle__choicevariant0__group_e<'input> {
        const NODE_NAME: &'static str = "element (normal) typeDefParticle__choicevariant0__group_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "group" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(typeDefParticle__choicevariant0__group_e {
                                        child: try_rollback!(stream, tx, typeDefParticle__choicevariant0__group_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct typeDefParticle__choicevariant0<'input>(typeDefParticle__choicevariant0__group_e<'input>);


    impl<'input> ParseXml<'input> for typeDefParticle__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element typeDefParticle__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            typeDefParticle__choicevariant0__group_e::parse_xml(stream, parse_context, parent_context).map(typeDefParticle__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum typeDefParticle<'input> {
        choicevariant0(Box<typeDefParticle__choicevariant0<'input>>),
        choicevariant1(Box<super::UNQUAL::all_e<'input>>),
        choicevariant2(Box<super::UNQUAL::choice_e<'input>>),
        choicevariant3(Box<super::UNQUAL::sequence_e<'input>>),
    }

    impl<'input> Default for typeDefParticle<'input> { fn default() -> typeDefParticle<'input> { typeDefParticle::choicevariant3(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None })), (None, None, Ref(QName(Some("xs"), "all"))), (None, None, Ref(QName(Some("xs"), "choice"))), (None, None, Ref(QName(Some("xs"), "sequence")))])


    impl<'input> ParseXml<'input> for typeDefParticle<'input> {
        const NODE_NAME: &'static str = "choice typeDefParticle";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match typeDefParticle__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::all_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::choice_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::choicevariant2(Box::new(r))), None => () }



            match super::UNQUAL::sequence_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::choicevariant3(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum schemaTop<'input> {
        choicevariant0(Box<super::UNQUAL::redefinable<'input>>),
        choicevariant1(Box<super::UNQUAL::element_e<'input>>),
        choicevariant2(Box<super::UNQUAL::attribute_e<'input>>),
        choicevariant3(Box<super::UNQUAL::notation_e<'input>>),
    }

    impl<'input> Default for schemaTop<'input> { fn default() -> schemaTop<'input> { schemaTop::choicevariant3(Default::default()) } }

    // ^-- from Choice([(None, None, GroupRef(QName(Some("xs"), "redefinable"))), (None, None, Ref(QName(Some("xs"), "element"))), (None, None, Ref(QName(Some("xs"), "attribute"))), (None, None, Ref(QName(Some("xs"), "notation")))])


    impl<'input> ParseXml<'input> for schemaTop<'input> {
        const NODE_NAME: &'static str = "choice schemaTop";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::redefinable::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::element_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::attribute_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::choicevariant2(Box::new(r))), None => () }



            match super::UNQUAL::notation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::choicevariant3(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield0<'input>(Option<super::UNQUAL::annotation_e<'input>>);


    impl<'input> ParseXml<'input> for allModel__seqfield0<'input> {
        const NODE_NAME: &'static str = "option allModel__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(allModel__seqfield0(super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_item__choicevariant0__element_e_inner<'input>(super::UNQUAL::localElement<'input>);


    impl<'input> ParseXml<'input> for allModel__seqfield1_item__choicevariant0__element_e_inner<'input> {
        const NODE_NAME: &'static str = "custom allModel__seqfield1_item__choicevariant0__element_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localElement::parse_xml(stream, parse_context, parent_context).map(allModel__seqfield1_item__choicevariant0__element_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_item__choicevariant0__element_e<'input> {
        child: allModel__seqfield1_item__choicevariant0__element_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for allModel__seqfield1_item__choicevariant0__element_e<'input> {
        const NODE_NAME: &'static str = "element (normal) allModel__seqfield1_item__choicevariant0__element_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "element" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(allModel__seqfield1_item__choicevariant0__element_e {
                                        child: try_rollback!(stream, tx, allModel__seqfield1_item__choicevariant0__element_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_item__choicevariant0<'input>(allModel__seqfield1_item__choicevariant0__element_e<'input>);


    impl<'input> ParseXml<'input> for allModel__seqfield1_item__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element allModel__seqfield1_item__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            allModel__seqfield1_item__choicevariant0__element_e::parse_xml(stream, parse_context, parent_context).map(allModel__seqfield1_item__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_item__choicevariant2__group_e_inner<'input>(super::UNQUAL::groupRef<'input>);


    impl<'input> ParseXml<'input> for allModel__seqfield1_item__choicevariant2__group_e_inner<'input> {
        const NODE_NAME: &'static str = "custom allModel__seqfield1_item__choicevariant2__group_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context).map(allModel__seqfield1_item__choicevariant2__group_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_item__choicevariant2__group_e<'input> {
        child: allModel__seqfield1_item__choicevariant2__group_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for allModel__seqfield1_item__choicevariant2__group_e<'input> {
        const NODE_NAME: &'static str = "element (normal) allModel__seqfield1_item__choicevariant2__group_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "group" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(allModel__seqfield1_item__choicevariant2__group_e {
                                        child: try_rollback!(stream, tx, allModel__seqfield1_item__choicevariant2__group_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_item__choicevariant2<'input>(allModel__seqfield1_item__choicevariant2__group_e<'input>);


    impl<'input> ParseXml<'input> for allModel__seqfield1_item__choicevariant2<'input> {
        const NODE_NAME: &'static str = "elementtype element allModel__seqfield1_item__choicevariant2";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            allModel__seqfield1_item__choicevariant2__group_e::parse_xml(stream, parse_context, parent_context).map(allModel__seqfield1_item__choicevariant2)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum allModel__seqfield1_item<'input> {
        choicevariant0(Box<allModel__seqfield1_item__choicevariant0<'input>>),
        choicevariant1(Box<super::UNQUAL::any_e<'input>>),
        choicevariant2(Box<allModel__seqfield1_item__choicevariant2<'input>>),
    }

    impl<'input> Default for allModel__seqfield1_item<'input> { fn default() -> allModel__seqfield1_item<'input> { allModel__seqfield1_item::choicevariant2(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None })), (None, None, Ref(QName(Some("xs"), "any"))), (None, None, Element(Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }))])


    impl<'input> ParseXml<'input> for allModel__seqfield1_item<'input> {
        const NODE_NAME: &'static str = "choice allModel__seqfield1_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match allModel__seqfield1_item__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(allModel__seqfield1_item::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(allModel__seqfield1_item::choicevariant1(Box::new(r))), None => () }



            match allModel__seqfield1_item__choicevariant2::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(allModel__seqfield1_item::choicevariant2(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1<'input>(Vec<allModel__seqfield1_item<'input>>);


    impl<'input> ParseXml<'input> for allModel__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec allModel__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = allModel__seqfield1_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(allModel__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel<'input> {
        seqfield0: allModel__seqfield0<'input>,
        seqfield1: allModel__seqfield1<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Ref(QName(Some("xs"), "annotation"))), (Some(0), Some(18446744073709551615), Choice([(None, None, Element(Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None })), (None, None, Ref(QName(Some("xs"), "any"))), (None, None, Element(Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }))]))])


    impl<'input> ParseXml<'input> for allModel<'input> {
        const NODE_NAME: &'static str = "sequence allModel";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(allModel {



                seqfield0: try_rollback!(stream, tx, allModel__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, allModel__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct occurs<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield0_item__simpleType_e_inner<'input>(super::UNQUAL::localSimpleType<'input>);


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield0_item__simpleType_e_inner<'input> {
        const NODE_NAME: &'static str = "custom simpleRestrictionModel__seqfield0_item__simpleType_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(simpleRestrictionModel__seqfield0_item__simpleType_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield0_item__simpleType_e<'input> {
        child: simpleRestrictionModel__seqfield0_item__simpleType_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield0_item__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleRestrictionModel__seqfield0_item__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "simpleType" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(simpleRestrictionModel__seqfield0_item__simpleType_e {
                                        child: try_rollback!(stream, tx, simpleRestrictionModel__seqfield0_item__simpleType_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield0_item<'input>(simpleRestrictionModel__seqfield0_item__simpleType_e<'input>);


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield0_item<'input> {
        const NODE_NAME: &'static str = "elementtype element simpleRestrictionModel__seqfield0_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            simpleRestrictionModel__seqfield0_item__simpleType_e::parse_xml(stream, parse_context, parent_context).map(simpleRestrictionModel__seqfield0_item)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield0<'input>(Option<simpleRestrictionModel__seqfield0_item<'input>>);


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield0<'input> {
        const NODE_NAME: &'static str = "option simpleRestrictionModel__seqfield0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(simpleRestrictionModel__seqfield0(simpleRestrictionModel__seqfield0_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum simpleRestrictionModel__seqfield1_item<'input> {
        choicevariant0(Box<super::UNQUAL::facet_e<'input>>),
        choicevariant1(Box<any_e<'input>>),
    }

    impl<'input> Default for simpleRestrictionModel__seqfield1_item<'input> { fn default() -> simpleRestrictionModel__seqfield1_item<'input> { simpleRestrictionModel__seqfield1_item::choicevariant1(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "facet"))), (None, None, Ref(QName(None, "any")))])


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield1_item<'input> {
        const NODE_NAME: &'static str = "choice simpleRestrictionModel__seqfield1_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::facet_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleRestrictionModel__seqfield1_item::choicevariant0(Box::new(r))), None => () }



            match any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleRestrictionModel__seqfield1_item::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield1<'input>(Vec<simpleRestrictionModel__seqfield1_item<'input>>);


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield1<'input> {
        const NODE_NAME: &'static str = "vec simpleRestrictionModel__seqfield1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = simpleRestrictionModel__seqfield1_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(simpleRestrictionModel__seqfield1(items))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel<'input> {
        seqfield0: simpleRestrictionModel__seqfield0<'input>,
        seqfield1: simpleRestrictionModel__seqfield1<'input>,
    }

    // ^-- from Sequence([(Some(0), None, Element(Element { name: QName(None, "simpleType"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None })), (Some(0), Some(18446744073709551615), Choice([(None, None, Ref(QName(Some("xs"), "facet"))), (None, None, Ref(QName(None, "any")))]))])


    impl<'input> ParseXml<'input> for simpleRestrictionModel<'input> {
        const NODE_NAME: &'static str = "sequence simpleRestrictionModel";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            Some(simpleRestrictionModel {



                seqfield0: try_rollback!(stream, tx, simpleRestrictionModel__seqfield0::parse_xml(stream, parse_context, parent_context)),



                seqfield1: try_rollback!(stream, tx, simpleRestrictionModel__seqfield1::parse_xml(stream, parse_context, parent_context)),



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct particle__choicevariant0__element_e_inner<'input>(super::UNQUAL::localElement<'input>);


    impl<'input> ParseXml<'input> for particle__choicevariant0__element_e_inner<'input> {
        const NODE_NAME: &'static str = "custom particle__choicevariant0__element_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localElement::parse_xml(stream, parse_context, parent_context).map(particle__choicevariant0__element_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct particle__choicevariant0__element_e<'input> {
        child: particle__choicevariant0__element_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for particle__choicevariant0__element_e<'input> {
        const NODE_NAME: &'static str = "element (normal) particle__choicevariant0__element_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "element" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(particle__choicevariant0__element_e {
                                        child: try_rollback!(stream, tx, particle__choicevariant0__element_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct particle__choicevariant0<'input>(particle__choicevariant0__element_e<'input>);


    impl<'input> ParseXml<'input> for particle__choicevariant0<'input> {
        const NODE_NAME: &'static str = "elementtype element particle__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            particle__choicevariant0__element_e::parse_xml(stream, parse_context, parent_context).map(particle__choicevariant0)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct particle__choicevariant1__group_e_inner<'input>(super::UNQUAL::groupRef<'input>);


    impl<'input> ParseXml<'input> for particle__choicevariant1__group_e_inner<'input> {
        const NODE_NAME: &'static str = "custom particle__choicevariant1__group_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context).map(particle__choicevariant1__group_e_inner)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct particle__choicevariant1__group_e<'input> {
        child: particle__choicevariant1__group_e_inner<'input>,
    }

    // ^-- from Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for particle__choicevariant1__group_e<'input> {
        const NODE_NAME: &'static str = "element (normal) particle__choicevariant1__group_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let tx = stream.transaction();
            let mut tok = stream.next().unwrap();
            loop {
                match tok {
                    Token::Whitespaces(_) => (),
                    Token::Comment(_) => (),
                    _ => break,
                }
                tok = stream.next().unwrap();
            }
            match tok {
                Token::ElementStart(prefix, name) => {
                    if name.to_str() == "group" {



                        loop {
                            let tok = stream.next().unwrap();
                            match tok {
                                Token::Whitespaces(_) => (),
                                Token::Comment(_) => (),
                                Token::Attribute(_, _) => (),
                                Token::ElementEnd(ElementEnd::Open) =>
                                    return Some(particle__choicevariant1__group_e {
                                        child: try_rollback!(stream, tx, particle__choicevariant1__group_e_inner::parse_xml(stream, parse_context, parent_context)),



                                    }),
                                Token::ElementEnd(_) =>
                                    return Default::default(), // TODO
                                _ => panic!(format!("Did not expect token {:?}", tok)),
                            }
                        }
                    }
                    else {
                        tx.rollback(stream);
                        None
                    }
                },
                _ => panic!(format!("Did not expect token {:?}", tok)),
            }
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct particle__choicevariant1<'input>(particle__choicevariant1__group_e<'input>);


    impl<'input> ParseXml<'input> for particle__choicevariant1<'input> {
        const NODE_NAME: &'static str = "elementtype element particle__choicevariant1";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            particle__choicevariant1__group_e::parse_xml(stream, parse_context, parent_context).map(particle__choicevariant1)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum particle<'input> {
        choicevariant0(Box<particle__choicevariant0<'input>>),
        choicevariant1(Box<particle__choicevariant1<'input>>),
        choicevariant2(Box<super::UNQUAL::all_e<'input>>),
        choicevariant3(Box<super::UNQUAL::choice_e<'input>>),
        choicevariant4(Box<super::UNQUAL::sequence_e<'input>>),
        choicevariant5(Box<super::UNQUAL::any_e<'input>>),
    }

    impl<'input> Default for particle<'input> { fn default() -> particle<'input> { particle::choicevariant5(Default::default()) } }

    // ^-- from Choice([(None, None, Element(Element { name: QName(None, "element"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None })), (None, None, Element(Element { name: QName(None, "group"), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None })), (None, None, Ref(QName(Some("xs"), "all"))), (None, None, Ref(QName(Some("xs"), "choice"))), (None, None, Ref(QName(Some("xs"), "sequence"))), (None, None, Ref(QName(Some("xs"), "any")))])


    impl<'input> ParseXml<'input> for particle<'input> {
        const NODE_NAME: &'static str = "choice particle";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match particle__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choicevariant0(Box::new(r))), None => () }



            match particle__choicevariant1::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::all_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choicevariant2(Box::new(r))), None => () }



            match super::UNQUAL::choice_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choicevariant3(Box::new(r))), None => () }



            match super::UNQUAL::sequence_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choicevariant4(Box::new(r))), None => () }



            match super::UNQUAL::any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choicevariant5(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum identityConstraint<'input> {
        choicevariant0(Box<super::UNQUAL::unique_e<'input>>),
        choicevariant1(Box<super::UNQUAL::key_e<'input>>),
        choicevariant2(Box<super::UNQUAL::keyref_e<'input>>),
    }

    impl<'input> Default for identityConstraint<'input> { fn default() -> identityConstraint<'input> { identityConstraint::choicevariant2(Default::default()) } }

    // ^-- from Choice([(None, None, Ref(QName(Some("xs"), "unique"))), (None, None, Ref(QName(Some("xs"), "key"))), (None, None, Ref(QName(Some("xs"), "keyref")))])


    impl<'input> ParseXml<'input> for identityConstraint<'input> {
        const NODE_NAME: &'static str = "choice identityConstraint";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::unique_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(identityConstraint::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::key_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(identityConstraint::choicevariant1(Box::new(r))), None => () }



            match super::UNQUAL::keyref_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(identityConstraint::choicevariant2(Box::new(r))), None => () }



            None
        }
    }
}
