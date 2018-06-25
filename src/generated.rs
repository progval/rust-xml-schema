#[allow(bad_style)]
pub mod UNQUAL {
    use std::marker::PhantomData;

    use support::*;


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

    // ^-- from Union(Some([QName(Some("xs"), "nonNegativeInteger")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq)]
    pub enum altType_item__extension<'input> {
        simpleType(Box<super::UNQUAL::localSimpleType<'input>>),
        complexType(Box<super::UNQUAL::localComplexType<'input>>),
    }

    impl<'input> Default for altType_item__extension<'input> { fn default() -> altType_item__extension<'input> { altType_item__extension::complexType(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for altType_item__extension<'input> {
        const NODE_NAME: &'static str = "choice altType_item__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(altType_item__extension::simpleType(Box::new(r))), None => () }



            match super::UNQUAL::localComplexType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(altType_item__extension::complexType(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType_item<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: altType_item__extension<'input>,
    }


    impl<'input> ParseXml<'input> for altType_item<'input> {
        const NODE_NAME: &'static str = "extension altType_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(altType_item {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: altType_item__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct altType<'input>(Option<altType_item<'input>>);


    impl<'input> ParseXml<'input> for altType<'input> {
        const NODE_NAME: &'static str = "option altType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(altType(altType_item::parse_xml(stream, parse_context, parent_context)))
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for annotated__extension__annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) annotated__extension__annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context).map(annotated__extension__annotation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension<'input> {
        seqfield0: annotated__extension__annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }])


    impl<'input> ParseXml<'input> for annotated__extension<'input> {
        const NODE_NAME: &'static str = "sequence annotated__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(annotated__extension {



                seqfield0: annotated__extension__annotation_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotated__extension<'input>,
    }


    impl<'input> ParseXml<'input> for annotated<'input> {
        const NODE_NAME: &'static str = "extension annotated";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(annotated {
                BASE: super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: annotated__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct anyType__seqfield0_e<'input>(any<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for anyType__seqfield0_e<'input> {
        const NODE_NAME: &'static str = "element (normal) anyType__seqfield0_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            any::parse_xml(stream, parse_context, parent_context).map(anyType__seqfield0_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyType<'input> {
        seqfield0: anyType__seqfield0_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for anyType<'input> {
        const NODE_NAME: &'static str = "sequence anyType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(anyType {



                seqfield0: anyType__seqfield0_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for assertion<'input> {
        const NODE_NAME: &'static str = "extension assertion";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(assertion {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for attribute__extension__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (custom) attribute__extension__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(attribute__extension__simpleType_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension<'input> {
        simpleType: attribute__extension__simpleType_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }])


    impl<'input> ParseXml<'input> for attribute__extension<'input> {
        const NODE_NAME: &'static str = "sequence attribute__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(attribute__extension {



