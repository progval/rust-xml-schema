// Input: "xml-schema/derived.nxsd"
// Input: "xml-schema/XMLSchema.xsd"
#[allow(unused_imports)]
use support;
pub use std::collections::HashMap;

pub use std::marker::PhantomData;

pub mod unqualified {
    //! None

    #[allow(unused_imports)]
    use super::*;
}

pub mod xs {
    //! Some("http://www.w3.org/2001/XMLSchema")

    #[allow(unused_imports)]
    use super::*;

    pub type Entities<'input> = restrictions::RestrictAnySimpleType<'input>;

    pub type Entity<'input> = restrictions::RestrictNcName<'input>;

    pub type Id<'input> = restrictions::RestrictNcName<'input>;

    pub type Idref<'input> = restrictions::RestrictNcName<'input>;

    pub type Idrefs<'input> = restrictions::RestrictAnySimpleType<'input>;

    pub type Nmtoken<'input> = restrictions::RestrictToken2<'input>;

    pub type Nmtokens<'input> = restrictions::RestrictAnySimpleType<'input>;

    pub type Name<'input> = restrictions::RestrictToken3<'input>;

    pub type Byte<'input> = restrictions::RestrictShort<'input>;

    pub type DateTimeStamp<'input> = restrictions::RestrictDateTime<'input>;

    pub type DayTimeDuration<'input> = restrictions::RestrictDuration2<'input>;

    pub type Int<'input> = restrictions::RestrictLong<'input>;

    pub type Integer<'input> = restrictions::RestrictDecimal<'input>;

    pub type Language<'input> = restrictions::RestrictToken<'input>;

    pub type Long<'input> = restrictions::RestrictInteger2<'input>;

    pub type NegativeInteger<'input> = restrictions::RestrictNonPositiveInteger<'input>;

    pub type NonPositiveInteger<'input> = restrictions::RestrictInteger<'input>;

    pub type NormalizedString<'input> = restrictions::RestrictString<'input>;

    pub type Short<'input> = restrictions::RestrictInt<'input>;

    pub type UnsignedByte<'input> = restrictions::RestrictUnsignedShort<'input>;

    pub type UnsignedInt<'input> = restrictions::RestrictUnsignedLong<'input>;

    pub type UnsignedLong<'input> = restrictions::RestrictNonNegativeInteger<'input>;

    pub type UnsignedShort<'input> = restrictions::RestrictUnsignedInt<'input>;

    pub type YearMonthDuration<'input> = restrictions::RestrictDuration<'input>;

    pub type AllNni<'input> = unions::UnionNonNegativeIntegerNmtoken<'input>;

    pub type BasicNamespaceList<'input> = lists::UnionAnyUriTokenList<'input>;

    pub type BlockSet<'input> = unions::UnionTokenDerivationControlList<'input>;

    pub type DerivationControl<'input> = restrictions::RestrictNmtoken8<'input>;

    pub type DerivationSet<'input> = unions::UnionTokenReducedDerivationControlList<'input>;

    pub type FormChoice<'input> = restrictions::RestrictNmtoken6<'input>;

    pub type FullDerivationSet<'input> = unions::UnionTokenTypeDerivationControlList<'input>;

    pub type NamespaceList<'input> = unions::UnionSpecialNamespaceListBasicNamespaceList<'input>;

    pub type Public<'input> = restrictions::RestrictToken4<'input>;

    pub type QnameList<'input> = lists::UnionQNameTokenList<'input>;

    pub type QnameListA<'input> = lists::UnionQNameTokenList<'input>;

    pub type ReducedDerivationControl<'input> = restrictions::RestrictDerivationControl<'input>;

    pub type SimpleDerivationSet<'input> = unions::UnionTokenDerivationControlList<'input>;

    pub type SpecialNamespaceList<'input> = restrictions::RestrictToken5<'input>;

    pub type TypeDerivationControl<'input> = restrictions::RestrictDerivationControl2<'input>;

    pub type XpathDefaultNamespace<'input> = unions::UnionAnyUriToken<'input>;

