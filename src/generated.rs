pub mod UNQUAL {
    use support::*;


    /////////// types


    #[derive(Debug, PartialEq, Default)]
    pub struct all(super::UNQUAL::explicitGroup);

    #[derive(Debug, PartialEq, Default)]
    pub struct allNNI__item0(super::UNQUAL::NMTOKEN);

    #[derive(Debug, PartialEq, Default)]
    pub struct allNNI {
        member0: super::UNQUAL::nonNegativeInteger,
        item0: allNNI__item0,
    }

    #[derive(Debug, PartialEq)]
    pub enum altType__extension {
        simpleType(Box<super::UNQUAL::localSimpleType>),
        complexType(Box<super::UNQUAL::localComplexType>),
    }

    impl Default for altType__extension { fn default() -> altType__extension { altType__extension::complexType(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }, Element { name: Some(Id(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localComplexType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct altType {
        BASE: super::UNQUAL::annotated,
        EXTENSION: altType__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated__extension {
        annotation: super::UNQUAL::annotation_e,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct annotated {
        BASE: super::UNQUAL::openAttrs,
        EXTENSION: annotated__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyType {
        seqfield0: Vec<u8>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute__extension {
        simpleType: super::UNQUAL::localSimpleType,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute {
        BASE: super::UNQUAL::annotated,
        EXTENSION: attribute__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup {
        BASE: super::UNQUAL::annotated,
        EXTENSION: super::UNQUAL::attrDecls,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroupRef(super::UNQUAL::attributeGroup);

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList__valuetype__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList__valuetype {
        member0: super::UNQUAL::anyURI,
        item0: basicNamespaceList__valuetype__item0,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct basicNamespaceList(Vec<basicNamespaceList__valuetype>);

    // ^-- from ComplexList(false, Union(Some([Id(Some("xs"), "anyURI")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "token"))) }])))

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1__valuetype(super::UNQUAL::derivationControl);

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet__item1(Vec<blockSet__item1__valuetype>);

    // ^-- from ComplexList(false, Custom(Id(Some("xs"), "derivationControl")))

    #[derive(Debug, PartialEq, Default)]
    pub struct blockSet {
        item0: blockSet__item0,
        item1: blockSet__item1,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexRestrictionType(super::UNQUAL::restrictionType);

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType {
        BASE: super::UNQUAL::annotated,
        EXTENSION: super::UNQUAL::complexTypeModel,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationControl(super::UNQUAL::NMTOKEN);

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet__item1(Vec<super::UNQUAL::reducedDerivationControl>);

    #[derive(Debug, PartialEq, Default)]
    pub struct derivationSet {
        item0: derivationSet__item0,
        item1: derivationSet__item1,
    }

    #[derive(Debug, PartialEq)]
    pub enum element__extension__seqfield0 {
        simpleType(Box<super::UNQUAL::localSimpleType>),
        complexType(Box<super::UNQUAL::localComplexType>),
    }

    impl Default for element__extension__seqfield0 { fn default() -> element__extension__seqfield0 { element__extension__seqfield0::complexType(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }, Element { name: Some(Id(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localComplexType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct element__extension {
        seqfield0: element__extension__seqfield0,
        alternative: super::UNQUAL::altType,
        identityConstraint: super::UNQUAL::identityConstraint,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }, Element { name: Some(Id(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localComplexType"))) }])) }, Element { name: Some(Id(None, "alternative")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "altType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "identityConstraint"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct element {
        BASE: super::UNQUAL::annotated,
        EXTENSION: element__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitGroup(super::UNQUAL::group);

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType__extension {
        openContent: super::UNQUAL::openContent_e,
        typeDefParticle: super::UNQUAL::typeDefParticle,
        attrDecls: super::UNQUAL::attrDecls,
        assertions: super::UNQUAL::assertions,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "typeDefParticle"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "assertions"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct extensionType {
        BASE: super::UNQUAL::annotated,
        EXTENSION: extensionType__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct formChoice(super::UNQUAL::NMTOKEN);

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet__item1(Vec<super::UNQUAL::typeDerivationControl>);

    #[derive(Debug, PartialEq, Default)]
    pub struct fullDerivationSet {
        item0: fullDerivationSet__item0,
        item1: fullDerivationSet__item1,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct group {
        BASE: super::UNQUAL::annotated,
        EXTENSION: super::UNQUAL::particle,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct groupRef(super::UNQUAL::realGroup);

    #[derive(Debug, PartialEq, Default)]
    pub struct intFacet(super::UNQUAL::facet);

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase__extension {
        selector: super::UNQUAL::selector_e,
        field: super::UNQUAL::field_e,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "selector"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "field"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct keybase {
        BASE: super::UNQUAL::annotated,
        EXTENSION: keybase__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct localComplexType(super::UNQUAL::complexType);

    #[derive(Debug, PartialEq, Default)]
    pub struct localElement(super::UNQUAL::element);

    #[derive(Debug, PartialEq, Default)]
    pub struct localSimpleType(super::UNQUAL::simpleType);

    #[derive(Debug, PartialEq, Default)]
    pub struct namedAttributeGroup(super::UNQUAL::attributeGroup);

    #[derive(Debug, PartialEq, Default)]
    pub struct namedGroup(super::UNQUAL::realGroup);

    #[derive(Debug, PartialEq, Default)]
    pub struct namespaceList {
        member0: super::UNQUAL::specialNamespaceList,
        member1: super::UNQUAL::basicNamespaceList,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct noFixedFacet(super::UNQUAL::facet);

    #[derive(Debug, PartialEq, Default)]
    pub struct numFacet(super::UNQUAL::facet);

    #[derive(Debug, PartialEq, Default)]
    pub struct openAttrs(super::UNQUAL::anyType);

    #[derive(Debug, PartialEq, Default)]
    pub struct public(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList__valuetype__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList__valuetype {
        member0: super::UNQUAL::QName,
        item0: qnameList__valuetype__item0,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameList(Vec<qnameList__valuetype>);

    // ^-- from ComplexList(false, Union(Some([Id(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "token"))) }])))

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA__valuetype__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA__valuetype {
        member0: super::UNQUAL::QName,
        item0: qnameListA__valuetype__item0,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct qnameListA(Vec<qnameListA__valuetype>);

    // ^-- from ComplexList(false, Union(Some([Id(Some("xs"), "QName")]), Some([Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "token"))) }])))

    #[derive(Debug, PartialEq, Default)]
    pub struct realGroup(super::UNQUAL::group);

    #[derive(Debug, PartialEq, Default)]
    pub struct reducedDerivationControl(super::UNQUAL::derivationControl);

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension__seqfield0__choicevariant0 {
        openContent: super::UNQUAL::openContent_e,
        typeDefParticle: super::UNQUAL::typeDefParticle,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "typeDefParticle"))) }])

    #[derive(Debug, PartialEq)]
    pub enum restrictionType__extension__seqfield0 {
        choicevariant0(Box<restrictionType__extension__seqfield0__choicevariant0>),
        simpleRestrictionModel(Box<super::UNQUAL::simpleRestrictionModel>),
    }

    impl Default for restrictionType__extension__seqfield0 { fn default() -> restrictionType__extension__seqfield0 { restrictionType__extension__seqfield0::simpleRestrictionModel(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "typeDefParticle"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "simpleRestrictionModel"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType__extension {
        seqfield0: restrictionType__extension__seqfield0,
        attrDecls: super::UNQUAL::attrDecls,
        assertions: super::UNQUAL::assertions,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "typeDefParticle"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "simpleRestrictionModel"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "assertions"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct restrictionType {
        BASE: super::UNQUAL::annotated,
        EXTENSION: restrictionType__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item1__valuetype(super::UNQUAL::derivationControl);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet__item1(Vec<simpleDerivationSet__item1__valuetype>);

    // ^-- from ComplexList(false, Custom(Id(Some("xs"), "derivationControl")))

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleDerivationSet {
        item0: simpleDerivationSet__item0,
        item1: simpleDerivationSet__item1,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleExplicitGroup(super::UNQUAL::explicitGroup);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleExtensionType(super::UNQUAL::extensionType);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionType(super::UNQUAL::restrictionType);

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType {
        BASE: super::UNQUAL::annotated,
        EXTENSION: super::UNQUAL::simpleDerivation,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct specialNamespaceList(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelAttribute(super::UNQUAL::attribute);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelComplexType(super::UNQUAL::complexType);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelElement(super::UNQUAL::element);

    #[derive(Debug, PartialEq, Default)]
    pub struct topLevelSimpleType(super::UNQUAL::simpleType);

    #[derive(Debug, PartialEq, Default)]
    pub struct typeDerivationControl(super::UNQUAL::derivationControl);

    #[derive(Debug, PartialEq, Default)]
    pub struct wildcard {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct xpathDefaultNamespace__item0(super::UNQUAL::token);

    #[derive(Debug, PartialEq, Default)]
    pub struct xpathDefaultNamespace {
        member0: super::UNQUAL::anyURI,
        item0: xpathDefaultNamespace__item0,
    }


    /////////// elements


    #[derive(Debug, PartialEq, Default)]
    pub struct all_e(super::UNQUAL::all);

    // ^-- from Element { name: Some(Id(None, "all")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "all"))) }

    #[derive(Debug, PartialEq)]
    pub enum annotation_e_inner__extension {
        appinfo(Box<super::UNQUAL::appinfo_e>),
        documentation(Box<super::UNQUAL::documentation_e>),
    }

    impl Default for annotation_e_inner__extension { fn default() -> annotation_e_inner__extension { annotation_e_inner__extension::documentation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "appinfo"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "documentation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e_inner {
        BASE: super::UNQUAL::openAttrs,
        EXTENSION: annotation_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct annotation_e(annotation_e_inner);

    // ^-- from Element { name: Some(Id(None, "annotation")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "openAttrs"), [SmallDef { name: "id", type_: Some(Id(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "appinfo"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "documentation"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e_inner {
        BASE: super::UNQUAL::wildcard,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct any_e(any_e_inner);

    // ^-- from Element { name: Some(Id(None, "any")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(Id(Some("xs"), "qnameList")), default: None }, GroupRef(Id(Some("xs"), "occurs"))], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e_inner {
        BASE: super::UNQUAL::wildcard,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttribute_e(anyAttribute_e_inner);

    // ^-- from Element { name: Some(Id(None, "anyAttribute")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "wildcard"), [SmallDef { name: "notQName", type_: Some(Id(Some("xs"), "qnameListA")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e_inner {
        seqfield0: Vec<u8>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct appinfo_e(appinfo_e_inner);

    // ^-- from Element { name: Some(Id(None, "appinfo")), attrs: [SmallDef { name: "source", type_: Some(Id(Some("xs"), "anyURI")), default: None }, Any(anyAttribute_e(anyAttribute_e_inner { BASE: wildcard { BASE: annotated { BASE: openAttrs(anyType { seqfield0: [] }), EXTENSION: annotated__extension { annotation: annotation_e(annotation_e_inner { BASE: openAttrs(anyType { seqfield0: [] }), EXTENSION: documentation(documentation_e(documentation_e_inner { seqfield0: [] })) }) } } } }))], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])) }

    #[derive(Debug, PartialEq, Default)]
    pub struct assertion_e(super::UNQUAL::assertion);

    // ^-- from Element { name: Some(Id(None, "assertion")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "assertion"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct attribute_e(super::UNQUAL::topLevelAttribute);

    // ^-- from Element { name: Some(Id(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "topLevelAttribute"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct attributeGroup_e(super::UNQUAL::namedAttributeGroup);

    // ^-- from Element { name: Some(Id(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "namedAttributeGroup"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct choice_e(super::UNQUAL::explicitGroup);

    // ^-- from Element { name: Some(Id(None, "choice")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "explicitGroup"))) }

    #[derive(Debug, PartialEq)]
    pub enum complexContent_e_inner__extension {
        restriction(Box<super::UNQUAL::complexRestrictionType>),
        extension(Box<super::UNQUAL::extensionType>),
    }

    impl Default for complexContent_e_inner__extension { fn default() -> complexContent_e_inner__extension { complexContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "complexRestrictionType"))) }, Element { name: Some(Id(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "extensionType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: complexContent_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexContent_e(complexContent_e_inner);

    // ^-- from Element { name: Some(Id(None, "complexContent")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [LongDef { name: "mixed", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "boolean"))) } }], Some(Choice([Element { name: Some(Id(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "complexRestrictionType"))) }, Element { name: Some(Id(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "extensionType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct complexType_e(super::UNQUAL::topLevelComplexType);

    // ^-- from Element { name: Some(Id(None, "complexType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "topLevelComplexType"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner__extension {
        any: super::UNQUAL::wildcard,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "any")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "wildcard"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: defaultOpenContent_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct defaultOpenContent_e(defaultOpenContent_e_inner);

    // ^-- from Element { name: Some(Id(None, "defaultOpenContent")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [SmallDef { name: "appliesToEmpty", type_: Some(Id(Some("xs"), "boolean")), default: Some("false") }, LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "NMTOKEN"))) } }], Some(Sequence([Element { name: Some(Id(None, "any")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "wildcard"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e_inner {
        seqfield0: Vec<u8>,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct documentation_e(documentation_e_inner);

    // ^-- from Element { name: Some(Id(None, "documentation")), attrs: [SmallDef { name: "source", type_: Some(Id(Some("xs"), "anyURI")), default: None }, Ref(Id(Some("xml"), "lang")), Any(anyAttribute_e(anyAttribute_e_inner { BASE: wildcard { BASE: annotated { BASE: openAttrs(anyType { seqfield0: [] }), EXTENSION: annotated__extension { annotation: annotation_e(annotation_e_inner { BASE: openAttrs(anyType { seqfield0: [] }), EXTENSION: documentation(documentation_e(documentation_e_inner { seqfield0: [] })) }) } } } }))], mixed: true, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])) }

    #[derive(Debug, PartialEq, Default)]
    pub struct element_e(super::UNQUAL::topLevelElement);

    // ^-- from Element { name: Some(Id(None, "element")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "topLevelElement"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct enumeration_e(super::UNQUAL::noFixedFacet);

    // ^-- from Element { name: Some(Id(None, "enumeration")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "noFixedFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct explicitTimezone_e(super::UNQUAL::facet);

    // ^-- from Element { name: Some(Id(None, "explicitTimezone")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct facet_e;

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e_inner {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct field_e(field_e_inner);

    // ^-- from Element { name: Some(Id(None, "field")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "token"))) } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(Id(Some("xs"), "xpathDefaultNamespace")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct fractionDigits_e(super::UNQUAL::numFacet);

    // ^-- from Element { name: Some(Id(None, "fractionDigits")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct group_e(super::UNQUAL::namedGroup);

    // ^-- from Element { name: Some(Id(None, "group")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "namedGroup"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e_inner {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct import_e(import_e_inner);

    // ^-- from Element { name: Some(Id(None, "import")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [SmallDef { name: "namespace", type_: Some(Id(Some("xs"), "anyURI")), default: None }, SmallDef { name: "schemaLocation", type_: Some(Id(Some("xs"), "anyURI")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e_inner {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct include_e(include_e_inner);

    // ^-- from Element { name: Some(Id(None, "include")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [SmallDef { name: "schemaLocation", type_: Some(Id(Some("xs"), "anyURI")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct key_e(super::UNQUAL::keybase);

    // ^-- from Element { name: Some(Id(None, "key")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "keybase"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e_inner {
        BASE: super::UNQUAL::keybase,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct keyref_e(keyref_e_inner);

    // ^-- from Element { name: Some(Id(None, "keyref")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "keybase"), [SmallDef { name: "refer", type_: Some(Id(Some("xs"), "QName")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct length_e(super::UNQUAL::numFacet);

    // ^-- from Element { name: Some(Id(None, "length")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner__extension {
        simpleType: super::UNQUAL::localSimpleType,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: list_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct list_e(list_e_inner);

    // ^-- from Element { name: Some(Id(None, "list")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [SmallDef { name: "itemType", type_: Some(Id(Some("xs"), "QName")), default: None }], Some(Sequence([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxExclusive_e(super::UNQUAL::facet);

    // ^-- from Element { name: Some(Id(None, "maxExclusive")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxInclusive_e(super::UNQUAL::facet);

    // ^-- from Element { name: Some(Id(None, "maxInclusive")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct maxLength_e(super::UNQUAL::numFacet);

    // ^-- from Element { name: Some(Id(None, "maxLength")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct minExclusive_e(super::UNQUAL::facet);

    // ^-- from Element { name: Some(Id(None, "minExclusive")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct minInclusive_e(super::UNQUAL::facet);

    // ^-- from Element { name: Some(Id(None, "minInclusive")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "facet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct minLength_e(super::UNQUAL::numFacet);

    // ^-- from Element { name: Some(Id(None, "minLength")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e_inner {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct notation_e(notation_e_inner);

    // ^-- from Element { name: Some(Id(None, "notation")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [SmallDef { name: "name", type_: Some(Id(Some("xs"), "NCName")), default: None }, SmallDef { name: "public", type_: Some(Id(Some("xs"), "public")), default: None }, SmallDef { name: "system", type_: Some(Id(Some("xs"), "anyURI")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner__extension {
        any: super::UNQUAL::wildcard,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "any")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "wildcard"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: openContent_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct openContent_e(openContent_e_inner);

    // ^-- from Element { name: Some(Id(None, "openContent")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [LongDef { name: "mode", default: Some("interleave"), inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "NMTOKEN"))) } }], Some(Sequence([Element { name: Some(Id(None, "any")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "wildcard"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner__extension {
        annotation: super::UNQUAL::annotation_e,
        schemaTop: super::UNQUAL::schemaTop,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "schemaTop"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e_inner {
        BASE: super::UNQUAL::openAttrs,
        EXTENSION: override_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct override_e(override_e_inner);

    // ^-- from Element { name: Some(Id(None, "override")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(Id(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(Id(Some("xs"), "ID")), default: None }], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "schemaTop"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct pattern_e(super::UNQUAL::noFixedFacet);

    // ^-- from Element { name: Some(Id(None, "pattern")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "noFixedFacet"))) }

    #[derive(Debug, PartialEq)]
    pub enum redefine_e_inner__extension {
        annotation(Box<super::UNQUAL::annotation_e>),
        redefinable(Box<super::UNQUAL::redefinable>),
    }

    impl Default for redefine_e_inner__extension { fn default() -> redefine_e_inner__extension { redefine_e_inner__extension::redefinable(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "redefinable"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e_inner {
        BASE: super::UNQUAL::openAttrs,
        EXTENSION: redefine_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct redefine_e(redefine_e_inner);

    // ^-- from Element { name: Some(Id(None, "redefine")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "openAttrs"), [SmallDef { name: "schemaLocation", type_: Some(Id(Some("xs"), "anyURI")), default: None }, SmallDef { name: "id", type_: Some(Id(Some("xs"), "ID")), default: None }], Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "redefinable"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: super::UNQUAL::simpleRestrictionModel,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct restriction_e(restriction_e_inner);

    // ^-- from Element { name: Some(Id(None, "restriction")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [SmallDef { name: "base", type_: Some(Id(Some("xs"), "QName")), default: None }], Some(GroupRef(Id(Some("xs"), "simpleRestrictionModel"))))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield1 {
        defaultOpenContent: super::UNQUAL::defaultOpenContent_e,
        annotation: super::UNQUAL::annotation_e,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "defaultOpenContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension__seqfield2 {
        schemaTop: super::UNQUAL::schemaTop,
        annotation: super::UNQUAL::annotation_e,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "schemaTop"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner__extension {
        composition: super::UNQUAL::composition,
        seqfield1: schema_e_inner__extension__seqfield1,
        seqfield2: schema_e_inner__extension__seqfield2,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "composition"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "defaultOpenContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "schemaTop"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e_inner {
        BASE: super::UNQUAL::openAttrs,
        EXTENSION: schema_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct schema_e(schema_e_inner);

    // ^-- from Element { name: Some(Id(None, "schema")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "openAttrs"), [SmallDef { name: "targetNamespace", type_: Some(Id(Some("xs"), "anyURI")), default: None }, SmallDef { name: "version", type_: Some(Id(Some("xs"), "token")), default: None }, SmallDef { name: "finalDefault", type_: Some(Id(Some("xs"), "fullDerivationSet")), default: Some("") }, SmallDef { name: "blockDefault", type_: Some(Id(Some("xs"), "blockSet")), default: Some("") }, SmallDef { name: "attributeFormDefault", type_: Some(Id(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "elementFormDefault", type_: Some(Id(Some("xs"), "formChoice")), default: Some("unqualified") }, SmallDef { name: "defaultAttributes", type_: Some(Id(Some("xs"), "QName")), default: None }, SmallDef { name: "xpathDefaultNamespace", type_: Some(Id(Some("xs"), "xpathDefaultNamespace")), default: Some("##local") }, SmallDef { name: "id", type_: Some(Id(Some("xs"), "ID")), default: None }, Ref(Id(Some("xml"), "lang"))], Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "composition"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "defaultOpenContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "schemaTop"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e_inner {
        BASE: super::UNQUAL::annotated,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct selector_e(selector_e_inner);

    // ^-- from Element { name: Some(Id(None, "selector")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [LongDef { name: "xpath", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "token"))) } }, SmallDef { name: "xpathDefaultNamespace", type_: Some(Id(Some("xs"), "xpathDefaultNamespace")), default: None }], None)) }

    #[derive(Debug, PartialEq, Default)]
    pub struct sequence_e(super::UNQUAL::explicitGroup);

    // ^-- from Element { name: Some(Id(None, "sequence")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "explicitGroup"))) }

    #[derive(Debug, PartialEq)]
    pub enum simpleContent_e_inner__extension {
        restriction(Box<super::UNQUAL::simpleRestrictionType>),
        extension(Box<super::UNQUAL::simpleExtensionType>),
    }

    impl Default for simpleContent_e_inner__extension { fn default() -> simpleContent_e_inner__extension { simpleContent_e_inner__extension::extension(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "simpleRestrictionType"))) }, Element { name: Some(Id(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "simpleExtensionType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: simpleContent_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleContent_e(simpleContent_e_inner);

    // ^-- from Element { name: Some(Id(None, "simpleContent")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [], Some(Choice([Element { name: Some(Id(None, "restriction")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "simpleRestrictionType"))) }, Element { name: Some(Id(None, "extension")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "simpleExtensionType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleType_e(super::UNQUAL::topLevelSimpleType);

    // ^-- from Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "topLevelSimpleType"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct totalDigits_e(super::UNQUAL::numFacet);

    // ^-- from Element { name: Some(Id(None, "totalDigits")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "numFacet"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner__extension {
        simpleType: super::UNQUAL::localSimpleType,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e_inner {
        BASE: super::UNQUAL::annotated,
        EXTENSION: union_e_inner__extension,
    }

    #[derive(Debug, PartialEq, Default)]
    pub struct union_e(union_e_inner);

    // ^-- from Element { name: Some(Id(None, "union")), attrs: [], mixed: false, type_: Some(Extension(Id(Some("xs"), "annotated"), [LongDef { name: "memberTypes", default: None, inner: Element { name: None, attrs: [], mixed: false, type_: Some(SimpleList(Id(Some("xs"), "QName"))) } }], Some(Sequence([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }])))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct unique_e(super::UNQUAL::keybase);

    // ^-- from Element { name: Some(Id(None, "unique")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "keybase"))) }

    #[derive(Debug, PartialEq, Default)]
    pub struct whiteSpace_e(super::UNQUAL::facet);

    // ^-- from Element { name: Some(Id(None, "whiteSpace")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "facet"))) }


    /////////// groups


    #[derive(Debug, PartialEq, Default)]
    pub struct complexTypeModel__choicevariant2 {
        openContent: super::UNQUAL::openContent_e,
        typeDefParticle: super::UNQUAL::typeDefParticle,
        attrDecls: super::UNQUAL::attrDecls,
        assertions: super::UNQUAL::assertions,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "typeDefParticle"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "assertions"))) }])

    #[derive(Debug, PartialEq)]
    pub enum complexTypeModel {
        simpleContent(Box<super::UNQUAL::simpleContent_e>),
        complexContent(Box<super::UNQUAL::complexContent_e>),
        choicevariant2(Box<complexTypeModel__choicevariant2>),
    }

    impl Default for complexTypeModel { fn default() -> complexTypeModel { complexTypeModel::choicevariant2(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "simpleContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "complexContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "openContent"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "typeDefParticle"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "attrDecls"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "assertions"))) }])) }])

    #[derive(Debug, PartialEq)]
    pub enum redefinable {
        simpleType(Box<super::UNQUAL::simpleType_e>),
        complexType(Box<super::UNQUAL::complexType_e>),
        group(Box<super::UNQUAL::group_e>),
        attributeGroup(Box<super::UNQUAL::attributeGroup_e>),
    }

    impl Default for redefinable { fn default() -> redefinable { redefinable::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "simpleType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "complexType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "group"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "attributeGroup"))) }])

    #[derive(Debug, PartialEq)]
    pub enum simpleRestrictionModel__seqfield1 {
        facet(Box<super::UNQUAL::facet_e>),
        choicevariant1(Box<Vec<u8>>),
    }

    impl Default for simpleRestrictionModel__seqfield1 { fn default() -> simpleRestrictionModel__seqfield1 { simpleRestrictionModel__seqfield1::choicevariant1(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "facet"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct simpleRestrictionModel {
        simpleType: super::UNQUAL::localSimpleType,
        seqfield1: simpleRestrictionModel__seqfield1,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "simpleType")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localSimpleType"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "facet"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Any) }])) }])

    #[derive(Debug, PartialEq)]
    pub enum allModel__seqfield1 {
        element(Box<super::UNQUAL::localElement>),
        any(Box<super::UNQUAL::any_e>),
        group(Box<super::UNQUAL::groupRef>),
    }

    impl Default for allModel__seqfield1 { fn default() -> allModel__seqfield1 { allModel__seqfield1::group(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "element")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localElement"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "any"))) }, Element { name: Some(Id(None, "group")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "groupRef"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct allModel {
        annotation: super::UNQUAL::annotation_e,
        seqfield1: allModel__seqfield1,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(Id(None, "element")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localElement"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "any"))) }, Element { name: Some(Id(None, "group")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "groupRef"))) }])) }])

    #[derive(Debug, PartialEq)]
    pub enum composition {
        include(Box<super::UNQUAL::include_e>),
        import(Box<super::UNQUAL::import_e>),
        redefine(Box<super::UNQUAL::redefine_e>),
        override_(Box<super::UNQUAL::override_e>),
        annotation(Box<super::UNQUAL::annotation_e>),
    }

    impl Default for composition { fn default() -> composition { composition::annotation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "include"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "import"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "redefine"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "override"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "annotation"))) }])

    #[derive(Debug, PartialEq)]
    pub enum nestedParticle {
        element(Box<super::UNQUAL::localElement>),
        group(Box<super::UNQUAL::groupRef>),
        choice(Box<super::UNQUAL::choice_e>),
        sequence(Box<super::UNQUAL::sequence_e>),
        any(Box<super::UNQUAL::any_e>),
    }

    impl Default for nestedParticle { fn default() -> nestedParticle { nestedParticle::any(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "element")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localElement"))) }, Element { name: Some(Id(None, "group")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "groupRef"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "choice"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "sequence"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "any"))) }])

    #[derive(Debug, PartialEq)]
    pub enum particle {
        element(Box<super::UNQUAL::localElement>),
        group(Box<super::UNQUAL::groupRef>),
        all(Box<super::UNQUAL::all_e>),
        choice(Box<super::UNQUAL::choice_e>),
        sequence(Box<super::UNQUAL::sequence_e>),
        any(Box<super::UNQUAL::any_e>),
    }

    impl Default for particle { fn default() -> particle { particle::any(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "element")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "localElement"))) }, Element { name: Some(Id(None, "group")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "groupRef"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "all"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "choice"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "sequence"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "any"))) }])

    #[derive(Debug, PartialEq)]
    pub enum schemaTop {
        redefinable(Box<super::UNQUAL::redefinable>),
        element(Box<super::UNQUAL::element_e>),
        attribute(Box<super::UNQUAL::attribute_e>),
        notation(Box<super::UNQUAL::notation_e>),
    }

    impl Default for schemaTop { fn default() -> schemaTop { schemaTop::notation(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(GroupRef(Id(Some("xs"), "redefinable"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "element"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "attribute"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "notation"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct occurs;

    #[derive(Debug, PartialEq)]
    pub enum identityConstraint {
        unique(Box<super::UNQUAL::unique_e>),
        key(Box<super::UNQUAL::key_e>),
        keyref(Box<super::UNQUAL::keyref_e>),
    }

    impl Default for identityConstraint { fn default() -> identityConstraint { identityConstraint::keyref(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "unique"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "key"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "keyref"))) }])

    #[derive(Debug, PartialEq)]
    pub enum simpleDerivation {
        restriction(Box<super::UNQUAL::restriction_e>),
        list(Box<super::UNQUAL::list_e>),
        union(Box<super::UNQUAL::union_e>),
    }

    impl Default for simpleDerivation { fn default() -> simpleDerivation { simpleDerivation::union(Default::default()) } }

    // ^-- from Choice([Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "restriction"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "list"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "union"))) }])

    #[derive(Debug, PartialEq)]
    pub enum typeDefParticle {
        group(Box<super::UNQUAL::groupRef>),
        all(Box<super::UNQUAL::all_e>),
        choice(Box<super::UNQUAL::choice_e>),
        sequence(Box<super::UNQUAL::sequence_e>),
    }

    impl Default for typeDefParticle { fn default() -> typeDefParticle { typeDefParticle::sequence(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "group")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "groupRef"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "all"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "choice"))) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "sequence"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct assertions {
        assert: super::UNQUAL::assertion,
    }

    // ^-- from Sequence([Element { name: Some(Id(None, "assert")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "assertion"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct anyAttrGroup;

    #[derive(Debug, PartialEq, Default)]
    pub struct defRef;

    #[derive(Debug, PartialEq)]
    pub enum attrDecls__seqfield0 {
        attribute(Box<super::UNQUAL::attribute>),
        attributeGroup(Box<super::UNQUAL::attributeGroupRef>),
    }

    impl Default for attrDecls__seqfield0 { fn default() -> attrDecls__seqfield0 { attrDecls__seqfield0::attributeGroup(Default::default()) } }

    // ^-- from Choice([Element { name: Some(Id(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "attribute"))) }, Element { name: Some(Id(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "attributeGroupRef"))) }])

    #[derive(Debug, PartialEq, Default)]
    pub struct attrDecls {
        seqfield0: attrDecls__seqfield0,
        anyAttribute: super::UNQUAL::anyAttribute_e,
    }

    // ^-- from Sequence([Element { name: None, attrs: [], mixed: false, type_: Some(Choice([Element { name: Some(Id(None, "attribute")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "attribute"))) }, Element { name: Some(Id(None, "attributeGroup")), attrs: [], mixed: false, type_: Some(Custom(Id(Some("xs"), "attributeGroupRef"))) }])) }, Element { name: None, attrs: [], mixed: false, type_: Some(Ref(Id(Some("xs"), "anyAttribute"))) }])
}
