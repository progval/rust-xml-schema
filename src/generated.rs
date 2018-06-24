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

    // ^-- from Union(Some([QName(Some("xs"), "nonNegativeInteger")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))) }]))

    #[derive(Debug, PartialEq)]
    pub enum altType__extension<'input> {
        simpleType(Box<super::UNQUAL::localSimpleType<'input>>),
        complexType(Box<super::UNQUAL::localComplexType<'input>>),
    }

    impl<'input> Default for altType__extension<'input> { fn default() -> altType__extension<'input> { altType__extension::complexType(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct altType<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: altType__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension<'input> {
        annotation: super::UNQUAL::annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotated__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyType<'input> {
        seqfield0: SUPPORT_ANY<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension<'input> {
        simpleType: super::UNQUAL::localSimpleType<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }])

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

    // ^-- from Union(Some([QName(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList<'input>(Vec<basicNamespaceList__valuetype<'input>>);

    // ^-- from ComplexList(false, Union(Some([QName(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }])))

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item0<'input>(super::UNQUAL::token<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1__valuetype<'input>(super::UNQUAL::derivationControl<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1<'input>(Vec<blockSet__item1__valuetype<'input>>);

    // ^-- from ComplexList(false, Custom(QName(Some("xs"), "derivationControl")))

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet<'input> {
        item0: blockSet__item0<'input>,
        item1: blockSet__item1<'input>,
    }

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(ComplexList(false, Custom(QName(Some("xs"), "derivationControl")))) }]))

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

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(SimpleList(QName(Some("xs"), "reducedDerivationControl"))) }]))

    #[derive(Debug, PartialEq)]
    pub enum element__extension__seqfield0<'input> {
        simpleType(Box<super::UNQUAL::localSimpleType<'input>>),
        complexType(Box<super::UNQUAL::localComplexType<'input>>),
    }

    impl<'input> Default for element__extension__seqfield0<'input> { fn default() -> element__extension__seqfield0<'input> { element__extension__seqfield0::complexType(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension<'input> {
        seqfield0: element__extension__seqfield0<'input>,
        alternative: super::UNQUAL::altType<'input>,
        identityConstraint: super::UNQUAL::identityConstraint<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }, Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localComplexType"))) }])) }, Element { name: Some(QName(None, "alternative")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "altType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "identityConstraint"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct element<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: element__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitGroup<'input>(super::UNQUAL::group<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension<'input> {
        openContent: super::UNQUAL::openContent_e<'input>,
        typeDefParticle: super::UNQUAL::typeDefParticle<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))) }])

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

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(SimpleList(QName(Some("xs"), "typeDerivationControl"))) }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct group<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::particle<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct groupRef<'input>(super::UNQUAL::realGroup<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct intFacet<'input>(super::UNQUAL::facet<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase__extension<'input> {
        selector: super::UNQUAL::selector_e<'input>,
        field: super::UNQUAL::field_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "selector"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "field"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: keybase__extension<'input>,
    }

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

    // ^-- from Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList<'input>(Vec<qnameList__valuetype<'input>>);

    // ^-- from ComplexList(false, Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }])))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA__valuetype__item0<'input>(super::UNQUAL::token<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA__valuetype<'input> {
        member0: super::UNQUAL::QName<'input>,
        item0: qnameListA__valuetype__item0<'input>,
    }

    // ^-- from Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }]))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA<'input>(Vec<qnameListA__valuetype<'input>>);

    // ^-- from ComplexList(false, Union(Some([QName(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }])))

    #[derive(Debug, PartialEq, Default)]
    pub struct realGroup<'input>(super::UNQUAL::group<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct reducedDerivationControl<'input>(super::UNQUAL::derivationControl<'input>);

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0__choicevariant0<'input> {
        openContent: super::UNQUAL::openContent_e<'input>,
        typeDefParticle: super::UNQUAL::typeDefParticle<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))) }])

    #[derive(Debug, PartialEq)]
    pub enum restrictionType__extension__seqfield0<'input> {
        choicevariant0(Box<restrictionType__extension__seqfield0__choicevariant0<'input>>),
        simpleRestrictionModel(Box<super::UNQUAL::simpleRestrictionModel<'input>>),
    }

    impl<'input> Default for restrictionType__extension__seqfield0<'input> { fn default() -> restrictionType__extension__seqfield0<'input> { restrictionType__extension__seqfield0::simpleRestrictionModel(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension<'input> {
        seqfield0: restrictionType__extension__seqfield0<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))) }])

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

    // ^-- from ComplexList(false, Custom(QName(Some("xs"), "derivationControl")))

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet<'input> {
        item0: simpleDerivationSet__item0<'input>,
        item1: simpleDerivationSet__item1<'input>,
    }

    // ^-- from Union(None, Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(ComplexList(false, Custom(QName(Some("xs"), "derivationControl")))) }]))

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

    // ^-- from Union(Some([QName(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) }]))


    /////////// elements


    #[derive(Debug, PartialEq, Default)]
    pub struct all_e<'input>(super::UNQUAL::all<'input>);

    // ^-- from Element { name: Some(QName(None, "all")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "all"))) }

    #[derive(Debug, PartialEq)]
    pub enum annotation_e_inner__extension<'input> {
        appinfo(Box<super::UNQUAL::appinfo_e<'input>>),
        documentation(Box<super::UNQUAL::documentation_e<'input>>),
    }

    impl<'input> Default for annotation_e_inner__extension<'input> { fn default() -> annotation_e_inner__extension<'input> { annotation_e_inner__extension::documentation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "appinfo"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "documentation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: annotation_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e<'input>(annotation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "annotation")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "appinfo"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "documentation"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e<'input>(any_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(QName(Some("xs"), "qnameList")), default: None }, GroupRef(QName(Some("xs"), "occurs"))], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e_inner<'input> {
        BASE: super::UNQUAL::wildcard<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e<'input>(anyAttribute_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "anyAttribute")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(QName(Some("xs"), "qnameListA")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e_inner<'input> {
        seqfield0: SUPPORT_ANY<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e<'input>(appinfo_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "appinfo")), attrs: [SmallDef { name: "source", type_: Some(QName(Some("xs"), "anyURI")), default: None }, Any(anyAttribute_e(anyAttribute_e_inner { BASE: wildcard { BASE: annotated { BASE: openAttrs(anyType { seqfield0: SUPPORT_ANY("") }), EXTENSION: annotated__extension { annotation: annotation_e(annotation_e_inner { BASE: openAttrs(anyType { seqfield0: SUPPORT_ANY("") }), EXTENSION: documentation(documentation_e(documentation_e_inner { seqfield0: SUPPORT_ANY("") })) }) } } } }))], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])) }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion_e<'input>(super::UNQUAL::assertion<'input>);

    // ^-- from Element { name: Some(QName(None, "assertion")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute_e<'input>(super::UNQUAL::topLevelAttribute<'input>);

    // ^-- from Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelAttribute"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup_e<'input>(super::UNQUAL::namedAttributeGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedAttributeGroup"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct choice_e<'input>(super::UNQUAL::explicitGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "choice")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))) }

    #[derive(Debug, PartialEq)]
    pub enum complexContent_e_inner__extension<'input> {
        restriction(Box<super::UNQUAL::complexRestrictionType<'input>>),
        extension(Box<super::UNQUAL::extensionType<'input>>),
    }

    impl<'input> Default for complexContent_e_inner__extension<'input> { fn default() -> complexContent_e_inner__extension<'input> { complexContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))) }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: complexContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e<'input>(complexContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "complexContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "mixed", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "boolean"))) } }], Some(Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "complexRestrictionType"))) }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "extensionType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType_e<'input>(super::UNQUAL::topLevelComplexType<'input>);

    // ^-- from Element { name: Some(QName(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelComplexType"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension<'input> {
        any: super::UNQUAL::wildcard<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: defaultOpenContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e<'input>(defaultOpenContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "defaultOpenContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "appliesToEmpty", type_: Some(QName(Some("xs"), "boolean")), default: Some("false") }, LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))) } }], Some(Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e_inner<'input> {
        seqfield0: SUPPORT_ANY<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e<'input>(documentation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "documentation")), attrs: [SmallDef { name: "source", type_: Some(QName(Some("xs"), "anyURI")), default: None }, Ref(QName(Some("xml"), "lang")), Any(anyAttribute_e(anyAttribute_e_inner { BASE: wildcard { BASE: annotated { BASE: openAttrs(anyType { seqfield0: SUPPORT_ANY("") }), EXTENSION: annotated__extension { annotation: annotation_e(annotation_e_inner { BASE: openAttrs(anyType { seqfield0: SUPPORT_ANY("") }), EXTENSION: documentation(documentation_e(documentation_e_inner { seqfield0: SUPPORT_ANY("") })) }) } } } }))], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])) }

    #[derive(Debug, PartialEq, Default)]
    pub struct element_e<'input>(super::UNQUAL::topLevelElement<'input>);

    // ^-- from Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelElement"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct enumeration_e<'input>(super::UNQUAL::noFixedFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "enumeration")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitTimezone_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "explicitTimezone")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet_e<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e<'input>(field_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "field")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct fractionDigits_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "fractionDigits")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct group_e<'input>(super::UNQUAL::namedGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "namedGroup"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e<'input>(import_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "import")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "namespace", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e<'input>(include_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "include")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct key_e<'input>(super::UNQUAL::keybase<'input>);

    // ^-- from Element { name: Some(QName(None, "key")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e_inner<'input> {
        BASE: super::UNQUAL::keybase<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e<'input>(keyref_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "keyref")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "keybase"), [SmallDef { name: "refer", type_: Some(QName(Some("xs"), "QName")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct length_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "length")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension<'input> {
        simpleType: super::UNQUAL::localSimpleType<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: list_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e<'input>(list_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "list")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "itemType", type_: Some(QName(Some("xs"), "QName")), default: None }], Some(Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxExclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxExclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxInclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxInclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxLength_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "maxLength")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct minExclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "minExclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct minInclusive_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "minInclusive")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct minLength_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "minLength")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e<'input>(notation_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "notation")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "name", type_: Some(QName(Some("xs"), "NCName")), default: None }, SmallDef { name: "public", type_: Some(QName(Some("xs"), "public")), default: None }, SmallDef { name: "system", type_: Some(QName(Some("xs"), "anyURI")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension<'input> {
        any: super::UNQUAL::wildcard<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: openContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e<'input>(openContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "openContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "NMTOKEN"))) } }], Some(Sequence([Element { name: Some(QName(None, "any")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "wildcard"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension<'input> {
        annotation: super::UNQUAL::annotation_e<'input>,
        schemaTop: super::UNQUAL::schemaTop<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: override_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e<'input>(override_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "override")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct pattern_e<'input>(super::UNQUAL::noFixedFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "pattern")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "noFixedFacet"))) }

    #[derive(Debug, PartialEq)]
    pub enum redefine_e_inner__extension<'input> {
        annotation(Box<super::UNQUAL::annotation_e<'input>>),
        redefinable(Box<super::UNQUAL::redefinable<'input>>),
    }

    impl<'input> Default for redefine_e_inner__extension<'input> { fn default() -> redefine_e_inner__extension<'input> { redefine_e_inner__extension::redefinable(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: redefine_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e<'input>(redefine_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "redefine")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: super::UNQUAL::simpleRestrictionModel<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e<'input>(restriction_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [SmallDef { name: "base", type_: Some(QName(Some("xs"), "QName")), default: None }], Some(GroupRef(QName(Some("xs"), "simpleRestrictionModel"))))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1<'input> {
        defaultOpenContent: super::UNQUAL::defaultOpenContent_e<'input>,
        annotation: super::UNQUAL::annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2<'input> {
        schemaTop: super::UNQUAL::schemaTop<'input>,
        annotation: super::UNQUAL::annotation_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension<'input> {
        composition: super::UNQUAL::composition<'input>,
        seqfield1: schema_e_inner__extension__seqfield1<'input>,
        seqfield2: schema_e_inner__extension__seqfield2<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner<'input> {
        BASE: super::UNQUAL::openAttrs<'input>,
        EXTENSION: schema_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e<'input>(schema_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "schema")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "openAttrs"), [SmallDef { name: "targetNamespace", type_: Some(QName(Some("xs"), "anyURI")), default: None }, SmallDef { name: "version", type_: Some(QName(Some("xs"), "token")), default: None }, SmallDef { name: "finalDefault", type_: Some(QName(Some("xs"), "fullDerivationSet")), default: Some("") }, SmallDef { name: "blockDefault", type_: Some(QName(Some("xs"), "blockSet")), default: Some("") }, SmallDef { name: "attributeFormDefault", type_: Some(QName(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "elementFormDefault", type_: Some(QName(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "defaultAttributes", type_: Some(QName(Some("xs"), "QName")), default: None }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: Some("##local") }, SmallDef { name: "id", type_: Some(QName(Some("xs"), "ID")), default: None }, Ref(QName(Some("xml"), "lang"))], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "composition"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "defaultOpenContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "schemaTop"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e<'input>(selector_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "selector")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "token"))) } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(QName(Some("xs"), "xpathDefaultNamespace")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct sequence_e<'input>(super::UNQUAL::explicitGroup<'input>);

    // ^-- from Element { name: Some(QName(None, "sequence")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "explicitGroup"))) }

    #[derive(Debug, PartialEq)]
    pub enum simpleContent_e_inner__extension<'input> {
        restriction(Box<super::UNQUAL::simpleRestrictionType<'input>>),
        extension(Box<super::UNQUAL::simpleExtensionType<'input>>),
    }

    impl<'input> Default for simpleContent_e_inner__extension<'input> { fn default() -> simpleContent_e_inner__extension<'input> { simpleContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))) }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: simpleContent_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e<'input>(simpleContent_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleContent")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [], Some(Choice([Element { name: Some(QName(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleRestrictionType"))) }, Element { name: Some(QName(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "simpleExtensionType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType_e<'input>(super::UNQUAL::topLevelSimpleType<'input>);

    // ^-- from Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "topLevelSimpleType"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct totalDigits_e<'input>(super::UNQUAL::numFacet<'input>);

    // ^-- from Element { name: Some(QName(None, "totalDigits")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension<'input> {
        simpleType: super::UNQUAL::localSimpleType<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner<'input> {
        BASE: super::UNQUAL::annotated<'input>,
        EXTENSION: union_e_inner__extension<'input>,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e<'input>(union_e_inner<'input>);

    // ^-- from Element { name: Some(QName(None, "union")), attrs: [], mixed: false, type_: Some(Extension(QName(Some("xs"), "annotated"), [LongDef { name: "memberTypes", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(SimpleList(QName(Some("xs"), "QName"))) } }], Some(Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct unique_e<'input>(super::UNQUAL::keybase<'input>);

    // ^-- from Element { name: Some(QName(None, "unique")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "keybase"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct whiteSpace_e<'input>(super::UNQUAL::facet<'input>);

    // ^-- from Element { name: Some(QName(None, "whiteSpace")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "facet"))) }


    /////////// groups


    #[derive(Debug, PartialEq)]
    pub enum identityConstraint<'input> {
        unique(Box<super::UNQUAL::unique_e<'input>>),
        key(Box<super::UNQUAL::key_e<'input>>),
        keyref(Box<super::UNQUAL::keyref_e<'input>>),
    }

    impl<'input> Default for identityConstraint<'input> { fn default() -> identityConstraint<'input> { identityConstraint::keyref(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "unique"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "key"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "keyref"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttrGroup<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum simpleRestrictionModel__seqfield1<'input> {
        facet(Box<super::UNQUAL::facet_e<'input>>),
        choicevariant1(Box<SUPPORT_ANY<'input>>),
    }

    impl<'input> Default for simpleRestrictionModel__seqfield1<'input> { fn default() -> simpleRestrictionModel__seqfield1<'input> { simpleRestrictionModel__seqfield1::choicevariant1(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel<'input> {
        simpleType: super::UNQUAL::localSimpleType<'input>,
        seqfield1: simpleRestrictionModel__seqfield1<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localSimpleType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "facet"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct defRef<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum schemaTop<'input> {
        redefinable(Box<super::UNQUAL::redefinable<'input>>),
        element(Box<super::UNQUAL::element_e<'input>>),
        attribute(Box<super::UNQUAL::attribute_e<'input>>),
        notation(Box<super::UNQUAL::notation_e<'input>>),
    }

    impl<'input> Default for schemaTop<'input> { fn default() -> schemaTop<'input> { schemaTop::notation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "redefinable"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "element"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "attribute"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "notation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct occurs<'input>(PhantomData<&'input ()>);

    #[derive(Debug, PartialEq)]
    pub enum attrDecls__seqfield0<'input> {
        attribute(Box<super::UNQUAL::attribute<'input>>),
        attributeGroup(Box<super::UNQUAL::attributeGroupRef<'input>>),
    }

    impl<'input> Default for attrDecls__seqfield0<'input> { fn default() -> attrDecls__seqfield0<'input> { attrDecls__seqfield0::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))) }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls<'input> {
        seqfield0: attrDecls__seqfield0<'input>,
        anyAttribute: super::UNQUAL::anyAttribute_e<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attribute"))) }, Element { name: Some(QName(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "attributeGroupRef"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "anyAttribute"))) }])

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

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))) }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "all"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))) }])

    #[derive(Debug, PartialEq)]
    pub enum composition<'input> {
        include(Box<super::UNQUAL::include_e<'input>>),
        import(Box<super::UNQUAL::import_e<'input>>),
        redefine(Box<super::UNQUAL::redefine_e<'input>>),
        override_(Box<super::UNQUAL::override_e<'input>>),
        annotation(Box<super::UNQUAL::annotation_e<'input>>),
    }

    impl<'input> Default for composition<'input> { fn default() -> composition<'input> { composition::annotation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "include"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "import"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "redefine"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "override"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq)]
    pub enum redefinable<'input> {
        simpleType(Box<super::UNQUAL::simpleType_e<'input>>),
        complexType(Box<super::UNQUAL::complexType_e<'input>>),
        group(Box<super::UNQUAL::group_e<'input>>),
        attributeGroup(Box<super::UNQUAL::attributeGroup_e<'input>>),
    }

    impl<'input> Default for redefinable<'input> { fn default() -> redefinable<'input> { redefinable::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "simpleType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "complexType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "group"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "attributeGroup"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2<'input> {
        openContent: super::UNQUAL::openContent_e<'input>,
        typeDefParticle: super::UNQUAL::typeDefParticle<'input>,
        attrDecls: super::UNQUAL::attrDecls<'input>,
        assertions: super::UNQUAL::assertions<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))) }])

    #[derive(Debug, PartialEq)]
    pub enum complexTypeModel<'input> {
        simpleContent(Box<super::UNQUAL::simpleContent_e<'input>>),
        complexContent(Box<super::UNQUAL::complexContent_e<'input>>),
        choicevariant2(Box<complexTypeModel__choicevariant2<'input>>),
    }

    impl<'input> Default for complexTypeModel<'input> { fn default() -> complexTypeModel<'input> { complexTypeModel::choicevariant2(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "simpleContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "complexContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "typeDefParticle"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(QName(Some("xs"), "assertions"))) }])) }])

    #[derive(Debug, PartialEq)]
    pub enum typeDefParticle<'input> {
        group(Box<super::UNQUAL::groupRef<'input>>),
        all(Box<super::UNQUAL::all_e<'input>>),
        choice(Box<super::UNQUAL::choice_e<'input>>),
        sequence(Box<super::UNQUAL::sequence_e<'input>>),
    }

    impl<'input> Default for typeDefParticle<'input> { fn default() -> typeDefParticle<'input> { typeDefParticle::sequence(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "all"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions<'input> {
        assert: super::UNQUAL::assertion<'input>,
    }

    // ^-- from Sequence([Element { name: Some(QName(None, "assert")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "assertion"))) }])

    #[derive(Debug, PartialEq)]
    pub enum nestedParticle<'input> {
        element(Box<super::UNQUAL::localElement<'input>>),
        group(Box<super::UNQUAL::groupRef<'input>>),
        choice(Box<super::UNQUAL::choice_e<'input>>),
        sequence(Box<super::UNQUAL::sequence_e<'input>>),
        any(Box<super::UNQUAL::any_e<'input>>),
    }

    impl<'input> Default for nestedParticle<'input> { fn default() -> nestedParticle<'input> { nestedParticle::any(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))) }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "choice"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "sequence"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))) }])

    #[derive(Debug, PartialEq)]
    pub enum allModel__seqfield1<'input> {
        element(Box<super::UNQUAL::localElement<'input>>),
        any(Box<super::UNQUAL::any_e<'input>>),
        group(Box<super::UNQUAL::groupRef<'input>>),
    }

    impl<'input> Default for allModel__seqfield1<'input> { fn default() -> allModel__seqfield1<'input> { allModel__seqfield1::group(Default::default()) } }

    // ^-- from Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))) }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel<'input> {
        annotation: super::UNQUAL::annotation_e<'input>,
        seqfield1: allModel__seqfield1<'input>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(QName(None, "element")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "localElement"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "any"))) }, Element { name: Some(QName(None, "group")), attrs: [], mixed: false, type_: Some(Custom(QName(Some("xs"), "groupRef"))) }])) }])

    #[derive(Debug, PartialEq)]
    pub enum simpleDerivation<'input> {
        restriction(Box<super::UNQUAL::restriction_e<'input>>),
        list(Box<super::UNQUAL::list_e<'input>>),
        union(Box<super::UNQUAL::union_e<'input>>),
    }

    impl<'input> Default for simpleDerivation<'input> { fn default() -> simpleDerivation<'input> { simpleDerivation::union(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "restriction"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "list"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(QName(Some("xs"), "union"))) }])
}