    ///  Only elements allowed inside
    #[derive(Debug, PartialEq)]
    pub struct All<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_min_occurs: Option<restrictions::RestrictNonNegativeInteger3<'input>>,
        pub attr_max_occurs: Option<restrictions::RestrictAllNni<'input>>,
        pub all_model: super::xs::AllModel<'input>,
    }

    impl_element!(All, "http://www.w3.org/2001/XMLSchema", "all", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "minOccurs") => attr_min_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "maxOccurs") => attr_max_occurs: optional,
    }, fields = {
        (all_model, xs, AllModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct Annotation<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation_content: Vec<super::enums::AnnotationContent<'input>>,
    }

    impl_element!(Annotation, "http://www.w3.org/2001/XMLSchema", "annotation", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation_content, enums, Vec<AnnotationContent; min=0;>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Any<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_not_q_name: Option<xs::QnameList<'input>>,
        pub attr_min_occurs: Option<support::NonNegativeInteger<'input>>,
        pub attr_max_occurs: Option<xs::AllNni<'input>>,
        pub attr_namespace: Option<xs::NamespaceList<'input>>,
        pub attr_not_namespace: Option<restrictions::RestrictBasicNamespaceList<'input>>,
        pub attr_process_contents: Option<restrictions::RestrictNmtoken7<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Any, "http://www.w3.org/2001/XMLSchema", "any", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "notQName") => attr_not_q_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "minOccurs") => attr_min_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "maxOccurs") => attr_max_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "namespace") => attr_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "notNamespace") => attr_not_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "processContents") => attr_process_contents: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct AnyAttribute<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_not_q_name: Option<xs::QnameListA<'input>>,
        pub attr_namespace: Option<xs::NamespaceList<'input>>,
        pub attr_not_namespace: Option<restrictions::RestrictBasicNamespaceList<'input>>,
        pub attr_process_contents: Option<restrictions::RestrictNmtoken7<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AnyAttribute, "http://www.w3.org/2001/XMLSchema", "anyAttribute", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "notQName") => attr_not_q_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "namespace") => attr_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "notNamespace") => attr_not_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "processContents") => attr_process_contents: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Appinfo<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_source: Option<support::AnyUri<'input>>,
        pub sequence_any: Vec<super::sequences::SequenceAny<'input>>,
    }

    impl_element!(Appinfo, "http://www.w3.org/2001/XMLSchema", "appinfo", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "source") => attr_source: optional,
    }, fields = {
        (sequence_any, sequences, Vec<SequenceAny; min=0;>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Assertion<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_test: Option<support::XmlString<'input>>,
        pub attr_xpath_default_namespace: Option<xs::XpathDefaultNamespace<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Assertion, "http://www.w3.org/2001/XMLSchema", "assertion", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "test") => attr_test: optional,
        ("http://www.w3.org/2001/XMLSchema", "xpathDefaultNamespace") => attr_xpath_default_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Attribute<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub attr_inheritable: Option<support::Boolean<'input>>,
        pub attr_type: Option<support::QName<'input>>,
        pub attr_default: Option<support::XmlString<'input>>,
        pub attr_fixed: Option<support::XmlString<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub local_simple_type: Option<super::inline_elements::LocalSimpleType<'input>>,
    }

    impl_element!(Attribute, "http://www.w3.org/2001/XMLSchema", "attribute", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
        ("http://www.w3.org/2001/XMLSchema", "inheritable") => attr_inheritable: optional,
        ("http://www.w3.org/2001/XMLSchema", "type") => attr_type: optional,
        ("http://www.w3.org/2001/XMLSchema", "default") => attr_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (local_simple_type, inline_elements, Option<LocalSimpleType>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AttributeGroup<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
    }

    impl_element!(AttributeGroup, "http://www.w3.org/2001/XMLSchema", "attributeGroup", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (attr_decls, xs, AttrDecls),
    });

    ///  group type for the three kinds of group
    #[derive(Debug, PartialEq)]
    pub struct Choice<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_min_occurs: Option<support::NonNegativeInteger<'input>>,
        pub attr_max_occurs: Option<xs::AllNni<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(Choice, "http://www.w3.org/2001/XMLSchema", "choice", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "minOccurs") => attr_min_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "maxOccurs") => attr_max_occurs: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ComplexContent<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_mixed: Option<support::Boolean<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_restriction_extension: super::enums::ChoiceRestrictionExtension<'input>,
    }

    impl_element!(ComplexContent, "http://www.w3.org/2001/XMLSchema", "complexContent", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "mixed") => attr_mixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (choice_restriction_extension, enums, ChoiceRestrictionExtension),
    });

    #[derive(Debug, PartialEq)]
    pub struct ComplexType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub attr_mixed: Option<support::Boolean<'input>>,
        pub attr_abstract: Option<support::Boolean<'input>>,
        pub attr_final: Option<xs::DerivationSet<'input>>,
        pub attr_block: Option<xs::DerivationSet<'input>>,
        pub attr_default_attributes_apply: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub complex_type_model: super::xs::ComplexTypeModel<'input>,
    }

    impl_element!(ComplexType, "http://www.w3.org/2001/XMLSchema", "complexType", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
        ("http://www.w3.org/2001/XMLSchema", "mixed") => attr_mixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "abstract") => attr_abstract: optional,
        ("http://www.w3.org/2001/XMLSchema", "final") => attr_final: optional,
        ("http://www.w3.org/2001/XMLSchema", "block") => attr_block: optional,
        ("http://www.w3.org/2001/XMLSchema", "defaultAttributesApply") => attr_default_attributes_apply: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (complex_type_model, xs, ComplexTypeModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct DefaultOpenContent<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_applies_to_empty: Option<support::Boolean<'input>>,
        pub attr_mode: Option<restrictions::RestrictNmtoken<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub any_wildcard: super::inline_elements::AnyWildcard<'input>,
    }

    impl_element!(DefaultOpenContent, "http://www.w3.org/2001/XMLSchema", "defaultOpenContent", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "appliesToEmpty") => attr_applies_to_empty: optional,
        ("http://www.w3.org/2001/XMLSchema", "mode") => attr_mode: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (any_wildcard, inline_elements, AnyWildcard),
    });

    #[derive(Debug, PartialEq)]
    pub struct Documentation<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_source: Option<support::AnyUri<'input>>,
        pub sequence_any: Vec<super::sequences::SequenceAny<'input>>,
    }

    impl_element!(Documentation, "http://www.w3.org/2001/XMLSchema", "documentation", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "source") => attr_source: optional,
    }, fields = {
        (sequence_any, sequences, Vec<SequenceAny; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Element<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub attr_type: Option<support::QName<'input>>,
        pub attr_substitution_group: Option<lists::QNameList<'input>>,
        pub attr_default: Option<support::XmlString<'input>>,
        pub attr_fixed: Option<support::XmlString<'input>>,
        pub attr_nillable: Option<support::Boolean<'input>>,
        pub attr_abstract: Option<support::Boolean<'input>>,
        pub attr_final: Option<xs::DerivationSet<'input>>,
        pub attr_block: Option<xs::BlockSet<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub type_: Option<super::enums::Type<'input>>,
        pub alternative_alt_type: Vec<super::inline_elements::AlternativeAltType<'input>>,
        pub identity_constraint: Vec<super::xs::IdentityConstraint<'input>>,
    }

    impl_element!(Element, "http://www.w3.org/2001/XMLSchema", "element", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
        ("http://www.w3.org/2001/XMLSchema", "type") => attr_type: optional,
        ("http://www.w3.org/2001/XMLSchema", "substitutionGroup") => attr_substitution_group: optional,
        ("http://www.w3.org/2001/XMLSchema", "default") => attr_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "nillable") => attr_nillable: optional,
        ("http://www.w3.org/2001/XMLSchema", "abstract") => attr_abstract: optional,
        ("http://www.w3.org/2001/XMLSchema", "final") => attr_final: optional,
        ("http://www.w3.org/2001/XMLSchema", "block") => attr_block: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (type_, enums, Option<Type>),
        (alternative_alt_type, inline_elements, Vec<AlternativeAltType; min=0;>),
        (identity_constraint, xs, Vec<IdentityConstraint; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Enumeration<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::AnySimpleType<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Enumeration, "http://www.w3.org/2001/XMLSchema", "enumeration", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ExplicitTimezone<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: restrictions::RestrictNmtoken3<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(ExplicitTimezone, "http://www.w3.org/2001/XMLSchema", "explicitTimezone", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub enum Facet<'input> {
        FacetHead(Box<super::xs::FacetHead<'input>>),
        MinExclusive(Box<super::xs::MinExclusive<'input>>),
        MinInclusive(Box<super::xs::MinInclusive<'input>>),
        MaxExclusive(Box<super::xs::MaxExclusive<'input>>),
        MaxInclusive(Box<super::xs::MaxInclusive<'input>>),
        TotalDigits(Box<super::xs::TotalDigits<'input>>),
        FractionDigits(Box<super::xs::FractionDigits<'input>>),
        Length(Box<super::xs::Length<'input>>),
        MinLength(Box<super::xs::MinLength<'input>>),
        MaxLength(Box<super::xs::MaxLength<'input>>),
        Enumeration(Box<super::xs::Enumeration<'input>>),
        WhiteSpace(Box<super::xs::WhiteSpace<'input>>),
        Pattern(Box<super::xs::Pattern<'input>>),
        Assertion(Box<super::xs::Assertion<'input>>),
        ExplicitTimezone(Box<super::xs::ExplicitTimezone<'input>>),
    }

    impl_enum!(Facet,
        impl_singleton_variant!(FacetHead, xs, Box<FacetHead>),
        impl_singleton_variant!(MinExclusive, xs, Box<MinExclusive>),
        impl_singleton_variant!(MinInclusive, xs, Box<MinInclusive>),
        impl_singleton_variant!(MaxExclusive, xs, Box<MaxExclusive>),
        impl_singleton_variant!(MaxInclusive, xs, Box<MaxInclusive>),
        impl_singleton_variant!(TotalDigits, xs, Box<TotalDigits>),
        impl_singleton_variant!(FractionDigits, xs, Box<FractionDigits>),
        impl_singleton_variant!(Length, xs, Box<Length>),
        impl_singleton_variant!(MinLength, xs, Box<MinLength>),
        impl_singleton_variant!(MaxLength, xs, Box<MaxLength>),
        impl_singleton_variant!(Enumeration, xs, Box<Enumeration>),
        impl_singleton_variant!(WhiteSpace, xs, Box<WhiteSpace>),
        impl_singleton_variant!(Pattern, xs, Box<Pattern>),
        impl_singleton_variant!(Assertion, xs, Box<Assertion>),
        impl_singleton_variant!(ExplicitTimezone, xs, Box<ExplicitTimezone>),
    );

    ///  An abstract element, representing facets in general. The facets defined by this spec are substitutable for this element, and implementation-defined facets should also name this as a substitution-group head. 
    #[derive(Debug, PartialEq)]
    pub struct FacetHead<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
    }

    impl_element!(FacetHead, "http://www.w3.org/2001/XMLSchema", "facet", attributes = {
    }, fields = {
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Field<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_xpath: restrictions::RestrictToken4<'input>,
        pub attr_xpath_default_namespace: Option<xs::XpathDefaultNamespace<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Field, "http://www.w3.org/2001/XMLSchema", "field", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "xpath") => attr_xpath: required,
        ("http://www.w3.org/2001/XMLSchema", "xpathDefaultNamespace") => attr_xpath_default_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct FractionDigits<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::NonNegativeInteger<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(FractionDigits, "http://www.w3.org/2001/XMLSchema", "fractionDigits", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Group<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_all_choice_sequence: super::enums::ChoiceAllChoiceSequence<'input>,
    }

    impl_element!(Group, "http://www.w3.org/2001/XMLSchema", "group", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (choice_all_choice_sequence, enums, ChoiceAllChoiceSequence),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Import<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_namespace: Option<support::AnyUri<'input>>,
        pub attr_schema_location: Option<support::AnyUri<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Import, "http://www.w3.org/2001/XMLSchema", "import", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "namespace") => attr_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "schemaLocation") => attr_schema_location: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Include<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_schema_location: support::AnyUri<'input>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Include, "http://www.w3.org/2001/XMLSchema", "include", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "schemaLocation") => attr_schema_location: required,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Key<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: Option<support::NcName<'input>>,
        pub attr_ref: Option<support::QName<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub uniqueness_spec: Option<super::sequences::UniquenessSpec<'input>>,
    }

    impl_element!(Key, "http://www.w3.org/2001/XMLSchema", "key", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (uniqueness_spec, sequences, Option<UniquenessSpec>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Keyref<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_refer: Option<support::QName<'input>>,
        pub attr_name: Option<support::NcName<'input>>,
        pub attr_ref: Option<support::QName<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub uniqueness_spec: Option<super::sequences::UniquenessSpec<'input>>,
    }

    impl_element!(Keyref, "http://www.w3.org/2001/XMLSchema", "keyref", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "refer") => attr_refer: optional,
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (uniqueness_spec, sequences, Option<UniquenessSpec>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Length<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::NonNegativeInteger<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Length, "http://www.w3.org/2001/XMLSchema", "length", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  itemType attribute and simpleType child are mutually exclusive, but one or other is required 
    ///  itemType attribute and simpleType child are mutually exclusive, but one or other is required 
    #[derive(Debug, PartialEq)]
    pub struct List<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_item_type: Option<support::QName<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub local_simple_type: Option<super::inline_elements::LocalSimpleType<'input>>,
    }

    impl_element!(List, "http://www.w3.org/2001/XMLSchema", "list", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "itemType") => attr_item_type: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (local_simple_type, inline_elements, Option<LocalSimpleType>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct MaxExclusive<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::AnySimpleType<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MaxExclusive, "http://www.w3.org/2001/XMLSchema", "maxExclusive", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct MaxInclusive<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::AnySimpleType<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MaxInclusive, "http://www.w3.org/2001/XMLSchema", "maxInclusive", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MaxLength<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::NonNegativeInteger<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MaxLength, "http://www.w3.org/2001/XMLSchema", "maxLength", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct MinExclusive<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::AnySimpleType<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MinExclusive, "http://www.w3.org/2001/XMLSchema", "minExclusive", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct MinInclusive<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::AnySimpleType<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MinInclusive, "http://www.w3.org/2001/XMLSchema", "minInclusive", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct MinLength<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::NonNegativeInteger<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(MinLength, "http://www.w3.org/2001/XMLSchema", "minLength", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Notation<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub attr_public: Option<xs::Public<'input>>,
        pub attr_system: Option<support::AnyUri<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Notation, "http://www.w3.org/2001/XMLSchema", "notation", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
        ("http://www.w3.org/2001/XMLSchema", "public") => attr_public: optional,
        ("http://www.w3.org/2001/XMLSchema", "system") => attr_system: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct OpenContent<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_mode: Option<restrictions::RestrictNmtoken2<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub any_wildcard: Option<super::inline_elements::AnyWildcard<'input>>,
    }

    impl_element!(OpenContent, "http://www.w3.org/2001/XMLSchema", "openContent", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "mode") => attr_mode: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (any_wildcard, inline_elements, Option<AnyWildcard>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Override<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_schema_location: support::AnyUri<'input>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub schema_top: Vec<super::xs::SchemaTop<'input>>,
    }

    impl_element!(Override, "http://www.w3.org/2001/XMLSchema", "override", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "schemaLocation") => attr_schema_location: required,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (schema_top, xs, Vec<SchemaTop; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Pattern<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::XmlString<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Pattern, "http://www.w3.org/2001/XMLSchema", "pattern", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Redefine<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_schema_location: support::AnyUri<'input>,
        pub attr_id: Option<xs::Id<'input>>,
        pub choice_annotation_redefinable: Vec<super::enums::ChoiceAnnotationRedefinable<'input>>,
    }

    impl_element!(Redefine, "http://www.w3.org/2001/XMLSchema", "redefine", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "schemaLocation") => attr_schema_location: required,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (choice_annotation_redefinable, enums, Vec<ChoiceAnnotationRedefinable; min=0;>),
    });

    ///  base attribute and simpleType child are mutually exclusive, but one or other is required 
    ///  base attribute and simpleType child are mutually exclusive, but one or other is required 
    #[derive(Debug, PartialEq)]
    pub struct Restriction<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_base: Option<support::QName<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_restriction_model: super::xs::SimpleRestrictionModel<'input>,
    }

    impl_element!(Restriction, "http://www.w3.org/2001/XMLSchema", "restriction", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "base") => attr_base: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (simple_restriction_model, xs, SimpleRestrictionModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct Schema<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_target_namespace: Option<support::AnyUri<'input>>,
        pub attr_version: Option<support::Token<'input>>,
        pub attr_final_default: Option<xs::FullDerivationSet<'input>>,
        pub attr_block_default: Option<xs::BlockSet<'input>>,
        pub attr_attribute_form_default: Option<xs::FormChoice<'input>>,
        pub attr_element_form_default: Option<xs::FormChoice<'input>>,
        pub attr_default_attributes: Option<support::QName<'input>>,
        pub attr_xpath_default_namespace: Option<xs::XpathDefaultNamespace<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub composition: Vec<super::xs::Composition<'input>>,
        pub open_content: Option<super::sequences::AnnotatedOpenContent<'input>>,
        pub sequence_schema_top_annotation: Vec<super::sequences::SequenceSchemaTopAnnotation<'input>>,
    }

    impl_element!(Schema, "http://www.w3.org/2001/XMLSchema", "schema", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "targetNamespace") => attr_target_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "version") => attr_version: optional,
        ("http://www.w3.org/2001/XMLSchema", "finalDefault") => attr_final_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "blockDefault") => attr_block_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "attributeFormDefault") => attr_attribute_form_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "elementFormDefault") => attr_element_form_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "defaultAttributes") => attr_default_attributes: optional,
        ("http://www.w3.org/2001/XMLSchema", "xpathDefaultNamespace") => attr_xpath_default_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (composition, xs, Vec<Composition; min=0;>),
        (open_content, sequences, Option<AnnotatedOpenContent>),
        (sequence_schema_top_annotation, sequences, Vec<SequenceSchemaTopAnnotation; min=0;>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Selector<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_xpath: restrictions::RestrictToken4<'input>,
        pub attr_xpath_default_namespace: Option<xs::XpathDefaultNamespace<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Selector, "http://www.w3.org/2001/XMLSchema", "selector", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "xpath") => attr_xpath: required,
        ("http://www.w3.org/2001/XMLSchema", "xpathDefaultNamespace") => attr_xpath_default_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  group type for the three kinds of group
    #[derive(Debug, PartialEq)]
    pub struct Sequence<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_min_occurs: Option<support::NonNegativeInteger<'input>>,
        pub attr_max_occurs: Option<xs::AllNni<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(Sequence, "http://www.w3.org/2001/XMLSchema", "sequence", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "minOccurs") => attr_min_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "maxOccurs") => attr_max_occurs: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct SimpleContent<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_restriction_extension: super::enums::ChoiceRestrictionExtension<'input>,
    }

    impl_element!(SimpleContent, "http://www.w3.org/2001/XMLSchema", "simpleContent", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (choice_restriction_extension, enums, ChoiceRestrictionExtension),
    });

    #[derive(Debug, PartialEq)]
    pub struct SimpleType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: support::NcName<'input>,
        pub attr_final: Option<xs::SimpleDerivationSet<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_derivation: super::xs::SimpleDerivation<'input>,
    }

    impl_element!(SimpleType, "http://www.w3.org/2001/XMLSchema", "simpleType", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: required,
        ("http://www.w3.org/2001/XMLSchema", "final") => attr_final: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (simple_derivation, xs, SimpleDerivation),
    });

    #[derive(Debug, PartialEq)]
    pub struct TotalDigits<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: support::PositiveInteger<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(TotalDigits, "http://www.w3.org/2001/XMLSchema", "totalDigits", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  memberTypes attribute must be non-empty or there must be at least one simpleType child 
    ///  memberTypes attribute must be non-empty or there must be at least one simpleType child 
    #[derive(Debug, PartialEq)]
    pub struct Union<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_member_types: Option<lists::QNameList<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub local_simple_type: Vec<super::inline_elements::LocalSimpleType<'input>>,
    }

    impl_element!(Union, "http://www.w3.org/2001/XMLSchema", "union", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "memberTypes") => attr_member_types: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (local_simple_type, inline_elements, Vec<LocalSimpleType; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Unique<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_name: Option<support::NcName<'input>>,
        pub attr_ref: Option<support::QName<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub uniqueness_spec: Option<super::sequences::UniquenessSpec<'input>>,
    }

    impl_element!(Unique, "http://www.w3.org/2001/XMLSchema", "unique", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (uniqueness_spec, sequences, Option<UniquenessSpec>),
    });

    #[derive(Debug, PartialEq)]
    pub struct WhiteSpace<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_value: restrictions::RestrictNmtoken4<'input>,
        pub attr_fixed: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(WhiteSpace, "http://www.w3.org/2001/XMLSchema", "whiteSpace", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "value") => attr_value: required,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AllModel<'input> {
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub choice_element_any_group: Vec<super::enums::ChoiceElementAnyGroup<'input>>,
    }

    impl_group_or_sequence!(AllModel,
        (annotation, xs, Option<Annotation>),
        (choice_element_any_group, enums, Vec<ChoiceElementAnyGroup; min=0;>),
    );

    #[derive(Debug, PartialEq)]
    pub struct Assertions<'input> {
        pub assertion: Vec<super::inline_elements::Assertion<'input>>,
    }

    impl_group_or_sequence!(Assertions,
        (assertion, inline_elements, Vec<Assertion; min=0;>),
    );

    #[derive(Debug, PartialEq)]
    pub struct AttrDecls<'input> {
        pub attribute: Vec<super::enums::AttrOrAttrGroup<'input>>,
        pub any_attribute: Option<super::xs::AnyAttribute<'input>>,
    }

    impl_group_or_sequence!(AttrDecls,
        (attribute, enums, Vec<AttrOrAttrGroup; min=0;>),
        (any_attribute, xs, Option<AnyAttribute>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ComplexTypeModel<'input> {
        SimpleContent(Box<super::xs::SimpleContent<'input>>),
        ComplexContent(Box<super::xs::ComplexContent<'input>>),
        CompleteContentModel {
            open_content: Option<Box<super::xs::OpenContent<'input>> >,
            type_def_particle: Option<Box<super::xs::TypeDefParticle<'input>> >,
            attr_decls: Box<super::xs::AttrDecls<'input>>,
            assertions: Box<super::xs::Assertions<'input>>,
        }
        ,
    }

    impl_enum!(ComplexTypeModel,
        impl_singleton_variant!(SimpleContent, xs, Box<SimpleContent>),
        impl_singleton_variant!(ComplexContent, xs, Box<ComplexContent>),
        impl_struct_variant!(CompleteContentModel,
            (open_content, xs, Option<Box<OpenContent> >),
            (type_def_particle, xs, Option<Box<TypeDefParticle> >),
            (attr_decls, xs, Box<AttrDecls>),
            (assertions, xs, Box<Assertions>),
        ),
    );

    #[derive(Debug, PartialEq)]
    pub enum Composition<'input> {
        Include(Box<super::xs::Include<'input>>),
        Import(Box<super::xs::Import<'input>>),
        Redefine(Box<super::xs::Redefine<'input>>),
        Override(Box<super::xs::Override<'input>>),
        Annotation(Box<super::xs::Annotation<'input>>),
    }

    impl_enum!(Composition,
        impl_singleton_variant!(Include, xs, Box<Include>),
        impl_singleton_variant!(Import, xs, Box<Import>),
        impl_singleton_variant!(Redefine, xs, Box<Redefine>),
        impl_singleton_variant!(Override, xs, Box<Override>),
        impl_singleton_variant!(Annotation, xs, Box<Annotation>),
    );

    /// The three kinds of identity constraints, all with type of or derived from 'keybase'. 
    #[derive(Debug, PartialEq)]
    pub enum IdentityConstraint<'input> {
        Unique(Box<super::xs::Unique<'input>>),
        Key(Box<super::xs::Key<'input>>),
        Keyref(Box<super::xs::Keyref<'input>>),
    }

    impl_enum!(IdentityConstraint,
        impl_singleton_variant!(Unique, xs, Box<Unique>),
        impl_singleton_variant!(Key, xs, Box<Key>),
        impl_singleton_variant!(Keyref, xs, Box<Keyref>),
    );

    #[derive(Debug, PartialEq)]
    pub enum NestedParticle<'input> {
        Element(Box<super::inline_elements::LocalElement<'input>>),
        Group(Box<super::inline_elements::GroupRef<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
        Any(Box<super::xs::Any<'input>>),
    }

    impl_enum!(NestedParticle,
        impl_singleton_variant!(Element, inline_elements, Box<LocalElement>),
        impl_singleton_variant!(Group, inline_elements, Box<GroupRef>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
        impl_singleton_variant!(Any, xs, Box<Any>),
    );

    #[derive(Debug, PartialEq)]
    pub enum Particle<'input> {
        Element(Box<super::inline_elements::LocalElement<'input>>),
        Group(Box<super::inline_elements::GroupRef<'input>>),
        All(Box<super::xs::All<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
        Any(Box<super::xs::Any<'input>>),
    }

    impl_enum!(Particle,
        impl_singleton_variant!(Element, inline_elements, Box<LocalElement>),
        impl_singleton_variant!(Group, inline_elements, Box<GroupRef>),
        impl_singleton_variant!(All, xs, Box<All>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
        impl_singleton_variant!(Any, xs, Box<Any>),
    );

    ///  This group is for the elements which can self-redefine (see <redefine> below).
    #[derive(Debug, PartialEq)]
    pub enum Redefinable<'input> {
        SimpleType(Box<super::xs::SimpleType<'input>>),
        ComplexType(Box<super::xs::ComplexType<'input>>),
        Group(Box<super::xs::Group<'input>>),
        AttributeGroup(Box<super::xs::AttributeGroup<'input>>),
    }

    impl_enum!(Redefinable,
        impl_singleton_variant!(SimpleType, xs, Box<SimpleType>),
        impl_singleton_variant!(ComplexType, xs, Box<ComplexType>),
        impl_singleton_variant!(Group, xs, Box<Group>),
        impl_singleton_variant!(AttributeGroup, xs, Box<AttributeGroup>),
    );

    ///  This group is for the elements which occur freely at the top level of schemas. All of their types are based on the "annotated" type by extension.
    #[derive(Debug, PartialEq)]
    pub enum SchemaTop<'input> {
        Redefinable(Box<super::xs::Redefinable<'input>>),
        Element(Box<super::xs::Element<'input>>),
        Attribute(Box<super::xs::Attribute<'input>>),
        Notation(Box<super::xs::Notation<'input>>),
    }

    impl_enum!(SchemaTop,
        impl_singleton_variant!(Redefinable, xs, Box<Redefinable>),
        impl_singleton_variant!(Element, xs, Box<Element>),
        impl_singleton_variant!(Attribute, xs, Box<Attribute>),
        impl_singleton_variant!(Notation, xs, Box<Notation>),
    );

    #[derive(Debug, PartialEq)]
    pub enum SimpleDerivation<'input> {
        Restriction(Box<super::xs::Restriction<'input>>),
        List(Box<super::xs::List<'input>>),
        Union(Box<super::xs::Union<'input>>),
    }

    impl_enum!(SimpleDerivation,
        impl_singleton_variant!(Restriction, xs, Box<Restriction>),
        impl_singleton_variant!(List, xs, Box<List>),
        impl_singleton_variant!(Union, xs, Box<Union>),
    );

    #[derive(Debug, PartialEq)]
    pub struct SimpleRestrictionModel<'input> {
        pub local_simple_type: Option<super::inline_elements::LocalSimpleType<'input>>,
        pub choice_facet_any: Vec<super::enums::ChoiceFacetAny<'input>>,
    }

    impl_group_or_sequence!(SimpleRestrictionModel,
        (local_simple_type, inline_elements, Option<LocalSimpleType>),
        (choice_facet_any, enums, Vec<ChoiceFacetAny; min=0;>),
    );

    ///  'complexType' uses this
    #[derive(Debug, PartialEq)]
    pub enum TypeDefParticle<'input> {
        Group(Box<super::inline_elements::GroupRef<'input>>),
        All(Box<super::xs::All<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
    }

    impl_enum!(TypeDefParticle,
        impl_singleton_variant!(Group, inline_elements, Box<GroupRef>),
        impl_singleton_variant!(All, xs, Box<All>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
    );
}

pub mod hfp {
    //! Some("http://www.w3.org/2001/XMLSchema-hasFacetAndProperty")

    #[allow(unused_imports)]
    use super::*;
}

pub mod enums {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum ChoiceAllChoiceSequence<'input> {
        All(Box<super::xs::All<'input>>),
        Choice(Box<super::xs::Choice<'input>>),
        Sequence(Box<super::xs::Sequence<'input>>),
    }

    impl_enum!(ChoiceAllChoiceSequence,
        impl_singleton_variant!(All, xs, Box<All>),
        impl_singleton_variant!(Choice, xs, Box<Choice>),
        impl_singleton_variant!(Sequence, xs, Box<Sequence>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceAllChoiceSequence2<'input> {
        All(Box<super::inline_elements::All<'input>>),
        Choice(Box<super::inline_elements::ChoiceSimpleExplicitGroup<'input>>),
        Sequence(Box<super::inline_elements::SequenceSimpleExplicitGroup<'input>>),
    }

    impl_enum!(ChoiceAllChoiceSequence2,
        impl_singleton_variant!(All, inline_elements, Box<All>),
        impl_singleton_variant!(Choice, inline_elements, Box<ChoiceSimpleExplicitGroup>),
        impl_singleton_variant!(Sequence, inline_elements, Box<SequenceSimpleExplicitGroup>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceAnnotationRedefinable<'input> {
        Annotation(Box<super::xs::Annotation<'input>>),
        Redefinable(Box<super::xs::Redefinable<'input>>),
    }

    impl_enum!(ChoiceAnnotationRedefinable,
        impl_singleton_variant!(Annotation, xs, Box<Annotation>),
        impl_singleton_variant!(Redefinable, xs, Box<Redefinable>),
    );

    #[derive(Debug, PartialEq)]
    pub enum AnnotationContent<'input> {
        Appinfo(Box<super::xs::Appinfo<'input>>),
        Documentation(Box<super::xs::Documentation<'input>>),
    }

    impl_enum!(AnnotationContent,
        impl_singleton_variant!(Appinfo, xs, Box<Appinfo>),
        impl_singleton_variant!(Documentation, xs, Box<Documentation>),
    );

    #[derive(Debug, PartialEq)]
    pub enum AttrOrAttrGroup<'input> {
        Attribute(Box<super::inline_elements::Attribute<'input>>),
        AttributeGroup(Box<super::inline_elements::AttributeGroupRef<'input>>),
    }

    impl_enum!(AttrOrAttrGroup,
        impl_singleton_variant!(Attribute, inline_elements, Box<Attribute>),
        impl_singleton_variant!(AttributeGroup, inline_elements, Box<AttributeGroupRef>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceElementAnyGroup<'input> {
        Element(Box<super::inline_elements::LocalElement<'input>>),
        Any(Box<super::xs::Any<'input>>),
        Group(Box<super::inline_elements::Group<'input>>),
    }

    impl_enum!(ChoiceElementAnyGroup,
        impl_singleton_variant!(Element, inline_elements, Box<LocalElement>),
        impl_singleton_variant!(Any, xs, Box<Any>),
        impl_singleton_variant!(Group, inline_elements, Box<Group>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceFacetAny<'input> {
        Facet(Box<super::xs::Facet<'input>>),
        Any(Box<super::support::Any<'input>>),
    }

    impl_enum!(ChoiceFacetAny,
        impl_singleton_variant!(Facet, xs, Box<Facet>),
        impl_singleton_variant!(Any, support, Box<Any>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceRestrictionExtension<'input> {
        Restriction(Box<super::inline_elements::ComplexRestrictionType<'input>>),
        Extension(Box<super::inline_elements::ExtensionType<'input>>),
    }

    impl_enum!(ChoiceRestrictionExtension,
        impl_singleton_variant!(Restriction, inline_elements, Box<ComplexRestrictionType>),
        impl_singleton_variant!(Extension, inline_elements, Box<ExtensionType>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceRestrictionExtension2<'input> {
        Restriction(Box<super::inline_elements::SimpleRestrictionType<'input>>),
        Extension(Box<super::inline_elements::SimpleExtensionType<'input>>),
    }

    impl_enum!(ChoiceRestrictionExtension2,
        impl_singleton_variant!(Restriction, inline_elements, Box<SimpleRestrictionType>),
        impl_singleton_variant!(Extension, inline_elements, Box<SimpleExtensionType>),
    );

    #[derive(Debug, PartialEq)]
    pub enum ChoiceSequenceOpenContentTypeDefParticleSimpleRestrictionModel<'input> {
        SequenceOpenContentTypeDefParticle {
            open_content: Option<Box<super::xs::OpenContent<'input>> >,
            type_def_particle: Box<super::xs::TypeDefParticle<'input>>,
        }
        ,
        SimpleRestrictionModel(Box<super::xs::SimpleRestrictionModel<'input>>),
    }

    impl_enum!(ChoiceSequenceOpenContentTypeDefParticleSimpleRestrictionModel,
        impl_struct_variant!(SequenceOpenContentTypeDefParticle,
            (open_content, xs, Option<Box<OpenContent> >),
            (type_def_particle, xs, Box<TypeDefParticle>),
        ),
        impl_singleton_variant!(SimpleRestrictionModel, xs, Box<SimpleRestrictionModel>),
    );

    #[derive(Debug, PartialEq)]
    pub enum Type<'input> {
        SimpleType(Box<super::inline_elements::LocalSimpleType<'input>>),
        ComplexType(Box<super::inline_elements::LocalComplexType<'input>>),
    }

    impl_enum!(Type,
        impl_singleton_variant!(SimpleType, inline_elements, Box<LocalSimpleType>),
        impl_singleton_variant!(ComplexType, inline_elements, Box<LocalComplexType>),
    );
}

pub mod restrictions {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Debug, PartialEq)] pub struct RestrictNcName<'input>(pub support::NcName<'input>);

    impl_simpletype_restriction!(RestrictNcName, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictName<'input>(pub xs::Name<'input>);

    impl_simpletype_restriction!(RestrictName, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("[\\i-[:]][\\c-[:]]*"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictAnySimpleType<'input>(pub support::AnySimpleType<'input>);

    impl_simpletype_restriction!(RestrictAnySimpleType, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: Some(1),
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDateTime<'input>(pub support::DateTime<'input>);

    impl_simpletype_restriction!(RestrictDateTime, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: Some("required"),
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDecimal<'input>(pub support::Decimal<'input>);

    impl_simpletype_restriction!(RestrictDecimal, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: Some(0),
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("[\\-+]?[0-9]+"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDuration<'input>(pub support::Duration<'input>);

    impl_simpletype_restriction!(RestrictDuration, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("[^DT]*"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDuration2<'input>(pub support::Duration<'input>);

    impl_simpletype_restriction!(RestrictDuration2, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("[^YM]*(T.*)?"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictInt<'input>(pub xs::Int<'input>);

    impl_simpletype_restriction!(RestrictInt, Facets {
        min_exclusive: None,
        min_inclusive: Some(BigFloatNotNaN::from_str("-32768").unwrap()),
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("32767").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictInteger<'input>(pub xs::Integer<'input>);

    impl_simpletype_restriction!(RestrictInteger, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("0").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictInteger2<'input>(pub xs::Integer<'input>);

    impl_simpletype_restriction!(RestrictInteger2, Facets {
        min_exclusive: None,
        min_inclusive: Some(BigFloatNotNaN::from_str("-9223372036854775808").unwrap()),
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("9223372036854775807").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictInteger3<'input>(pub xs::Integer<'input>);

    impl_simpletype_restriction!(RestrictInteger3, Facets {
        min_exclusive: None,
        min_inclusive: Some(BigFloatNotNaN::from_str("0").unwrap()),
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictLong<'input>(pub xs::Long<'input>);

    impl_simpletype_restriction!(RestrictLong, Facets {
        min_exclusive: None,
        min_inclusive: Some(BigFloatNotNaN::from_str("-2147483648").unwrap()),
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("2147483647").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNonNegativeInteger<'input>(pub support::NonNegativeInteger<'input>);

    impl_simpletype_restriction!(RestrictNonNegativeInteger, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("18446744073709551615").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNonNegativeInteger2<'input>(pub support::NonNegativeInteger<'input>);

    impl_simpletype_restriction!(RestrictNonNegativeInteger2, Facets {
        min_exclusive: None,
        min_inclusive: Some(BigFloatNotNaN::from_str("1").unwrap()),
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNonPositiveInteger<'input>(pub xs::NonPositiveInteger<'input>);

    impl_simpletype_restriction!(RestrictNonPositiveInteger, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("-1").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNormalizedString<'input>(pub xs::NormalizedString<'input>);

    impl_simpletype_restriction!(RestrictNormalizedString, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: Some("collapse"),
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictShort<'input>(pub xs::Short<'input>);

    impl_simpletype_restriction!(RestrictShort, Facets {
        min_exclusive: None,
        min_inclusive: Some(BigFloatNotNaN::from_str("-128").unwrap()),
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("127").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictString<'input>(pub support::XmlString<'input>);

    impl_simpletype_restriction!(RestrictString, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: Some("replace"),
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("[a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken2<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken2, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("\\c+"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken3<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken3, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: Some("\\i\\c*"),
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictUnsignedInt<'input>(pub xs::UnsignedInt<'input>);

    impl_simpletype_restriction!(RestrictUnsignedInt, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("65535").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictUnsignedLong<'input>(pub xs::UnsignedLong<'input>);

    impl_simpletype_restriction!(RestrictUnsignedLong, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("4294967295").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictUnsignedShort<'input>(pub xs::UnsignedShort<'input>);

    impl_simpletype_restriction!(RestrictUnsignedShort, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: Some(BigFloatNotNaN::from_str("255").unwrap()),
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["interleave", "suffix"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken2<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken2, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["none", "interleave", "suffix"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken3<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken3, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["optional", "required", "prohibited"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken4<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken4, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["preserve", "replace", "collapse"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken5<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken5, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["prohibited", "optional", "required"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken6<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken6, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["qualified", "unqualified"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken7<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken7, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["skip", "lax", "strict"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken8<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken8, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["substitution", "extension", "restriction", "list", "union"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNmtoken9<'input>(pub xs::Nmtoken<'input>);

    impl_simpletype_restriction!(RestrictNmtoken9, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["unbounded"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictAllNni<'input>(pub xs::AllNni<'input>);

    impl_simpletype_restriction!(RestrictAllNni, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["0", "1"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictBasicNamespaceList<'input>(pub xs::BasicNamespaceList<'input>);

    impl_simpletype_restriction!(RestrictBasicNamespaceList, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: Some(1),
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDerivationControl<'input>(pub xs::DerivationControl<'input>);

    impl_simpletype_restriction!(RestrictDerivationControl, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["extension", "restriction"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDerivationControl2<'input>(pub xs::DerivationControl<'input>);

    impl_simpletype_restriction!(RestrictDerivationControl2, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["extension", "restriction", "list", "union"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDerivationControl3<'input>(pub xs::DerivationControl<'input>);

    impl_simpletype_restriction!(RestrictDerivationControl3, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["extension", "restriction", "substitution"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictDerivationControl4<'input>(pub xs::DerivationControl<'input>);

    impl_simpletype_restriction!(RestrictDerivationControl4, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["list", "union", "restriction", "extension"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictNonNegativeInteger3<'input>(pub support::NonNegativeInteger<'input>);

    impl_simpletype_restriction!(RestrictNonNegativeInteger3, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["0", "1"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken4<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken4, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: None,
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken5<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken5, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["##any", "##other"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken6<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken6, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["##defaultNamespace", "##targetNamespace", "##local"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken7<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken7, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["##defined"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken8<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken8, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["##defined", "##definedSibling"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken9<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken9, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["##targetNamespace", "##local"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });

    #[derive(Debug, PartialEq)] pub struct RestrictToken10<'input>(pub support::Token<'input>);

    impl_simpletype_restriction!(RestrictToken10, Facets {
        min_exclusive: None,
        min_inclusive: None,
        max_exclusive: None,
        max_inclusive: None,
        total_digits: None,
        fraction_digits: None,
        length: None,
        min_length: None,
        max_length: None,
        enumeration: Some(vec!["#all"]),
        white_space: None,
        pattern: None,
        assertion: None,
        explicit_timezone: None,
    });
}

pub mod lists {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct QNameList<'input>(pub Vec<support::QName<'input>>);

    impl_list!(QNameList, support::QName);

    #[derive(Debug, PartialEq)]
    pub struct DerivationControlList<'input>(pub Vec<restrictions::RestrictDerivationControl3<'input>>);

    impl_list!(DerivationControlList, restrictions::RestrictDerivationControl3);

    #[derive(Debug, PartialEq)]
    pub struct DerivationControlList2<'input>(pub Vec<restrictions::RestrictDerivationControl4<'input>>);

    impl_list!(DerivationControlList2, restrictions::RestrictDerivationControl4);

    #[derive(Debug, PartialEq)]
    pub struct ReducedDerivationControlList<'input>(pub Vec<xs::ReducedDerivationControl<'input>>);

    impl_list!(ReducedDerivationControlList, xs::ReducedDerivationControl);

    #[derive(Debug, PartialEq)]
    pub struct TypeDerivationControlList<'input>(pub Vec<xs::TypeDerivationControl<'input>>);

    impl_list!(TypeDerivationControlList, xs::TypeDerivationControl);

    #[derive(Debug, PartialEq)]
    pub struct UnionQNameTokenList<'input>(pub Vec<unions::UnionQNameToken<'input>>);

    impl_list!(UnionQNameTokenList, unions::UnionQNameToken);

    #[derive(Debug, PartialEq)]
    pub struct UnionAnyUriTokenList<'input>(pub Vec<unions::UnionAnyUriToken<'input>>);

    impl_list!(UnionAnyUriTokenList, unions::UnionAnyUriToken);
}

pub mod unions {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum UnionQNameToken<'input> {
        QName(support::QName<'input>),
        Token(restrictions::RestrictToken7<'input>),
    }

    impl_union!(UnionQNameToken, {
        impl_union_variant!(QName),
        impl_union_variant!(Token),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionQNameToken2<'input> {
        QName(support::QName<'input>),
        Token(restrictions::RestrictToken8<'input>),
    }

    impl_union!(UnionQNameToken2, {
        impl_union_variant!(QName),
        impl_union_variant!(Token),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionAnyUriToken<'input> {
        AnyUri(support::AnyUri<'input>),
        Token(restrictions::RestrictToken6<'input>),
    }

    impl_union!(UnionAnyUriToken, {
        impl_union_variant!(AnyUri),
        impl_union_variant!(Token),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionAnyUriToken2<'input> {
        AnyUri(support::AnyUri<'input>),
        Token(restrictions::RestrictToken9<'input>),
    }

    impl_union!(UnionAnyUriToken2, {
        impl_union_variant!(AnyUri),
        impl_union_variant!(Token),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionNonNegativeIntegerNmtoken<'input> {
        NonNegativeInteger(support::NonNegativeInteger<'input>),
        Nmtoken(restrictions::RestrictNmtoken9<'input>),
    }

    impl_union!(UnionNonNegativeIntegerNmtoken, {
        impl_union_variant!(NonNegativeInteger),
        impl_union_variant!(Nmtoken),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionSpecialNamespaceListBasicNamespaceList<'input> {
        SpecialNamespaceList(xs::SpecialNamespaceList<'input>),
        BasicNamespaceList(xs::BasicNamespaceList<'input>),
    }

    impl_union!(UnionSpecialNamespaceListBasicNamespaceList, {
        impl_union_variant!(SpecialNamespaceList),
        impl_union_variant!(BasicNamespaceList),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionTokenDerivationControlList<'input> {
        Token(restrictions::RestrictToken10<'input>),
        DerivationControlList(lists::DerivationControlList<'input>),
    }

    impl_union!(UnionTokenDerivationControlList, {
        impl_union_variant!(Token),
        impl_union_variant!(DerivationControlList),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionTokenReducedDerivationControlList<'input> {
        Token(restrictions::RestrictToken10<'input>),
        ReducedDerivationControlList(lists::ReducedDerivationControlList<'input>),
    }

    impl_union!(UnionTokenReducedDerivationControlList, {
        impl_union_variant!(Token),
        impl_union_variant!(ReducedDerivationControlList),
    });

    #[derive(Debug, PartialEq)]
    pub enum UnionTokenTypeDerivationControlList<'input> {
        Token(restrictions::RestrictToken10<'input>),
        TypeDerivationControlList(lists::TypeDerivationControlList<'input>),
    }

    impl_union!(UnionTokenTypeDerivationControlList, {
        impl_union_variant!(Token),
        impl_union_variant!(TypeDerivationControlList),
    });
}

pub mod sequences {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct SequenceAny<'input> {
        pub any: super::support::Any<'input>,
    }

    impl_group_or_sequence!(SequenceAny,
        (any, support, Any),
    );

    #[derive(Debug, PartialEq)]
    pub struct AnnotatedOpenContent<'input> {
        pub default_open_content: super::xs::DefaultOpenContent<'input>,
        pub annotation: Vec<super::xs::Annotation<'input>>,
    }

    impl_group_or_sequence!(AnnotatedOpenContent,
        (default_open_content, xs, DefaultOpenContent),
        (annotation, xs, Vec<Annotation; min=0;>),
    );

    /// This choice is added simply to make this a valid restriction per the REC
    #[derive(Debug, PartialEq)]
    pub struct SequenceOpenContentTypeDefParticle<'input> {
        pub open_content: Option<super::xs::OpenContent<'input>>,
        pub type_def_particle: super::xs::TypeDefParticle<'input>,
    }

    impl_group_or_sequence!(SequenceOpenContentTypeDefParticle,
        (open_content, xs, Option<OpenContent>),
        (type_def_particle, xs, TypeDefParticle),
    );

    #[derive(Debug, PartialEq)]
    pub struct SequenceSchemaTopAnnotation<'input> {
        pub schema_top: super::xs::SchemaTop<'input>,
        pub annotation: Vec<super::xs::Annotation<'input>>,
    }

    impl_group_or_sequence!(SequenceSchemaTopAnnotation,
        (schema_top, xs, SchemaTop),
        (annotation, xs, Vec<Annotation; min=0;>),
    );

    #[derive(Debug, PartialEq)]
    pub struct UniquenessSpec<'input> {
        pub selector: super::xs::Selector<'input>,
        pub field: Vec<super::xs::Field<'input>>,
    }

    impl_group_or_sequence!(UniquenessSpec,
        (selector, xs, Selector),
        (field, xs, Vec<Field; min=1;>),
    );
}

pub mod inline_elements {
    #[allow(unused_imports)]
    use super::*;

    #[derive(Debug, PartialEq)]
    pub struct All<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub all_model: super::xs::AllModel<'input>,
    }

    impl_element!(All, "http://www.w3.org/2001/XMLSchema", "all", attributes = {
    }, fields = {
        (all_model, xs, AllModel),
    });

    ///  This type is used for 'alternative' elements. 
    #[derive(Debug, PartialEq)]
    pub struct AlternativeAltType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_test: Option<support::XmlString<'input>>,
        pub attr_type: Option<support::QName<'input>>,
        pub attr_xpath_default_namespace: Option<xs::XpathDefaultNamespace<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub type_: Option<super::enums::Type<'input>>,
    }

    impl_element!(AlternativeAltType, "http://www.w3.org/2001/XMLSchema", "alternative", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "test") => attr_test: optional,
        ("http://www.w3.org/2001/XMLSchema", "type") => attr_type: optional,
        ("http://www.w3.org/2001/XMLSchema", "xpathDefaultNamespace") => attr_xpath_default_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (type_, enums, Option<Type>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct AnyWildcard<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_namespace: Option<xs::NamespaceList<'input>>,
        pub attr_not_namespace: Option<restrictions::RestrictBasicNamespaceList<'input>>,
        pub attr_process_contents: Option<restrictions::RestrictNmtoken7<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AnyWildcard, "http://www.w3.org/2001/XMLSchema", "any", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "namespace") => attr_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "notNamespace") => attr_not_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "processContents") => attr_process_contents: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    ///  This type is extended by all types which allow annotation other than <schema> itself 
    #[derive(Debug, PartialEq)]
    pub struct Assertion<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_test: Option<support::XmlString<'input>>,
        pub attr_xpath_default_namespace: Option<xs::XpathDefaultNamespace<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Assertion, "http://www.w3.org/2001/XMLSchema", "assert", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "test") => attr_test: optional,
        ("http://www.w3.org/2001/XMLSchema", "xpathDefaultNamespace") => attr_xpath_default_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct Attribute<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_type: Option<support::QName<'input>>,
        pub attr_use: Option<restrictions::RestrictNmtoken5<'input>>,
        pub attr_default: Option<support::XmlString<'input>>,
        pub attr_fixed: Option<support::XmlString<'input>>,
        pub attr_form: Option<xs::FormChoice<'input>>,
        pub attr_target_namespace: Option<support::AnyUri<'input>>,
        pub attr_inheritable: Option<support::Boolean<'input>>,
        pub attr_name: Option<support::NcName<'input>>,
        pub attr_ref: Option<support::QName<'input>>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub local_simple_type: Option<super::inline_elements::LocalSimpleType<'input>>,
    }

    impl_element!(Attribute, "http://www.w3.org/2001/XMLSchema", "attribute", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "type") => attr_type: optional,
        ("http://www.w3.org/2001/XMLSchema", "use") => attr_use: optional,
        ("http://www.w3.org/2001/XMLSchema", "default") => attr_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "form") => attr_form: optional,
        ("http://www.w3.org/2001/XMLSchema", "targetNamespace") => attr_target_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "inheritable") => attr_inheritable: optional,
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: optional,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (local_simple_type, inline_elements, Option<LocalSimpleType>),
    });

    #[derive(Debug, PartialEq)]
    pub struct AttributeGroupRef<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_ref: support::QName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(AttributeGroupRef, "http://www.w3.org/2001/XMLSchema", "attributeGroup", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ChoiceSimpleExplicitGroup<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(ChoiceSimpleExplicitGroup, "http://www.w3.org/2001/XMLSchema", "choice", attributes = {
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct LocalComplexType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_mixed: Option<support::Boolean<'input>>,
        pub attr_default_attributes_apply: Option<support::Boolean<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub complex_type_model: super::xs::ComplexTypeModel<'input>,
    }

    impl_element!(LocalComplexType, "http://www.w3.org/2001/XMLSchema", "complexType", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "mixed") => attr_mixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "defaultAttributesApply") => attr_default_attributes_apply: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (complex_type_model, xs, ComplexTypeModel),
    });

    #[derive(Debug, PartialEq)]
    pub struct LocalElement<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_type: Option<support::QName<'input>>,
        pub attr_default: Option<support::XmlString<'input>>,
        pub attr_fixed: Option<support::XmlString<'input>>,
        pub attr_nillable: Option<support::Boolean<'input>>,
        pub attr_block: Option<xs::BlockSet<'input>>,
        pub attr_form: Option<xs::FormChoice<'input>>,
        pub attr_target_namespace: Option<support::AnyUri<'input>>,
        pub attr_name: Option<support::NcName<'input>>,
        pub attr_ref: Option<support::QName<'input>>,
        pub attr_min_occurs: Option<support::NonNegativeInteger<'input>>,
        pub attr_max_occurs: Option<xs::AllNni<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub type_: Option<super::enums::Type<'input>>,
        pub alternative_alt_type: Vec<super::inline_elements::AlternativeAltType<'input>>,
        pub identity_constraint: Vec<super::xs::IdentityConstraint<'input>>,
    }

    impl_element!(LocalElement, "http://www.w3.org/2001/XMLSchema", "element", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "type") => attr_type: optional,
        ("http://www.w3.org/2001/XMLSchema", "default") => attr_default: optional,
        ("http://www.w3.org/2001/XMLSchema", "fixed") => attr_fixed: optional,
        ("http://www.w3.org/2001/XMLSchema", "nillable") => attr_nillable: optional,
        ("http://www.w3.org/2001/XMLSchema", "block") => attr_block: optional,
        ("http://www.w3.org/2001/XMLSchema", "form") => attr_form: optional,
        ("http://www.w3.org/2001/XMLSchema", "targetNamespace") => attr_target_namespace: optional,
        ("http://www.w3.org/2001/XMLSchema", "name") => attr_name: optional,
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: optional,
        ("http://www.w3.org/2001/XMLSchema", "minOccurs") => attr_min_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "maxOccurs") => attr_max_occurs: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (type_, enums, Option<Type>),
        (alternative_alt_type, inline_elements, Vec<AlternativeAltType; min=0;>),
        (identity_constraint, xs, Vec<IdentityConstraint; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ExtensionType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_base: support::QName<'input>,
        pub attr_id: Option<xs::Id<'input>>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub open_content: Option<super::xs::OpenContent<'input>>,
        pub type_def_particle: Option<super::xs::TypeDefParticle<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(ExtensionType, "http://www.w3.org/2001/XMLSchema", "extension", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "base") => attr_base: required,
        ("http://www.w3.org/2001/XMLSchema", "id") => attr_id: optional,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (open_content, xs, Option<OpenContent>),
        (type_def_particle, xs, Option<TypeDefParticle>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    ///  No typeDefParticle group reference
    #[derive(Debug, PartialEq)]
    pub struct SimpleExtensionType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_base: support::QName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(SimpleExtensionType, "http://www.w3.org/2001/XMLSchema", "extension", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "base") => attr_base: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct Group<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_min_occurs: Option<support::NonNegativeInteger<'input>>,
        pub attr_max_occurs: Option<support::NonNegativeInteger<'input>>,
        pub attr_ref: support::QName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(Group, "http://www.w3.org/2001/XMLSchema", "group", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "minOccurs") => attr_min_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "maxOccurs") => attr_max_occurs: optional,
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct GroupRef<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_ref: support::QName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
    }

    impl_element!(GroupRef, "http://www.w3.org/2001/XMLSchema", "group", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "ref") => attr_ref: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
    });

    #[derive(Debug, PartialEq)]
    pub struct ComplexRestrictionType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_base: support::QName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub sequence_open_content_type_def_particle: Option<super::sequences::SequenceOpenContentTypeDefParticle<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(ComplexRestrictionType, "http://www.w3.org/2001/XMLSchema", "restriction", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "base") => attr_base: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (sequence_open_content_type_def_particle, sequences, Option<SequenceOpenContentTypeDefParticle>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct SimpleRestrictionType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub attr_base: support::QName<'input>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_restriction_model: Option<super::xs::SimpleRestrictionModel<'input>>,
        pub attr_decls: super::xs::AttrDecls<'input>,
        pub assertions: super::xs::Assertions<'input>,
    }

    impl_element!(SimpleRestrictionType, "http://www.w3.org/2001/XMLSchema", "restriction", attributes = {
        ("http://www.w3.org/2001/XMLSchema", "base") => attr_base: required,
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (simple_restriction_model, xs, Option<SimpleRestrictionModel>),
        (attr_decls, xs, AttrDecls),
        (assertions, xs, Assertions),
    });

    #[derive(Debug, PartialEq)]
    pub struct SequenceSimpleExplicitGroup<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub nested_particle: Vec<super::xs::NestedParticle<'input>>,
    }

    impl_element!(SequenceSimpleExplicitGroup, "http://www.w3.org/2001/XMLSchema", "sequence", attributes = {
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (nested_particle, xs, Vec<NestedParticle; min=0;>),
    });

    #[derive(Debug, PartialEq)]
    pub struct LocalSimpleType<'input> {
        pub attrs: HashMap<FullName<'input>, &'input str>,
        pub annotation: Option<super::xs::Annotation<'input>>,
        pub simple_derivation: super::xs::SimpleDerivation<'input>,
    }

    impl_element!(LocalSimpleType, "http://www.w3.org/2001/XMLSchema", "simpleType", attributes = {
    }, fields = {
        (annotation, xs, Option<Annotation>),
        (simple_derivation, xs, SimpleDerivation),
    });
}