                simpleType: attribute__extension__simpleType_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: attribute__extension<'input>,
    }


    impl<'input> ParseXml<'input> for attribute<'input> {
        const NODE_NAME: &'static str = "extension attribute";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(attribute {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: attribute__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::attrDecls<'input>,
    }


    impl<'input> ParseXml<'input> for attributeGroup<'input> {
        const NODE_NAME: &'static str = "extension attributeGroup";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(attributeGroup {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)?,



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

    // ^-- from Union(Some([QName(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList<'input>(Vec<basicNamespaceList__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Union(Some([QName(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))))

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

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(List(ComplexList(false, Custom(QName(Some("xs"), "derivationControl"))))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct complexRestrictionType<'input>(super::UNQUAL::restrictionType<'input>);


    impl<'input> ParseXml<'input> for complexRestrictionType<'input> {
        const NODE_NAME: &'static str = "custom complexRestrictionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::restrictionType::parse_xml(stream, parse_context, parent_context).map(complexRestrictionType)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::complexTypeModel<'input>,
    }


    impl<'input> ParseXml<'input> for complexType<'input> {
        const NODE_NAME: &'static str = "extension complexType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(complexType {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: super::UNQUAL::complexTypeModel::parse_xml(stream, parse_context, parent_context)?,



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

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(List(SimpleList(QName(Some("xs"), "reducedDerivationControl")))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq)]
    pub enum element__extension__seqfield0_e_inner<'input> {
        simpleType(Box<super::UNQUAL::localSimpleType<'input>>),
        complexType(Box<super::UNQUAL::localComplexType<'input>>),
    }

    impl<'input> Default for element__extension__seqfield0_e_inner<'input> { fn default() -> element__extension__seqfield0_e_inner<'input> { element__extension__seqfield0_e_inner::complexType(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for element__extension__seqfield0_e_inner<'input> {
        const NODE_NAME: &'static str = "choice element__extension__seqfield0_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(element__extension__seqfield0_e_inner::simpleType(Box::new(r))), None => () }



            match super::UNQUAL::localComplexType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(element__extension__seqfield0_e_inner::complexType(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0_e<'input>(element__extension__seqfield0_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for element__extension__seqfield0_e<'input> {
        const NODE_NAME: &'static str = "element (normal) element__extension__seqfield0_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            element__extension__seqfield0_e_inner::parse_xml(stream, parse_context, parent_context).map(element__extension__seqfield0_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__alternative_e<'input>(super::UNQUAL::altType<'input>);

    // ^-- from Element { name: Some(QName(None, "alternative")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for element__extension__alternative_e<'input> {
        const NODE_NAME: &'static str = "element (custom) element__extension__alternative_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::altType::parse_xml(stream, parse_context, parent_context).map(element__extension__alternative_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__identityConstraint_e<'input>(super::UNQUAL::identityConstraint<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "identityConstraint"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for element__extension__identityConstraint_e<'input> {
        const NODE_NAME: &'static str = "element (normal) element__extension__identityConstraint_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::identityConstraint::parse_xml(stream, parse_context, parent_context).map(element__extension__identityConstraint_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension<'input> {
        seqfield0: element__extension__seqfield0_e<'input>,
        alternative: element__extension__alternative_e<'input>,
        seqfield2: element__extension__identityConstraint_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }, Element { name: Some(QName(None, "alternative")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "identityConstraint"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for element__extension<'input> {
        const NODE_NAME: &'static str = "sequence element__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(element__extension {



                seqfield0: element__extension__seqfield0_e::parse_xml(stream, parse_context, parent_context)?,



                alternative: element__extension__alternative_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield2: element__extension__identityConstraint_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: element__extension<'input>,
    }


    impl<'input> ParseXml<'input> for element<'input> {
        const NODE_NAME: &'static str = "extension element";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(element {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: element__extension::parse_xml(stream, parse_context, parent_context)?,



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
    pub struct extensionType__extension__openContent_e<'input>(super::UNQUAL::openContent_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for extensionType__extension__openContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) extensionType__extension__openContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::openContent_e::parse_xml(stream, parse_context, parent_context).map(extensionType__extension__openContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension__typeDefParticle_e<'input>(super::UNQUAL::typeDefParticle<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for extensionType__extension__typeDefParticle_e<'input> {
        const NODE_NAME: &'static str = "element (normal) extensionType__extension__typeDefParticle_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::typeDefParticle::parse_xml(stream, parse_context, parent_context).map(extensionType__extension__typeDefParticle_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension<'input> {
        seqfield0: extensionType__extension__openContent_e<'input>,
        seqfield1: extensionType__extension__typeDefParticle_e<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for extensionType__extension<'input> {
        const NODE_NAME: &'static str = "sequence extensionType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(extensionType__extension {



                seqfield0: extensionType__extension__openContent_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: extensionType__extension__typeDefParticle_e::parse_xml(stream, parse_context, parent_context)?,



                attrDecls: super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)?,



                assertions: super::UNQUAL::assertions::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: extensionType__extension<'input>,
    }


    impl<'input> ParseXml<'input> for extensionType<'input> {
        const NODE_NAME: &'static str = "extension extensionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(extensionType {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: extensionType__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct facet<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for facet<'input> {
        const NODE_NAME: &'static str = "extension facet";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(facet {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



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

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(List(SimpleList(QName(Some("xs"), "typeDerivationControl")))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct group_item<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::particle<'input>,
    }


    impl<'input> ParseXml<'input> for group_item<'input> {
        const NODE_NAME: &'static str = "extension group_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(group_item {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: super::UNQUAL::particle::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct group<'input>(Vec<group_item<'input>>);


    impl<'input> ParseXml<'input> for group<'input> {
        const NODE_NAME: &'static str = "vec group";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            let mut items = Vec::new();
            while let Some(new_item) = group_item::parse_xml(stream, parse_context, parent_context) {
                items.push(new_item);
            }
            Some(group(items))
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
    pub struct keybase_item__extension__field_e<'input>(super::UNQUAL::field_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "field"))), min_occurs: Some(1), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for keybase_item__extension__field_e<'input> {
        const NODE_NAME: &'static str = "element (normal) keybase_item__extension__field_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::field_e::parse_xml(stream, parse_context, parent_context).map(keybase_item__extension__field_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase_item__extension<'input> {
        selector: super::UNQUAL::selector_e<'input>,
        seqfield1: keybase_item__extension__field_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "selector"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "field"))), min_occurs: Some(1), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for keybase_item__extension<'input> {
        const NODE_NAME: &'static str = "sequence keybase_item__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(keybase_item__extension {



                selector: super::UNQUAL::selector_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: keybase_item__extension__field_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase_item<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: keybase_item__extension<'input>,
    }


    impl<'input> ParseXml<'input> for keybase_item<'input> {
        const NODE_NAME: &'static str = "extension keybase_item";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(keybase_item {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: keybase_item__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct keybase<'input>(Option<keybase_item<'input>>);


    impl<'input> ParseXml<'input> for keybase<'input> {
        const NODE_NAME: &'static str = "option keybase";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(keybase(keybase_item::parse_xml(stream, parse_context, parent_context)))
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

    // ^-- from Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList<'input>(Vec<qnameList__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))))

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

    // ^-- from Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA<'input>(Vec<qnameListA__valuetype<'input>>);

    // ^-- from List(ComplexList(false, Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))))

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
    pub struct restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e<'input>(super::UNQUAL::openContent_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::openContent_e::parse_xml(stream, parse_context, parent_context).map(restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0_e_inner__choicevariant0<'input> {
        seqfield0: restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e<'input>,
        typeDefParticle: super::UNQUAL::typeDefParticle<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0_e_inner__choicevariant0<'input> {
        const NODE_NAME: &'static str = "sequence restrictionType__extension__seqfield0_e_inner__choicevariant0";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(restrictionType__extension__seqfield0_e_inner__choicevariant0 {



                seqfield0: restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e::parse_xml(stream, parse_context, parent_context)?,



                typeDefParticle: super::UNQUAL::typeDefParticle::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum restrictionType__extension__seqfield0_e_inner<'input> {
        choicevariant0(Box<restrictionType__extension__seqfield0_e_inner__choicevariant0<'input>>),
        simpleRestrictionModel(Box<super::UNQUAL::simpleRestrictionModel<'input>>),
    }

    impl<'input> Default for restrictionType__extension__seqfield0_e_inner<'input> { fn default() -> restrictionType__extension__seqfield0_e_inner<'input> { restrictionType__extension__seqfield0_e_inner::simpleRestrictionModel(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0_e_inner<'input> {
        const NODE_NAME: &'static str = "choice restrictionType__extension__seqfield0_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match restrictionType__extension__seqfield0_e_inner__choicevariant0::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(restrictionType__extension__seqfield0_e_inner::choicevariant0(Box::new(r))), None => () }



            match super::UNQUAL::simpleRestrictionModel::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(restrictionType__extension__seqfield0_e_inner::simpleRestrictionModel(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0_e<'input>(restrictionType__extension__seqfield0_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for restrictionType__extension__seqfield0_e<'input> {
        const NODE_NAME: &'static str = "element (normal) restrictionType__extension__seqfield0_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            restrictionType__extension__seqfield0_e_inner::parse_xml(stream, parse_context, parent_context).map(restrictionType__extension__seqfield0_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension<'input> {
        seqfield0: restrictionType__extension__seqfield0_e<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for restrictionType__extension<'input> {
        const NODE_NAME: &'static str = "sequence restrictionType__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(restrictionType__extension {



                seqfield0: restrictionType__extension__seqfield0_e::parse_xml(stream, parse_context, parent_context)?,



                attrDecls: super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)?,



                assertions: super::UNQUAL::assertions::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: restrictionType__extension<'input>,
    }


    impl<'input> ParseXml<'input> for restrictionType<'input> {
        const NODE_NAME: &'static str = "extension restrictionType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(restrictionType {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: restrictionType__extension::parse_xml(stream, parse_context, parent_context)?,



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

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(List(ComplexList(false, Custom(QName(Some("xs"), "derivationControl"))))), min_occurs: None, max_occurs: None }]))

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
    pub struct simpleType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::simpleDerivation<'input>,
    }


    impl<'input> ParseXml<'input> for simpleType<'input> {
        const NODE_NAME: &'static str = "extension simpleType";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(simpleType {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: super::UNQUAL::simpleDerivation::parse_xml(stream, parse_context, parent_context)?,



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
    pub struct wildcard<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for wildcard<'input> {
        const NODE_NAME: &'static str = "extension wildcard";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(wildcard {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



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

    // ^-- from Union(Some([QName(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None }]))


    /////////// elements


    #[derive(Debug, PartialEq, Default)]
    pub struct all_e<'input>(super::UNQUAL::all<'input>);

    // ^-- from Element { name: Some(QName(None, "all")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "all"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for all_e<'input> {
        const NODE_NAME: &'static str = "element (custom) all_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::all::parse_xml(stream, parse_context, parent_context).map(all_e)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum annotation_e_inner__extension<'input> {
        appinfo(Box<super::UNQUAL::appinfo_e<'input>>),
        documentation(Box<super::UNQUAL::documentation_e<'input>>),
    }

    impl<'input> Default for annotation_e_inner__extension<'input> { fn default() -> annotation_e_inner__extension<'input> { annotation_e_inner__extension::documentation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "appinfo"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "documentation"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for annotation_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "choice annotation_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::appinfo_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(annotation_e_inner__extension::appinfo(Box::new(r))), None => () }



            match super::UNQUAL::documentation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(annotation_e_inner__extension::documentation(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotation_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for annotation_e_inner<'input> {
        const NODE_NAME: &'static str = "extension annotation_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(annotation_e_inner {
                BASE: super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: annotation_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e<'input>(annotation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "annotation")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "appinfo"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "documentation"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            annotation_e_inner::parse_xml(stream, parse_context, parent_context).map(annotation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
    }


    impl<'input> ParseXml<'input> for any_e_inner<'input> {
        const NODE_NAME: &'static str = "extension any_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(any_e_inner {
                BASE: super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct any_e<'input>(any_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(QName(Some("xs"), "qnameList")), default: None }, GroupRef(QName(Some("xs"), "occurs"))], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for any_e<'input> {
        const NODE_NAME: &'static str = "element (normal) any_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            any_e_inner::parse_xml(stream, parse_context, parent_context).map(any_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
    }


    impl<'input> ParseXml<'input> for anyAttribute_e_inner<'input> {
        const NODE_NAME: &'static str = "extension anyAttribute_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(anyAttribute_e_inner {
                BASE: super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e<'input>(anyAttribute_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "anyAttribute")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(QName(Some("xs"), "qnameListA")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for anyAttribute_e<'input> {
        const NODE_NAME: &'static str = "element (normal) anyAttribute_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            anyAttribute_e_inner::parse_xml(stream, parse_context, parent_context).map(anyAttribute_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e_inner<'input> {
        seqfield0: any<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for appinfo_e_inner<'input> {
        const NODE_NAME: &'static str = "sequence appinfo_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(appinfo_e_inner {



                seqfield0: any::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e<'input>(appinfo_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "appinfo")), attrs: [SmallDef { name: "source", type_: Some(QName(Some("xs"), "anyURI")), default: None }, Any], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for appinfo_e<'input> {
        const NODE_NAME: &'static str = "element (normal) appinfo_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            appinfo_e_inner::parse_xml(stream, parse_context, parent_context).map(appinfo_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion_e<'input>(super::UNQUAL::assertion<'input>);

    // ^-- from Element { name: Some(QName(None, "assertion")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for assertion_e<'input> {
        const NODE_NAME: &'static str = "element (custom) assertion_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::assertion::parse_xml(stream, parse_context, parent_context).map(assertion_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute_e<'input>(super::UNQUAL::topLevelAttribute<'input>);

    // ^-- from Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelAttribute"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for attribute_e<'input> {
        const NODE_NAME: &'static str = "element (custom) attribute_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelAttribute::parse_xml(stream, parse_context, parent_context).map(attribute_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup_e<'input>(super::UNQUAL::namedAttributeGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedAttributeGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for attributeGroup_e<'input> {
        const NODE_NAME: &'static str = "element (custom) attributeGroup_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::namedAttributeGroup::parse_xml(stream, parse_context, parent_context).map(attributeGroup_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct choice_e<'input>(super::UNQUAL::explicitGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "choice")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for choice_e<'input> {
        const NODE_NAME: &'static str = "element (custom) choice_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::explicitGroup::parse_xml(stream, parse_context, parent_context).map(choice_e)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum complexContent_e_inner__extension<'input> {
        restriction(Box<super::UNQUAL::complexRestrictionType<'input>>),
        extension(Box<super::UNQUAL::extensionType<'input>>),
    }

    impl<'input> Default for complexContent_e_inner__extension<'input> { fn default() -> complexContent_e_inner__extension<'input> { complexContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for complexContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "choice complexContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::complexRestrictionType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexContent_e_inner__extension::restriction(Box::new(r))), None => () }



            match super::UNQUAL::extensionType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexContent_e_inner__extension::extension(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: complexContent_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for complexContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension complexContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(complexContent_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: complexContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e<'input>(complexContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "complexContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "mixed", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "boolean"))), min_occurs: None, max_occurs: None } }], Some(Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for complexContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            complexContent_e_inner::parse_xml(stream, parse_context, parent_context).map(complexContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType_e<'input>(super::UNQUAL::topLevelComplexType<'input>);

    // ^-- from Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelComplexType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for complexType_e<'input> {
        const NODE_NAME: &'static str = "element (custom) complexType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelComplexType::parse_xml(stream, parse_context, parent_context).map(complexType_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension<'input> {
        any: super::UNQUAL::wildcard<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence defaultOpenContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(defaultOpenContent_e_inner__extension {



                any: super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: defaultOpenContent_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for defaultOpenContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension defaultOpenContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(defaultOpenContent_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: defaultOpenContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e<'input>(defaultOpenContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "defaultOpenContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "appliesToEmpty", type_: Some(QName(Some("xs"), "boolean")), default: Some("false") }, LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))), min_occurs: None, max_occurs: None } }], Some(Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for defaultOpenContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) defaultOpenContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            defaultOpenContent_e_inner::parse_xml(stream, parse_context, parent_context).map(defaultOpenContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e_inner<'input> {
        seqfield0: any<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for documentation_e_inner<'input> {
        const NODE_NAME: &'static str = "sequence documentation_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(documentation_e_inner {



                seqfield0: any::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e<'input>(documentation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "documentation")), attrs: [SmallDef { name: "source", type_: Some(QName(Some("xs"), "anyURI")), default: None }, Ref(QName(Some("xml"), "lang")), Any], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for documentation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) documentation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            documentation_e_inner::parse_xml(stream, parse_context, parent_context).map(documentation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct element_e<'input>(super::UNQUAL::topLevelElement<'input>);

    // ^-- from Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelElement"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for element_e<'input> {
        const NODE_NAME: &'static str = "element (custom) element_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelElement::parse_xml(stream, parse_context, parent_context).map(element_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct enumeration_e<'input>(super::UNQUAL::noFixedFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "enumeration")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for enumeration_e<'input> {
        const NODE_NAME: &'static str = "element (custom) enumeration_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::noFixedFacet::parse_xml(stream, parse_context, parent_context).map(enumeration_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitTimezone_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "explicitTimezone")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for explicitTimezone_e<'input> {
        const NODE_NAME: &'static str = "element (custom) explicitTimezone_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(explicitTimezone_e)
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
    pub struct field_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for field_e_inner<'input> {
        const NODE_NAME: &'static str = "extension field_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(field_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct field_e<'input>(field_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "field")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for field_e<'input> {
        const NODE_NAME: &'static str = "element (normal) field_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            field_e_inner::parse_xml(stream, parse_context, parent_context).map(field_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct fractionDigits_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "fractionDigits")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for fractionDigits_e<'input> {
        const NODE_NAME: &'static str = "element (custom) fractionDigits_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(fractionDigits_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct group_e<'input>(super::UNQUAL::namedGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for group_e<'input> {
        const NODE_NAME: &'static str = "element (custom) group_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::namedGroup::parse_xml(stream, parse_context, parent_context).map(group_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for import_e_inner<'input> {
        const NODE_NAME: &'static str = "extension import_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(import_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct import_e<'input>(import_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "import")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "namespace", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for import_e<'input> {
        const NODE_NAME: &'static str = "element (normal) import_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            import_e_inner::parse_xml(stream, parse_context, parent_context).map(import_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for include_e_inner<'input> {
        const NODE_NAME: &'static str = "extension include_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(include_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct include_e<'input>(include_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "include")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for include_e<'input> {
        const NODE_NAME: &'static str = "element (normal) include_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            include_e_inner::parse_xml(stream, parse_context, parent_context).map(include_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct key_e<'input>(super::UNQUAL::keybase<'input>);

    // ^-- from Element { name: Some(QName(None, "key")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for key_e<'input> {
        const NODE_NAME: &'static str = "element (custom) key_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::keybase::parse_xml(stream, parse_context, parent_context).map(key_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e_inner<'input> {
        BASE: super::UNQUAL::keybase<'input>,
    }


    impl<'input> ParseXml<'input> for keyref_e_inner<'input> {
        const NODE_NAME: &'static str = "extension keyref_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(keyref_e_inner {
                BASE: super::UNQUAL::keybase::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e<'input>(keyref_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "keyref")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "keybase"), [SmallDef { name: "refer", type_: Some(QName(Some("xs"), "QName")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for keyref_e<'input> {
        const NODE_NAME: &'static str = "element (normal) keyref_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            keyref_e_inner::parse_xml(stream, parse_context, parent_context).map(keyref_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct length_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "length")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for length_e<'input> {
        const NODE_NAME: &'static str = "element (custom) length_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(length_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for list_e_inner__extension__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (custom) list_e_inner__extension__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(list_e_inner__extension__simpleType_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension<'input> {
        simpleType: list_e_inner__extension__simpleType_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }])


    impl<'input> ParseXml<'input> for list_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence list_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(list_e_inner__extension {



                simpleType: list_e_inner__extension__simpleType_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: list_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for list_e_inner<'input> {
        const NODE_NAME: &'static str = "extension list_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(list_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: list_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct list_e<'input>(list_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "list")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "itemType", type_: Some(QName(Some("xs"), "QName")), default: None }], Some(Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for list_e<'input> {
        const NODE_NAME: &'static str = "element (normal) list_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            list_e_inner::parse_xml(stream, parse_context, parent_context).map(list_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxExclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxExclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for maxExclusive_e<'input> {
        const NODE_NAME: &'static str = "element (custom) maxExclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(maxExclusive_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxInclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxInclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for maxInclusive_e<'input> {
        const NODE_NAME: &'static str = "element (custom) maxInclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(maxInclusive_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxLength_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxLength")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for maxLength_e<'input> {
        const NODE_NAME: &'static str = "element (custom) maxLength_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(maxLength_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minExclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "minExclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for minExclusive_e<'input> {
        const NODE_NAME: &'static str = "element (custom) minExclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(minExclusive_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minInclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "minInclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for minInclusive_e<'input> {
        const NODE_NAME: &'static str = "element (custom) minInclusive_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(minInclusive_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct minLength_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "minLength")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for minLength_e<'input> {
        const NODE_NAME: &'static str = "element (custom) minLength_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(minLength_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for notation_e_inner<'input> {
        const NODE_NAME: &'static str = "extension notation_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(notation_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e<'input>(notation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "notation")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "name", type_: Some(QName(Some("xs"), "NCName")), default: None }, SmallDef { name: "public", type_: Some(QName(Some("xs"), "public")), default: None }, SmallDef { name: "system", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for notation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) notation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            notation_e_inner::parse_xml(stream, parse_context, parent_context).map(notation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__any_e<'input>(super::UNQUAL::wildcard<'input>);

    // ^-- from Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for openContent_e_inner__extension__any_e<'input> {
        const NODE_NAME: &'static str = "element (custom) openContent_e_inner__extension__any_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::wildcard::parse_xml(stream, parse_context, parent_context).map(openContent_e_inner__extension__any_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension<'input> {
        any: openContent_e_inner__extension__any_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }])


    impl<'input> ParseXml<'input> for openContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence openContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(openContent_e_inner__extension {



                any: openContent_e_inner__extension__any_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: openContent_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for openContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension openContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(openContent_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: openContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e<'input>(openContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "openContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))), min_occurs: None, max_occurs: None } }], Some(Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for openContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) openContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            openContent_e_inner::parse_xml(stream, parse_context, parent_context).map(openContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for override_e_inner__extension__annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) override_e_inner__extension__annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context).map(override_e_inner__extension__annotation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__schemaTop_e<'input>(super::UNQUAL::schemaTop<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for override_e_inner__extension__schemaTop_e<'input> {
        const NODE_NAME: &'static str = "element (normal) override_e_inner__extension__schemaTop_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::schemaTop::parse_xml(stream, parse_context, parent_context).map(override_e_inner__extension__schemaTop_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension<'input> {
        seqfield0: override_e_inner__extension__annotation_e<'input>,
        seqfield1: override_e_inner__extension__schemaTop_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for override_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence override_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(override_e_inner__extension {



                seqfield0: override_e_inner__extension__annotation_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: override_e_inner__extension__schemaTop_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: override_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for override_e_inner<'input> {
        const NODE_NAME: &'static str = "extension override_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(override_e_inner {
                BASE: super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: override_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct override_e<'input>(override_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "override")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for override_e<'input> {
        const NODE_NAME: &'static str = "element (normal) override_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            override_e_inner::parse_xml(stream, parse_context, parent_context).map(override_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct pattern_e<'input>(super::UNQUAL::noFixedFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "pattern")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for pattern_e<'input> {
        const NODE_NAME: &'static str = "element (custom) pattern_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::noFixedFacet::parse_xml(stream, parse_context, parent_context).map(pattern_e)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum redefine_e_inner__extension<'input> {
        annotation(Box<super::UNQUAL::annotation_e<'input>>),
        redefinable(Box<super::UNQUAL::redefinable<'input>>),
    }

    impl<'input> Default for redefine_e_inner__extension<'input> { fn default() -> redefine_e_inner__extension<'input> { redefine_e_inner__extension::redefinable(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for redefine_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "choice redefine_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefine_e_inner__extension::annotation(Box::new(r))), None => () }



            match super::UNQUAL::redefinable::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefine_e_inner__extension::redefinable(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: redefine_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for redefine_e_inner<'input> {
        const NODE_NAME: &'static str = "extension redefine_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(redefine_e_inner {
                BASE: super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: redefine_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e<'input>(redefine_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "redefine")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for redefine_e<'input> {
        const NODE_NAME: &'static str = "element (normal) redefine_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            redefine_e_inner::parse_xml(stream, parse_context, parent_context).map(redefine_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::simpleRestrictionModel<'input>,
    }


    impl<'input> ParseXml<'input> for restriction_e_inner<'input> {
        const NODE_NAME: &'static str = "extension restriction_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(restriction_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: super::UNQUAL::simpleRestrictionModel::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e<'input>(restriction_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "base", type_: Some(QName(Some("xs"), "QName")), default: None }], Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for restriction_e<'input> {
        const NODE_NAME: &'static str = "element (normal) restriction_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            restriction_e_inner::parse_xml(stream, parse_context, parent_context).map(restriction_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__composition_e<'input>(super::UNQUAL::composition<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for schema_e_inner__extension__composition_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e_inner__extension__composition_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::composition::parse_xml(stream, parse_context, parent_context).map(schema_e_inner__extension__composition_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1_e_inner__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield1_e_inner__annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e_inner__extension__seqfield1_e_inner__annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context).map(schema_e_inner__extension__seqfield1_e_inner__annotation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1_e_inner<'input> {
        defaultOpenContent: super::UNQUAL::defaultOpenContent_e<'input>,
        seqfield1: schema_e_inner__extension__seqfield1_e_inner__annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield1_e_inner<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension__seqfield1_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(schema_e_inner__extension__seqfield1_e_inner {



                defaultOpenContent: super::UNQUAL::defaultOpenContent_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: schema_e_inner__extension__seqfield1_e_inner__annotation_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1_e<'input>(schema_e_inner__extension__seqfield1_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield1_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e_inner__extension__seqfield1_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            schema_e_inner__extension__seqfield1_e_inner::parse_xml(stream, parse_context, parent_context).map(schema_e_inner__extension__seqfield1_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2_e_inner__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield2_e_inner__annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e_inner__extension__seqfield2_e_inner__annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context).map(schema_e_inner__extension__seqfield2_e_inner__annotation_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2_e_inner<'input> {
        schemaTop: super::UNQUAL::schemaTop<'input>,
        seqfield1: schema_e_inner__extension__seqfield2_e_inner__annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield2_e_inner<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension__seqfield2_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(schema_e_inner__extension__seqfield2_e_inner {



                schemaTop: super::UNQUAL::schemaTop::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: schema_e_inner__extension__seqfield2_e_inner__annotation_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2_e<'input>(schema_e_inner__extension__seqfield2_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for schema_e_inner__extension__seqfield2_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e_inner__extension__seqfield2_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            schema_e_inner__extension__seqfield2_e_inner::parse_xml(stream, parse_context, parent_context).map(schema_e_inner__extension__seqfield2_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension<'input> {
        seqfield0: schema_e_inner__extension__composition_e<'input>,
        seqfield1: schema_e_inner__extension__seqfield1_e<'input>,
        seqfield2: schema_e_inner__extension__seqfield2_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for schema_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence schema_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(schema_e_inner__extension {



                seqfield0: schema_e_inner__extension__composition_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: schema_e_inner__extension__seqfield1_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield2: schema_e_inner__extension__seqfield2_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: schema_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for schema_e_inner<'input> {
        const NODE_NAME: &'static str = "extension schema_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(schema_e_inner {
                BASE: super::UNQUAL::openAttrs::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: schema_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e<'input>(schema_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "schema")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "targetNamespace", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "version", type_: Some(QName(Some("xs"), "token")), default: None }, SmallDef { name: "finalDefault", type_: Some(QName(Some("xs"), "fullDerivationSet")), default: Some("") }, SmallDef { name: "blockDefault", type_: Some(QName(Some("xs"), "blockSet")), default: Some("") }, SmallDef { name: "attributeFormDefault", type_: Some(QName(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "elementFormDefault", type_: Some(QName(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "defaultAttributes", type_: Some(QName(Some("xs"), "QName")), default: None }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: Some("##local") }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }, Ref(QName(Some("xml"), "lang"))], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for schema_e<'input> {
        const NODE_NAME: &'static str = "element (normal) schema_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            schema_e_inner::parse_xml(stream, parse_context, parent_context).map(schema_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }


    impl<'input> ParseXml<'input> for selector_e_inner<'input> {
        const NODE_NAME: &'static str = "extension selector_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(selector_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e<'input>(selector_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "selector")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: None }], None)), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for selector_e<'input> {
        const NODE_NAME: &'static str = "element (normal) selector_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            selector_e_inner::parse_xml(stream, parse_context, parent_context).map(selector_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct sequence_e<'input>(super::UNQUAL::explicitGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "sequence")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for sequence_e<'input> {
        const NODE_NAME: &'static str = "element (custom) sequence_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::explicitGroup::parse_xml(stream, parse_context, parent_context).map(sequence_e)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum simpleContent_e_inner__extension<'input> {
        restriction(Box<super::UNQUAL::simpleRestrictionType<'input>>),
        extension(Box<super::UNQUAL::simpleExtensionType<'input>>),
    }

    impl<'input> Default for simpleContent_e_inner__extension<'input> { fn default() -> simpleContent_e_inner__extension<'input> { simpleContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for simpleContent_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "choice simpleContent_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::simpleRestrictionType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleContent_e_inner__extension::restriction(Box::new(r))), None => () }



            match super::UNQUAL::simpleExtensionType::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleContent_e_inner__extension::extension(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: simpleContent_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for simpleContent_e_inner<'input> {
        const NODE_NAME: &'static str = "extension simpleContent_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(simpleContent_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: simpleContent_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e<'input>(simpleContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [], Some(Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for simpleContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            simpleContent_e_inner::parse_xml(stream, parse_context, parent_context).map(simpleContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType_e<'input>(super::UNQUAL::topLevelSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelSimpleType"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (custom) simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::topLevelSimpleType::parse_xml(stream, parse_context, parent_context).map(simpleType_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct totalDigits_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "totalDigits")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for totalDigits_e<'input> {
        const NODE_NAME: &'static str = "element (custom) totalDigits_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::numFacet::parse_xml(stream, parse_context, parent_context).map(totalDigits_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for union_e_inner__extension__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (custom) union_e_inner__extension__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(union_e_inner__extension__simpleType_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension<'input> {
        simpleType: union_e_inner__extension__simpleType_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for union_e_inner__extension<'input> {
        const NODE_NAME: &'static str = "sequence union_e_inner__extension";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(union_e_inner__extension {



                simpleType: union_e_inner__extension__simpleType_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: union_e_inner__extension<'input>,
    }


    impl<'input> ParseXml<'input> for union_e_inner<'input> {
        const NODE_NAME: &'static str = "extension union_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(union_e_inner {
                BASE: super::UNQUAL::annotated::parse_xml(stream, parse_context, parent_context)?,



                EXTENSION: union_e_inner__extension::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }


    #[derive(Debug, PartialEq, Default)]
    pub struct union_e<'input>(union_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "union")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "memberTypes", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(List(SimpleList(QName(Some("xs"), "QName")))), min_occurs: None, max_occurs: None } }], Some(Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for union_e<'input> {
        const NODE_NAME: &'static str = "element (normal) union_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            union_e_inner::parse_xml(stream, parse_context, parent_context).map(union_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct unique_e<'input>(super::UNQUAL::keybase<'input>);

    // ^-- from Element { name: Some(QName(None, "unique")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for unique_e<'input> {
        const NODE_NAME: &'static str = "element (custom) unique_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::keybase::parse_xml(stream, parse_context, parent_context).map(unique_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct whiteSpace_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "whiteSpace")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    impl<'input> ParseXml<'input> for whiteSpace_e<'input> {
        const NODE_NAME: &'static str = "element (custom) whiteSpace_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::facet::parse_xml(stream, parse_context, parent_context).map(whiteSpace_e)
        }
    }


    /////////// groups


    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2__openContent_e<'input>(super::UNQUAL::openContent_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for complexTypeModel__choicevariant2__openContent_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexTypeModel__choicevariant2__openContent_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::openContent_e::parse_xml(stream, parse_context, parent_context).map(complexTypeModel__choicevariant2__openContent_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2__typeDefParticle_e<'input>(super::UNQUAL::typeDefParticle<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for complexTypeModel__choicevariant2__typeDefParticle_e<'input> {
        const NODE_NAME: &'static str = "element (normal) complexTypeModel__choicevariant2__typeDefParticle_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::typeDefParticle::parse_xml(stream, parse_context, parent_context).map(complexTypeModel__choicevariant2__typeDefParticle_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2<'input> {
        seqfield0: complexTypeModel__choicevariant2__openContent_e<'input>,
        seqfield1: complexTypeModel__choicevariant2__typeDefParticle_e<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for complexTypeModel__choicevariant2<'input> {
        const NODE_NAME: &'static str = "sequence complexTypeModel__choicevariant2";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(complexTypeModel__choicevariant2 {



                seqfield0: complexTypeModel__choicevariant2__openContent_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: complexTypeModel__choicevariant2__typeDefParticle_e::parse_xml(stream, parse_context, parent_context)?,



                attrDecls: super::UNQUAL::attrDecls::parse_xml(stream, parse_context, parent_context)?,



                assertions: super::UNQUAL::assertions::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum complexTypeModel<'input> {
        simpleContent(Box<super::UNQUAL::simpleContent_e<'input>>),
        complexContent(Box<super::UNQUAL::complexContent_e<'input>>),
        choicevariant2(Box<complexTypeModel__choicevariant2<'input>>),
    }

    impl<'input> Default for complexTypeModel<'input> { fn default() -> complexTypeModel<'input> { complexTypeModel::choicevariant2(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "simpleContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "complexContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for complexTypeModel<'input> {
        const NODE_NAME: &'static str = "choice complexTypeModel";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::simpleContent_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexTypeModel::simpleContent(Box::new(r))), None => () }



            match super::UNQUAL::complexContent_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexTypeModel::complexContent(Box::new(r))), None => () }



            match complexTypeModel__choicevariant2::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(complexTypeModel::choicevariant2(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum nestedParticle<'input> {
        element(Box<super::UNQUAL::localElement<'input>>),
        group(Box<super::UNQUAL::groupRef<'input>>),
        choice(Box<super::UNQUAL::choice_e<'input>>),
        sequence(Box<super::UNQUAL::sequence_e<'input>>),
        any(Box<super::UNQUAL::any_e<'input>>),
    }

    impl<'input> Default for nestedParticle<'input> { fn default() -> nestedParticle<'input> { nestedParticle::any(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for nestedParticle<'input> {
        const NODE_NAME: &'static str = "choice nestedParticle";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::localElement::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::element(Box::new(r))), None => () }



            match super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::group(Box::new(r))), None => () }



            match super::UNQUAL::choice_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::choice(Box::new(r))), None => () }



            match super::UNQUAL::sequence_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::sequence(Box::new(r))), None => () }



            match super::UNQUAL::any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(nestedParticle::any(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum attrDecls__seqfield0_e_inner<'input> {
        attribute(Box<super::UNQUAL::attribute<'input>>),
        attributeGroup(Box<super::UNQUAL::attributeGroupRef<'input>>),
    }

    impl<'input> Default for attrDecls__seqfield0_e_inner<'input> { fn default() -> attrDecls__seqfield0_e_inner<'input> { attrDecls__seqfield0_e_inner::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_e_inner<'input> {
        const NODE_NAME: &'static str = "choice attrDecls__seqfield0_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::attribute::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(attrDecls__seqfield0_e_inner::attribute(Box::new(r))), None => () }



            match super::UNQUAL::attributeGroupRef::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(attrDecls__seqfield0_e_inner::attributeGroup(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_e<'input>(attrDecls__seqfield0_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for attrDecls__seqfield0_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attrDecls__seqfield0_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            attrDecls__seqfield0_e_inner::parse_xml(stream, parse_context, parent_context).map(attrDecls__seqfield0_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__anyAttribute_e<'input>(super::UNQUAL::anyAttribute_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "anyAttribute"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for attrDecls__anyAttribute_e<'input> {
        const NODE_NAME: &'static str = "element (normal) attrDecls__anyAttribute_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::anyAttribute_e::parse_xml(stream, parse_context, parent_context).map(attrDecls__anyAttribute_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls<'input> {
        seqfield0: attrDecls__seqfield0_e<'input>,
        seqfield1: attrDecls__anyAttribute_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "anyAttribute"))), min_occurs: Some(0), max_occurs: None }])


    impl<'input> ParseXml<'input> for attrDecls<'input> {
        const NODE_NAME: &'static str = "sequence attrDecls";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(attrDecls {



                seqfield0: attrDecls__seqfield0_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: attrDecls__anyAttribute_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for allModel__annotation_e<'input> {
        const NODE_NAME: &'static str = "element (normal) allModel__annotation_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context).map(allModel__annotation_e)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum allModel__seqfield1_e_inner<'input> {
        element(Box<super::UNQUAL::localElement<'input>>),
        any(Box<super::UNQUAL::any_e<'input>>),
        group(Box<super::UNQUAL::groupRef<'input>>),
    }

    impl<'input> Default for allModel__seqfield1_e_inner<'input> { fn default() -> allModel__seqfield1_e_inner<'input> { allModel__seqfield1_e_inner::group(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for allModel__seqfield1_e_inner<'input> {
        const NODE_NAME: &'static str = "choice allModel__seqfield1_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::localElement::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(allModel__seqfield1_e_inner::element(Box::new(r))), None => () }



            match super::UNQUAL::any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(allModel__seqfield1_e_inner::any(Box::new(r))), None => () }



            match super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(allModel__seqfield1_e_inner::group(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_e<'input>(allModel__seqfield1_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for allModel__seqfield1_e<'input> {
        const NODE_NAME: &'static str = "element (normal) allModel__seqfield1_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            allModel__seqfield1_e_inner::parse_xml(stream, parse_context, parent_context).map(allModel__seqfield1_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel<'input> {
        seqfield0: allModel__annotation_e<'input>,
        seqfield1: allModel__seqfield1_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for allModel<'input> {
        const NODE_NAME: &'static str = "sequence allModel";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(allModel {



                seqfield0: allModel__annotation_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: allModel__seqfield1_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }


    impl<'input> ParseXml<'input> for simpleRestrictionModel__simpleType_e<'input> {
        const NODE_NAME: &'static str = "element (custom) simpleRestrictionModel__simpleType_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::localSimpleType::parse_xml(stream, parse_context, parent_context).map(simpleRestrictionModel__simpleType_e)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum simpleRestrictionModel__seqfield1_e_inner<'input> {
        facet(Box<super::UNQUAL::facet_e<'input>>),
        choicevariant1(Box<any<'input>>),
    }

    impl<'input> Default for simpleRestrictionModel__seqfield1_e_inner<'input> { fn default() -> simpleRestrictionModel__seqfield1_e_inner<'input> { simpleRestrictionModel__seqfield1_e_inner::choicevariant1(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield1_e_inner<'input> {
        const NODE_NAME: &'static str = "choice simpleRestrictionModel__seqfield1_e_inner";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::facet_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleRestrictionModel__seqfield1_e_inner::facet(Box::new(r))), None => () }



            match any::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleRestrictionModel__seqfield1_e_inner::choicevariant1(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield1_e<'input>(simpleRestrictionModel__seqfield1_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for simpleRestrictionModel__seqfield1_e<'input> {
        const NODE_NAME: &'static str = "element (normal) simpleRestrictionModel__seqfield1_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            simpleRestrictionModel__seqfield1_e_inner::parse_xml(stream, parse_context, parent_context).map(simpleRestrictionModel__seqfield1_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel<'input> {
        simpleType: simpleRestrictionModel__simpleType_e<'input>,
        seqfield1: simpleRestrictionModel__seqfield1_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for simpleRestrictionModel<'input> {
        const NODE_NAME: &'static str = "sequence simpleRestrictionModel";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(simpleRestrictionModel {



                simpleType: simpleRestrictionModel__simpleType_e::parse_xml(stream, parse_context, parent_context)?,



                seqfield1: simpleRestrictionModel__seqfield1_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct occurs<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum particle<'input> {
        element(Box<super::UNQUAL::localElement<'input>>),
        group(Box<super::UNQUAL::groupRef<'input>>),
        all(Box<super::UNQUAL::all_e<'input>>),
        choice(Box<super::UNQUAL::choice_e<'input>>),
        sequence(Box<super::UNQUAL::sequence_e<'input>>),
        any(Box<super::UNQUAL::any_e<'input>>),
    }

    impl<'input> Default for particle<'input> { fn default() -> particle<'input> { particle::any(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "all"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for particle<'input> {
        const NODE_NAME: &'static str = "choice particle";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::localElement::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::element(Box::new(r))), None => () }



            match super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::group(Box::new(r))), None => () }



            match super::UNQUAL::all_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::all(Box::new(r))), None => () }



            match super::UNQUAL::choice_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::choice(Box::new(r))), None => () }



            match super::UNQUAL::sequence_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::sequence(Box::new(r))), None => () }



            match super::UNQUAL::any_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(particle::any(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defRef<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum identityConstraint<'input> {
        unique(Box<super::UNQUAL::unique_e<'input>>),
        key(Box<super::UNQUAL::key_e<'input>>),
        keyref(Box<super::UNQUAL::keyref_e<'input>>),
    }

    impl<'input> Default for identityConstraint<'input> { fn default() -> identityConstraint<'input> { identityConstraint::keyref(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "unique"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "key"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "keyref"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for identityConstraint<'input> {
        const NODE_NAME: &'static str = "choice identityConstraint";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::unique_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(identityConstraint::unique(Box::new(r))), None => () }



            match super::UNQUAL::key_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(identityConstraint::key(Box::new(r))), None => () }



            match super::UNQUAL::keyref_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(identityConstraint::keyref(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions__assert_e<'input>(super::UNQUAL::assertion<'input>);

    // ^-- from Element { name: Some(QName(None, "assert")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }


    impl<'input> ParseXml<'input> for assertions__assert_e<'input> {
        const NODE_NAME: &'static str = "element (custom) assertions__assert_e";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            super::UNQUAL::assertion::parse_xml(stream, parse_context, parent_context).map(assertions__assert_e)
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions<'input> {
        assert: assertions__assert_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "assert")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])


    impl<'input> ParseXml<'input> for assertions<'input> {
        const NODE_NAME: &'static str = "sequence assertions";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {
            Some(assertions {



                assert: assertions__assert_e::parse_xml(stream, parse_context, parent_context)?,



            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum schemaTop<'input> {
        redefinable(Box<super::UNQUAL::redefinable<'input>>),
        element(Box<super::UNQUAL::element_e<'input>>),
        attribute(Box<super::UNQUAL::attribute_e<'input>>),
        notation(Box<super::UNQUAL::notation_e<'input>>),
    }

    impl<'input> Default for schemaTop<'input> { fn default() -> schemaTop<'input> { schemaTop::notation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "element"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "notation"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for schemaTop<'input> {
        const NODE_NAME: &'static str = "choice schemaTop";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::redefinable::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::redefinable(Box::new(r))), None => () }



            match super::UNQUAL::element_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::element(Box::new(r))), None => () }



            match super::UNQUAL::attribute_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::attribute(Box::new(r))), None => () }



            match super::UNQUAL::notation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(schemaTop::notation(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttrGroup<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum redefinable<'input> {
        simpleType(Box<super::UNQUAL::simpleType_e<'input>>),
        complexType(Box<super::UNQUAL::complexType_e<'input>>),
        group(Box<super::UNQUAL::group_e<'input>>),
        attributeGroup(Box<super::UNQUAL::attributeGroup_e<'input>>),
    }

    impl<'input> Default for redefinable<'input> { fn default() -> redefinable<'input> { redefinable::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "simpleType"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "complexType"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "group"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "attributeGroup"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for redefinable<'input> {
        const NODE_NAME: &'static str = "choice redefinable";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::simpleType_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::simpleType(Box::new(r))), None => () }



            match super::UNQUAL::complexType_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::complexType(Box::new(r))), None => () }



            match super::UNQUAL::group_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::group(Box::new(r))), None => () }



            match super::UNQUAL::attributeGroup_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(redefinable::attributeGroup(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum typeDefParticle<'input> {
        group(Box<super::UNQUAL::groupRef<'input>>),
        all(Box<super::UNQUAL::all_e<'input>>),
        choice(Box<super::UNQUAL::choice_e<'input>>),
        sequence(Box<super::UNQUAL::sequence_e<'input>>),
    }

    impl<'input> Default for typeDefParticle<'input> { fn default() -> typeDefParticle<'input> { typeDefParticle::sequence(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "all"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for typeDefParticle<'input> {
        const NODE_NAME: &'static str = "choice typeDefParticle";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::groupRef::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::group(Box::new(r))), None => () }



            match super::UNQUAL::all_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::all(Box::new(r))), None => () }



            match super::UNQUAL::choice_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::choice(Box::new(r))), None => () }



            match super::UNQUAL::sequence_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(typeDefParticle::sequence(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum composition<'input> {
        include(Box<super::UNQUAL::include_e<'input>>),
        import(Box<super::UNQUAL::import_e<'input>>),
        redefine(Box<super::UNQUAL::redefine_e<'input>>),
        override_(Box<super::UNQUAL::override_e<'input>>),
        annotation(Box<super::UNQUAL::annotation_e<'input>>),
    }

    impl<'input> Default for composition<'input> { fn default() -> composition<'input> { composition::annotation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "include"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "import"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "redefine"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "override"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for composition<'input> {
        const NODE_NAME: &'static str = "choice composition";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::include_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::include(Box::new(r))), None => () }



            match super::UNQUAL::import_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::import(Box::new(r))), None => () }



            match super::UNQUAL::redefine_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::redefine(Box::new(r))), None => () }



            match super::UNQUAL::override_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::override_(Box::new(r))), None => () }



            match super::UNQUAL::annotation_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(composition::annotation(Box::new(r))), None => () }



            None
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum simpleDerivation<'input> {
        restriction(Box<super::UNQUAL::restriction_e<'input>>),
        list(Box<super::UNQUAL::list_e<'input>>),
        union(Box<super::UNQUAL::union_e<'input>>),
    }

    impl<'input> Default for simpleDerivation<'input> { fn default() -> simpleDerivation<'input> { simpleDerivation::union(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "restriction"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "list"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "union"))), min_occurs: None, max_occurs: None }])


    impl<'input> ParseXml<'input> for simpleDerivation<'input> {
        const NODE_NAME: &'static str = "choice simpleDerivation";
        fn parse_self_xml<TParseContext, TParentContext>(stream: &mut Stream<'input>, parse_context: &mut TParseContext, parent_context: &TParentContext) -> Option<Self> {



            match super::UNQUAL::restriction_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleDerivation::restriction(Box::new(r))), None => () }



            match super::UNQUAL::list_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleDerivation::list(Box::new(r))), None => () }



            match super::UNQUAL::union_e::parse_xml(stream, parse_context, parent_context) { Some(r) => return Some(simpleDerivation::union(Box::new(r))), None => () }



            None
        }
    }
}
