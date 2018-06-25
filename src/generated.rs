pub mod UNQUAL {
    use std::marker::PhantomData;

    use support::*;


    /////////// types


    #[derive(Debug, PartialEq, Default)]
    pub struct all<'input>(super::UNQUAL::explicitGroup<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct allNNI__item0<'input>(super::UNQUAL::NMTOKEN<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct altType_item<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: altType_item__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct altType<'input>(Option<altType_item<'input>>);

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension<'input> {
        seqfield0: annotated__extension__annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotated__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyType__seqfield0_e<'input>(SUPPORT_ANY<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyType<'input> {
        seqfield0: anyType__seqfield0_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension<'input> {
        simpleType: attribute__extension__simpleType_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: attribute__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::attrDecls<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroupRef<'input>(super::UNQUAL::attributeGroup<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList__valuetype__item0<'input>(super::UNQUAL::token<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1__valuetype<'input>(super::UNQUAL::derivationControl<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::complexTypeModel<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationControl<'input>(super::UNQUAL::NMTOKEN<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet__item0<'input>(super::UNQUAL::token<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__seqfield0_e<'input>(element__extension__seqfield0_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__alternative_e<'input>(super::UNQUAL::altType<'input>);

    // ^-- from Element { name: Some(QName(None, "alternative")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension__identityConstraint_e<'input>(super::UNQUAL::identityConstraint<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "identityConstraint"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension<'input> {
        seqfield0: element__extension__seqfield0_e<'input>,
        alternative: element__extension__alternative_e<'input>,
        seqfield2: element__extension__identityConstraint_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }, Element { name: Some(QName(None, "alternative")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "identityConstraint"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct element<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: element__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitGroup<'input>(super::UNQUAL::group<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension__openContent_e<'input>(super::UNQUAL::openContent_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension__typeDefParticle_e<'input>(super::UNQUAL::typeDefParticle<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension<'input> {
        seqfield0: extensionType__extension__openContent_e<'input>,
        seqfield1: extensionType__extension__typeDefParticle_e<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: extensionType__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct formChoice<'input>(super::UNQUAL::NMTOKEN<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet__item0<'input>(super::UNQUAL::token<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct group<'input>(Vec<group_item<'input>>);

    #[derive(Debug, PartialEq, Default)]
    pub struct groupRef<'input>(super::UNQUAL::realGroup<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct intFacet<'input>(super::UNQUAL::facet<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase_item__extension__field_e<'input>(super::UNQUAL::field_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "field"))), min_occurs: Some(1), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase_item__extension<'input> {
        selector: super::UNQUAL::selector_e<'input>,
        seqfield1: keybase_item__extension__field_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "selector"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "field"))), min_occurs: Some(1), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase_item<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: keybase_item__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase<'input>(Option<keybase_item<'input>>);

    #[derive(Debug, PartialEq, Default)]
    pub struct localComplexType<'input>(super::UNQUAL::complexType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct localElement<'input>(super::UNQUAL::element<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct localSimpleType<'input>(super::UNQUAL::simpleType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct namedAttributeGroup<'input>(super::UNQUAL::attributeGroup<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct namedGroup<'input>(super::UNQUAL::realGroup<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct namespaceList<'input> {
        member0: super::UNQUAL::specialNamespaceList<'input>,
        member1: super::UNQUAL::basicNamespaceList<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "specialNamespaceList"), QName(Some("xs"), "basicNamespaceList")]), None)

    #[derive(Debug, PartialEq, Default)]
    pub struct noFixedFacet<'input>(super::UNQUAL::facet<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct numFacet<'input>(super::UNQUAL::facet<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct openAttrs<'input>(super::UNQUAL::anyType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct public<'input>(super::UNQUAL::token<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList__valuetype__item0<'input>(super::UNQUAL::token<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct reducedDerivationControl<'input>(super::UNQUAL::derivationControl<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e<'input>(super::UNQUAL::openContent_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0_e_inner__choicevariant0<'input> {
        seqfield0: restrictionType__extension__seqfield0_e_inner__choicevariant0__openContent_e<'input>,
        typeDefParticle: super::UNQUAL::typeDefParticle<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq)]
    pub enum restrictionType__extension__seqfield0_e_inner<'input> {
        choicevariant0(Box<restrictionType__extension__seqfield0_e_inner__choicevariant0<'input>>),
        simpleRestrictionModel(Box<super::UNQUAL::simpleRestrictionModel<'input>>),
    }

    impl<'input> Default for restrictionType__extension__seqfield0_e_inner<'input> { fn default() -> restrictionType__extension__seqfield0_e_inner<'input> { restrictionType__extension__seqfield0_e_inner::simpleRestrictionModel(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0_e<'input>(restrictionType__extension__seqfield0_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension<'input> {
        seqfield0: restrictionType__extension__seqfield0_e<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: restrictionType__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item0<'input>(super::UNQUAL::token<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item1__valuetype<'input>(super::UNQUAL::derivationControl<'input>);

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

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleExtensionType<'input>(super::UNQUAL::extensionType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionType<'input>(super::UNQUAL::restrictionType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::simpleDerivation<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct specialNamespaceList<'input>(super::UNQUAL::token<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelAttribute<'input>(super::UNQUAL::attribute<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelComplexType<'input>(super::UNQUAL::complexType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelElement<'input>(super::UNQUAL::element<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelSimpleType<'input>(super::UNQUAL::simpleType<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct typeDerivationControl<'input>(super::UNQUAL::derivationControl<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct wildcard<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct xpathDefaultNamespace__item0<'input>(super::UNQUAL::token<'input>);

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

    #[derive(Debug, PartialEq)]
    pub enum annotation_e_inner__extension<'input> {
        appinfo(Box<super::UNQUAL::appinfo_e<'input>>),
        documentation(Box<super::UNQUAL::documentation_e<'input>>),
    }

    impl<'input> Default for annotation_e_inner__extension<'input> { fn default() -> annotation_e_inner__extension<'input> { annotation_e_inner__extension::documentation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "appinfo"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "documentation"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotation_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e<'input>(annotation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "annotation")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "appinfo"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "documentation"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e<'input>(any_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(QName(Some("xs"), "qnameList")), default: None }, GroupRef(QName(Some("xs"), "occurs"))], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e<'input>(anyAttribute_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "anyAttribute")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(QName(Some("xs"), "qnameListA")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e_inner<'input> {
        seqfield0: SUPPORT_ANY<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e<'input>(appinfo_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "appinfo")), attrs: [SmallDef { name: "source", type_: Some(QName(Some("xs"), "anyURI")), default: None }, Any], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion_e<'input>(super::UNQUAL::assertion<'input>);

    // ^-- from Element { name: Some(QName(None, "assertion")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute_e<'input>(super::UNQUAL::topLevelAttribute<'input>);

    // ^-- from Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelAttribute"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup_e<'input>(super::UNQUAL::namedAttributeGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedAttributeGroup"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct choice_e<'input>(super::UNQUAL::explicitGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "choice")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq)]
    pub enum complexContent_e_inner__extension<'input> {
        restriction(Box<super::UNQUAL::complexRestrictionType<'input>>),
        extension(Box<super::UNQUAL::extensionType<'input>>),
    }

    impl<'input> Default for complexContent_e_inner__extension<'input> { fn default() -> complexContent_e_inner__extension<'input> { complexContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: complexContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e<'input>(complexContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "complexContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "mixed", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "boolean"))), min_occurs: None, max_occurs: None } }], Some(Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType_e<'input>(super::UNQUAL::topLevelComplexType<'input>);

    // ^-- from Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelComplexType"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension<'input> {
        any: super::UNQUAL::wildcard<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: defaultOpenContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e<'input>(defaultOpenContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "defaultOpenContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "appliesToEmpty", type_: Some(QName(Some("xs"), "boolean")), default: Some("false") }, LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))), min_occurs: None, max_occurs: None } }], Some(Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e_inner<'input> {
        seqfield0: SUPPORT_ANY<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e<'input>(documentation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "documentation")), attrs: [SmallDef { name: "source", type_: Some(QName(Some("xs"), "anyURI")), default: None }, Ref(QName(Some("xml"), "lang")), Any], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct element_e<'input>(super::UNQUAL::topLevelElement<'input>);

    // ^-- from Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelElement"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct enumeration_e<'input>(super::UNQUAL::noFixedFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "enumeration")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitTimezone_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "explicitTimezone")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet_e<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e<'input>(field_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "field")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct fractionDigits_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "fractionDigits")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct group_e<'input>(super::UNQUAL::namedGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedGroup"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e<'input>(import_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "import")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "namespace", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e<'input>(include_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "include")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct key_e<'input>(super::UNQUAL::keybase<'input>);

    // ^-- from Element { name: Some(QName(None, "key")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e_inner<'input> {
        BASE: super::UNQUAL::keybase<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e<'input>(keyref_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "keyref")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "keybase"), [SmallDef { name: "refer", type_: Some(QName(Some("xs"), "QName")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct length_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "length")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension<'input> {
        simpleType: list_e_inner__extension__simpleType_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: list_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e<'input>(list_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "list")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "itemType", type_: Some(QName(Some("xs"), "QName")), default: None }], Some(Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxExclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxExclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxInclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxInclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxLength_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxLength")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct minExclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "minExclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct minInclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "minInclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct minLength_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "minLength")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e<'input>(notation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "notation")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "name", type_: Some(QName(Some("xs"), "NCName")), default: None }, SmallDef { name: "public", type_: Some(QName(Some("xs"), "public")), default: None }, SmallDef { name: "system", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension__any_e<'input>(super::UNQUAL::wildcard<'input>);

    // ^-- from Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension<'input> {
        any: openContent_e_inner__extension__any_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: openContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e<'input>(openContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "openContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))), min_occurs: None, max_occurs: None } }], Some(Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))), min_occurs: Some(0), max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension__schemaTop_e<'input>(super::UNQUAL::schemaTop<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension<'input> {
        seqfield0: override_e_inner__extension__annotation_e<'input>,
        seqfield1: override_e_inner__extension__schemaTop_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: override_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e<'input>(override_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "override")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct pattern_e<'input>(super::UNQUAL::noFixedFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "pattern")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq)]
    pub enum redefine_e_inner__extension<'input> {
        annotation(Box<super::UNQUAL::annotation_e<'input>>),
        redefinable(Box<super::UNQUAL::redefinable<'input>>),
    }

    impl<'input> Default for redefine_e_inner__extension<'input> { fn default() -> redefine_e_inner__extension<'input> { redefine_e_inner__extension::redefinable(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: redefine_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e<'input>(redefine_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "redefine")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::simpleRestrictionModel<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e<'input>(restriction_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "base", type_: Some(QName(Some("xs"), "QName")), default: None }], Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__composition_e<'input>(super::UNQUAL::composition<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1_e_inner__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1_e_inner<'input> {
        defaultOpenContent: super::UNQUAL::defaultOpenContent_e<'input>,
        seqfield1: schema_e_inner__extension__seqfield1_e_inner__annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1_e<'input>(schema_e_inner__extension__seqfield1_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2_e_inner__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2_e_inner<'input> {
        schemaTop: super::UNQUAL::schemaTop<'input>,
        seqfield1: schema_e_inner__extension__seqfield2_e_inner__annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2_e<'input>(schema_e_inner__extension__seqfield2_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension<'input> {
        seqfield0: schema_e_inner__extension__composition_e<'input>,
        seqfield1: schema_e_inner__extension__seqfield1_e<'input>,
        seqfield2: schema_e_inner__extension__seqfield2_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: schema_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e<'input>(schema_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "schema")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "targetNamespace", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "version", type_: Some(QName(Some("xs"), "token")), default: None }, SmallDef { name: "finalDefault", type_: Some(QName(Some("xs"), "fullDerivationSet")), default: Some("") }, SmallDef { name: "blockDefault", type_: Some(QName(Some("xs"), "blockSet")), default: Some("") }, SmallDef { name: "attributeFormDefault", type_: Some(QName(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "elementFormDefault", type_: Some(QName(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "defaultAttributes", type_: Some(QName(Some("xs"), "QName")), default: None }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: Some("##local") }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }, Ref(QName(Some("xml"), "lang"))], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e<'input>(selector_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "selector")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))), min_occurs: None, max_occurs: None } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: None }], None)), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct sequence_e<'input>(super::UNQUAL::explicitGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "sequence")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq)]
    pub enum simpleContent_e_inner__extension<'input> {
        restriction(Box<super::UNQUAL::simpleRestrictionType<'input>>),
        extension(Box<super::UNQUAL::simpleExtensionType<'input>>),
    }

    impl<'input> Default for simpleContent_e_inner__extension<'input> { fn default() -> simpleContent_e_inner__extension<'input> { simpleContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: simpleContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e<'input>(simpleContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [], Some(Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))), min_occurs: None, max_occurs: None }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType_e<'input>(super::UNQUAL::topLevelSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelSimpleType"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct totalDigits_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "totalDigits")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension<'input> {
        simpleType: union_e_inner__extension__simpleType_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: union_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e<'input>(union_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "union")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "memberTypes", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(List(SimpleList(QName(Some("xs"), "QName")))), min_occurs: None, max_occurs: None } }], Some(Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct unique_e<'input>(super::UNQUAL::keybase<'input>);

    // ^-- from Element { name: Some(QName(None, "unique")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))), min_occurs: None, max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct whiteSpace_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "whiteSpace")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }


    /////////// groups


    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2__openContent_e<'input>(super::UNQUAL::openContent_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2__typeDefParticle_e<'input>(super::UNQUAL::typeDefParticle<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2<'input> {
        seqfield0: complexTypeModel__choicevariant2__openContent_e<'input>,
        seqfield1: complexTypeModel__choicevariant2__typeDefParticle_e<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq)]
    pub enum complexTypeModel<'input> {
        simpleContent(Box<super::UNQUAL::simpleContent_e<'input>>),
        complexContent(Box<super::UNQUAL::complexContent_e<'input>>),
        choicevariant2(Box<complexTypeModel__choicevariant2<'input>>),
    }

    impl<'input> Default for complexTypeModel<'input> { fn default() -> complexTypeModel<'input> { complexTypeModel::choicevariant2(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "simpleContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "complexContent"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))), min_occurs: None, max_occurs: None }])), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__annotation_e<'input>(super::UNQUAL::annotation_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq)]
    pub enum allModel__seqfield1_e_inner<'input> {
        element(Box<super::UNQUAL::localElement<'input>>),
        any(Box<super::UNQUAL::any_e<'input>>),
        group(Box<super::UNQUAL::groupRef<'input>>),
    }

    impl<'input> Default for allModel__seqfield1_e_inner<'input> { fn default() -> allModel__seqfield1_e_inner<'input> { allModel__seqfield1_e_inner::group(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel__seqfield1_e<'input>(allModel__seqfield1_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel<'input> {
        seqfield0: allModel__annotation_e<'input>,
        seqfield1: allModel__seqfield1_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq)]
    pub enum identityConstraint<'input> {
        unique(Box<super::UNQUAL::unique_e<'input>>),
        key(Box<super::UNQUAL::key_e<'input>>),
        keyref(Box<super::UNQUAL::keyref_e<'input>>),
    }

    impl<'input> Default for identityConstraint<'input> { fn default() -> identityConstraint<'input> { identityConstraint::keyref(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "unique"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "key"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "keyref"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq)]
    pub enum attrDecls__seqfield0_e_inner<'input> {
        attribute(Box<super::UNQUAL::attribute<'input>>),
        attributeGroup(Box<super::UNQUAL::attributeGroupRef<'input>>),
    }

    impl<'input> Default for attrDecls__seqfield0_e_inner<'input> { fn default() -> attrDecls__seqfield0_e_inner<'input> { attrDecls__seqfield0_e_inner::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__seqfield0_e<'input>(attrDecls__seqfield0_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls__anyAttribute_e<'input>(super::UNQUAL::anyAttribute_e<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "anyAttribute"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls<'input> {
        seqfield0: attrDecls__seqfield0_e<'input>,
        seqfield1: attrDecls__anyAttribute_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "anyAttribute"))), min_occurs: Some(0), max_occurs: None }])

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

    #[derive(Debug, PartialEq)]
    pub enum typeDefParticle<'input> {
        group(Box<super::UNQUAL::groupRef<'input>>),
        all(Box<super::UNQUAL::all_e<'input>>),
        choice(Box<super::UNQUAL::choice_e<'input>>),
        sequence(Box<super::UNQUAL::sequence_e<'input>>),
    }

    impl<'input> Default for typeDefParticle<'input> { fn default() -> typeDefParticle<'input> { typeDefParticle::sequence(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "all"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq)]
    pub enum schemaTop<'input> {
        redefinable(Box<super::UNQUAL::redefinable<'input>>),
        element(Box<super::UNQUAL::element_e<'input>>),
        attribute(Box<super::UNQUAL::attribute_e<'input>>),
        notation(Box<super::UNQUAL::notation_e<'input>>),
    }

    impl<'input> Default for schemaTop<'input> { fn default() -> schemaTop<'input> { schemaTop::notation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "element"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "attribute"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "notation"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct defRef<'input>(PhantomData<&'input ()>);

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

    #[derive(Debug, PartialEq)]
    pub enum redefinable<'input> {
        simpleType(Box<super::UNQUAL::simpleType_e<'input>>),
        complexType(Box<super::UNQUAL::complexType_e<'input>>),
        group(Box<super::UNQUAL::group_e<'input>>),
        attributeGroup(Box<super::UNQUAL::attributeGroup_e<'input>>),
    }

    impl<'input> Default for redefinable<'input> { fn default() -> redefinable<'input> { redefinable::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "simpleType"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "complexType"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "group"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "attributeGroup"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct occurs<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum simpleDerivation<'input> {
        restriction(Box<super::UNQUAL::restriction_e<'input>>),
        list(Box<super::UNQUAL::list_e<'input>>),
        union(Box<super::UNQUAL::union_e<'input>>),
    }

    impl<'input> Default for simpleDerivation<'input> { fn default() -> simpleDerivation<'input> { simpleDerivation::union(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "restriction"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "list"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "union"))), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttrGroup<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__simpleType_e<'input>(super::UNQUAL::localSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }

    #[derive(Debug, PartialEq)]
    pub enum simpleRestrictionModel__seqfield1_e_inner<'input> {
        facet(Box<super::UNQUAL::facet_e<'input>>),
        choicevariant1(Box<SUPPORT_ANY<'input>>),
    }

    impl<'input> Default for simpleRestrictionModel__seqfield1_e_inner<'input> { fn default() -> simpleRestrictionModel__seqfield1_e_inner<'input> { simpleRestrictionModel__seqfield1_e_inner::choicevariant1(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel__seqfield1_e<'input>(simpleRestrictionModel__seqfield1_e_inner<'input>);

    // ^-- from Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel<'input> {
        simpleType: simpleRestrictionModel__simpleType_e<'input>,
        seqfield1: simpleRestrictionModel__seqfield1_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))), min_occurs: Some(0), max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))), min_occurs: None, max_occurs: None }, Element { name: None, attrs: [], mixed: false, type_: Some(Any), min_occurs: None, max_occurs: None }])), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions__assert_e<'input>(super::UNQUAL::assertion<'input>);

    // ^-- from Element { name: Some(QName(None, "assert")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions<'input> {
        assert: assertions__assert_e<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "assert")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))), min_occurs: Some(0), max_occurs: Some(18446744073709551615) }])

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
}
