//! Contains a more idiomatic AST to work on, and functions to generate it from the full AST.

use std::hash::Hash;

use xmlparser::{TextUnescape, XmlSpace};

use asts;
use asts::recursive::{ComplexType, SimpleType};
use names::FullName;
use parser::*;
use primitives::{AnyUri, NonNegativeInteger, QName};
use support::Facets;
use toplevel::Toplevel;

pub const SCHEMA_URI: &'static str = "http://www.w3.org/2001/XMLSchema";

fn parse_min_occurs(x: &Option<NonNegativeInteger>) -> usize {
    match x {
        None => 1,
        Some(n) => n.0 as usize,
    }
}
fn parse_max_occurs(x: &Option<unions::UnionNonNegativeIntegerNmtoken>) -> usize {
    match x {
        None => 1,
        Some(unions::UnionNonNegativeIntegerNmtoken::NonNegativeInteger(n)) => n.0 as usize,
        Some(unions::UnionNonNegativeIntegerNmtoken::Nmtoken(restrictions::Unbounded(_))) => {
            usize::max_value()
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[must_use]
pub struct Documentation<'input>(Vec<&'input str>);
impl<'input> Documentation<'input> {
    pub fn new() -> Documentation<'input> {
        Documentation(Vec::new())
    }
    pub fn extend(&mut self, v: &Documentation<'input>) {
        self.0.extend(v.0.iter());
    }
}

impl<'input> ToString for Documentation<'input> {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|doc| TextUnescape::unescape(doc, XmlSpace::Default))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttrUse {
    Prohibited,
    Required,
    Optional,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attrs<'input> {
    pub named: Vec<(FullName<'input>, AttrUse, Option<OutSimpleType<'input>>)>,
    pub refs: Vec<(Option<FullName<'input>>, AttrUse, FullName<'input>)>,
    pub group_refs: Vec<FullName<'input>>,
    pub any_attributes: bool,
}
impl<'input> Attrs<'input> {
    pub fn new() -> Attrs<'input> {
        Attrs {
            named: Vec::new(),
            refs: Vec::new(),
            group_refs: Vec::new(),
            any_attributes: false,
        }
    }
    fn extend(&mut self, other: Attrs<'input>) {
        let Attrs {
            named,
            refs,
            group_refs,
            any_attributes,
        } = other;
        self.named.extend(named);
        self.refs.extend(refs);
        self.group_refs.extend(group_refs);
        self.any_attributes |= any_attributes;
    }
}

pub type OutSimpleType<'input> = asts::recursive::SimpleType<'input>;
pub type OutComplexType<'input> =
    asts::recursive::ComplexType<'input, Attrs<'input>, ComplexTypeExtra<'input, Attrs<'input>>>;

/// Other possibilities for SimpleType that will be shaven off by
/// other passes
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplexTypeExtra<'input, TAttrs> {
    AttrDecl(
        TAttrs,
        Box<ComplexType<'input, TAttrs, ComplexTypeExtra<'input, TAttrs>>>,
    ),
}

#[derive(Debug)]
pub struct SimpleToplevel<'input> {
    pub target_namespace: Option<&'input str>,
    pub element_form_default_qualified: bool,
    pub attribute_form_default_qualified: bool,
    pub elements: HashMap<FullName<'input>, OutComplexType<'input>>,
    pub simple_types: HashMap<FullName<'input>, OutSimpleType<'input>>,
    pub complex_types: HashMap<FullName<'input>, OutComplexType<'input>>,
    pub groups: HashMap<FullName<'input>, OutComplexType<'input>>,
    pub attribute_groups: HashMap<FullName<'input>, Attrs<'input>>,
}

fn hashmap_map<K: Hash + Eq, V1, V2, F>(map: HashMap<K, V1>, mut mapper: F) -> HashMap<K, V2>
where
    F: FnMut(V1) -> V2,
{
    map.into_iter().map(|(k, v)| (k, mapper(v))).collect()
}

impl<'input> SimpleToplevel<'input> {
    pub fn new_from_toplevel<'ast>(toplevel: Toplevel<'ast, 'input>) -> SimpleToplevel<'input> {
        let Toplevel {
            target_namespace,
            element_form_default_qualified,
            attribute_form_default_qualified,
            elements,
            simple_types,
            complex_types,
            groups,
            attribute_groups,
        } = toplevel;

        let mut processor = Processor {
            target_namespace,
            element_form_default_qualified,
            _phantom: PhantomData::default(),
        };

        SimpleToplevel {
            target_namespace,
            element_form_default_qualified,
            attribute_form_default_qualified,
            elements: hashmap_map(elements, |e| processor.process_toplevel_element(e)),
            simple_types: hashmap_map(simple_types, |t| processor.process_toplevel_simple_type(t)),
            complex_types: hashmap_map(complex_types, |t| {
                processor.process_toplevel_complex_type(t)
            }),
            groups: hashmap_map(groups, |g| processor.process_toplevel_group(g)),
            attribute_groups: hashmap_map(attribute_groups, |g| {
                processor.process_toplevel_attribute_group(g)
            }),
        }
    }
}

struct Processor<'ast, 'input: 'ast> {
    target_namespace: Option<&'input str>,
    element_form_default_qualified: bool,
    _phantom: PhantomData<&'ast ()>, // To avoid repetition in each method
}

impl<'ast, 'input: 'ast> Processor<'ast, 'input> {
    fn process_toplevel_element(
        &mut self,
        element: &'ast xs::Element<'input>,
    ) -> OutComplexType<'input> {
        let xs::Element {
            ref attr_type,
            ref attr_name,
            type_: ref child_type,
            ..
        } = element;

        // TODO: substitution group

        match (attr_type, &child_type) {
            (None, Some(ref c)) => match c {
                enums::Type::SimpleType(ref e) => ComplexType::Simple(self.process_local_simple_type(e)),
                enums::Type::ComplexType(ref e) => self.process_local_complex_type(e),
            },
            (Some(t), None) => ComplexType::Alias(FullName::from_qname(t, self.target_namespace)),
            (None, None) => ComplexType::Empty,
            (Some(ref t1), Some(ref t2)) => {
                panic!(
                "Toplevel element '{}:{}' has both a type attribute ({:?}) and a child type ({:?}).",
                self.target_namespace.unwrap_or(""), attr_name.0, t1, t2
            )
            }
        }
    }

    fn process_toplevel_complex_type(
        &mut self,
        complex_type: &'ast xs::ComplexType<'input>,
    ) -> OutComplexType<'input> {
        let xs::ComplexType {
            ref complex_type_model,
            ..
        } = complex_type;

        self.process_complex_type_model(complex_type_model)
    }

    fn process_complex_type_model(
        &mut self,
        complex_type_model: &'ast xs::ComplexTypeModel<'input>,
    ) -> OutComplexType<'input> {
        match complex_type_model {
            xs::ComplexTypeModel::SimpleContent(_) => unimplemented!("simpleContent"),
            xs::ComplexTypeModel::ComplexContent(ref model) => self.process_complex_content(model),
            xs::ComplexTypeModel::CompleteContentModel {
                ref open_content,
                ref type_def_particle,
                ref attr_decls,
                ref assertions,
            } => self.process_complete_content_model(
                open_content,
                type_def_particle,
                attr_decls,
                assertions,
            ),
        }
    }

    fn process_local_complex_type(
        &mut self,
        complex_type: &'ast inline_elements::LocalComplexType<'input>,
    ) -> OutComplexType<'input> {
        let inline_elements::LocalComplexType {
            ref complex_type_model,
            ..
        } = complex_type;
        self.process_complex_type_model(complex_type_model)
    }

    fn process_toplevel_simple_type(
        &mut self,
        simple_type: &'ast xs::SimpleType<'input>,
    ) -> SimpleType<'input> {
        let xs::SimpleType {
            ref simple_derivation,
            ..
        } = simple_type;
        match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => self.process_simple_restriction(e),
            xs::SimpleDerivation::List(ref e) => self.process_list(e),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e),
        }
    }

    fn process_local_simple_type(
        &mut self,
        simple_type: &'ast inline_elements::LocalSimpleType<'input>,
    ) -> SimpleType<'input> {
        let inline_elements::LocalSimpleType {
            ref simple_derivation,
            ..
        } = simple_type;
        match simple_derivation {
            xs::SimpleDerivation::Restriction(e) => self.process_simple_restriction(e),
            xs::SimpleDerivation::List(ref e) => self.process_list(e),
            xs::SimpleDerivation::Union(ref e) => self.process_union(e),
        }
    }

    fn process_simple_restriction(
        &mut self,
        restriction: &'ast xs::Restriction<'input>,
    ) -> SimpleType<'input> {
        let xs::Restriction {
            ref attr_base,
            ref simple_restriction_model,
            ..
        } = restriction;
        let base = attr_base;
        let base = base.unwrap_or(QName {
            namespace: Some(SCHEMA_URI),
            local_name: "anySimpleType",
        });
        let xs::SimpleRestrictionModel {
            ref local_simple_type,
            ref choice_facet_any,
        } = simple_restriction_model;
        let facets = self.process_facets(choice_facet_any);

        let base = FullName::from_qname(&base, self.target_namespace);

        match local_simple_type {
            Some(inline_elements::LocalSimpleType { .. }) => {
                SimpleType::Restriction(base, facets) // TODO: use the simple_derivation
            }
            None => SimpleType::Restriction(base, facets),
        }
    }

    fn process_facets(
        &mut self,
        facet_list: &Vec<enums::ChoiceFacetAny<'input>>,
    ) -> Facets<'input> {
        let mut facets = Facets::default();
        use parser::xs::Facet::*;
        for facet_or_any in facet_list {
            match facet_or_any {
                enums::ChoiceFacetAny::Facet(e) => {
                    match **e {
                        FacetHead(_) => panic!("abstract element"),
                        MinExclusive(ref e) => {
                            facets.min_exclusive =
                                Some(e.attr_value.0.parse().expect("invalid minexclusive"))
                        }
                        MinInclusive(ref e) => {
                            facets.min_inclusive =
                                Some(e.attr_value.0.parse().expect("invalid mininclusive"))
                        }
                        MaxExclusive(ref e) => {
                            facets.max_exclusive =
                                Some(e.attr_value.0.parse().expect("invalid maxexclusive"))
                        }
                        MaxInclusive(ref e) => {
                            facets.max_inclusive =
                                Some(e.attr_value.0.parse().expect("invalid maxinclusive"))
                        }
                        TotalDigits(ref e) => facets.total_digits = Some(e.attr_value.0),
                        FractionDigits(ref e) => facets.fraction_digits = Some(e.attr_value.0),
                        Length(ref e) => facets.length = Some(e.attr_value.0 as usize),
                        MinLength(ref e) => facets.min_length = Some(e.attr_value.0 as usize),
                        MaxLength(ref e) => facets.max_length = Some(e.attr_value.0 as usize),
                        Enumeration(ref e) => facets
                            .enumeration
                            .get_or_insert(Vec::new())
                            .push(e.attr_value.0),
                        WhiteSpace(ref e) => facets.white_space = Some(((e.attr_value.0).0).0),
                        Pattern(ref e) => facets.pattern = Some(e.attr_value.0),
                        Assertion(_) => unimplemented!("assertion facet"),
                        ExplicitTimezone(ref e) => {
                            facets.explicit_timezone = Some(((e.attr_value.0).0).0)
                        }
                    };
                }
                enums::ChoiceFacetAny::Any(_) => (), // TODO (probably just whitespaces)
            }
        }
        facets
    }

    fn process_list(&mut self, list: &'ast xs::List<'input>) -> SimpleType<'input> {
        let item_type = list.attr_item_type;
        let item_type = item_type
            .as_ref()
            .map(|n| FullName::from_qname(n, self.target_namespace));

        let t = match (item_type, &list.local_simple_type) {
            (None, Some(st)) => self.process_local_simple_type(st),
            (Some(n), None) => SimpleType::Alias(n),
            (None, None) => panic!("<list> with no itemType or child type."),
            (Some(ref t1), Some(ref t2)) => panic!(
                "<list> has both an itemType attribute ({:?}) and a child type ({:?}).",
                t1, t2
            ),
        };

        SimpleType::List(Box::new(t))
    }

    fn process_union(&mut self, union: &'ast xs::Union<'input>) -> SimpleType<'input> {
        let member_types = union
            .local_simple_type
            .iter()
            .map(|t| self.process_local_simple_type(t))
            .collect();

        SimpleType::Union(member_types)
    }

    fn process_toplevel_group(&mut self, group: &'ast xs::Group<'input>) -> OutComplexType<'input> {
        let xs::Group {
            choice_all_choice_sequence: ref content,
            ..
        } = group;

        match content {
            enums::ChoiceAllChoiceSequence::All(_) => unimplemented!("all"),
            enums::ChoiceAllChoiceSequence::Choice(e) => self.process_choice(e),
            enums::ChoiceAllChoiceSequence::Sequence(e) => self.process_sequence(e),
        }
    }

    fn process_toplevel_attribute_group(
        &mut self,
        group: &'ast xs::AttributeGroup<'input>,
    ) -> Attrs<'input> {
        self.process_attr_decls(&group.attr_decls)
    }

    fn process_attr_decls(&mut self, attr_decls: &'ast xs::AttrDecls<'input>) -> Attrs<'input> {
        let mut attrs = Attrs::new();
        for attr_decl in &attr_decls.attribute {
            match attr_decl {
                enums::AttrOrAttrGroup::Attribute(e) => {
                    let name = e
                        .attr_name
                        .as_ref()
                        .map(|ncn| FullName::new(self.target_namespace, ncn.0));
                    let type_attr: Option<QName<'input>> = e.attr_type;
                    let use_ = match e.attr_use.as_ref().map(|x| ((x.0).0).0) {
                        Some("prohibited") => AttrUse::Prohibited,
                        Some("required") => AttrUse::Required,
                        Some("optional") => AttrUse::Optional,
                        None => AttrUse::Optional, // TODO
                        Some(s) => panic!("Unknown attribute value use={:?}", s),
                    };
                    match (name, e.attr_ref, type_attr, &e.local_simple_type) {
                        (Some(name), None, Some(t), None) => {
                            let t = FullName::from_qname(&t, self.target_namespace);
                            attrs.named.push((name, use_, Some(SimpleType::Alias(t))));
                        }
                        (Some(name), None, None, Some(t)) => {
                            let t = self.process_local_simple_type(t);
                            attrs.named.push((name, use_, Some(t)));
                        }
                        (Some(name), None, None, None) => attrs.named.push((name, use_, None)),
                        (None, None, None, None) => panic!("no attribute on <attribute>."),
                        (_, _, Some(ref t1), Some(ref t2)) => panic!(
                            "<attribute> has both a type attribute ({:?}) and a child type ({:?}).",
                            t1, t2
                        ),
                        (None, None, Some(_), None) | (None, None, None, Some(_)) => {
                            panic!("<attribute> has a type but no name.")
                        }
                        (_, Some(_), Some(_), None) | (_, Some(_), None, Some(_)) => {
                            panic!("<attribute> has a type and a ref.")
                        }
                        (_, Some(_ref), None, None) => (), // TODO
                    }
                }
                enums::AttrOrAttrGroup::AttributeGroup(e) => {
                    attrs
                        .group_refs
                        .push(FullName::from_qname(&e.attr_ref, self.target_namespace));
                }
            }
        }
        if attr_decls.any_attribute.is_some() {
            attrs.any_attributes = true;
        }
        attrs
    }

    fn process_complex_content(
        &mut self,
        model: &'ast xs::ComplexContent<'input>,
    ) -> OutComplexType<'input> {
        let xs::ComplexContent {
            ref choice_restriction_extension,
            ..
        } = model;
        match choice_restriction_extension {
            enums::ChoiceRestrictionExtension::Restriction(ref r) => {
                let inline_elements::ComplexRestrictionType {
                    ref attr_base,
                    ref sequence_open_content_type_def_particle,
                    ..
                } = **r;
                match sequence_open_content_type_def_particle {
                    Some(sequences::SequenceOpenContentTypeDefParticle {
                        type_def_particle,
                        ..
                    }) => self.process_complex_restriction(attr_base, type_def_particle),
                    None => ComplexType::Empty,
                }
            }
            enums::ChoiceRestrictionExtension::Extension(ref e) => {
                let inline_elements::ExtensionType {
                    ref attrs,
                    ref attr_base,
                    ref type_def_particle,
                    ..
                } = **e;
                match type_def_particle {
                    Some(type_def_particle) => {
                        self.process_extension(attrs, attr_base, type_def_particle)
                    }
                    None => self.process_trivial_extension(attrs, attr_base),
                }
            }
        }
    }

    fn process_extension(
        &mut self,
        _attrs: &'ast HashMap<FullName<'input>, &'input str>,
        attr_base: &'ast QName<'input>,
        type_def_particle: &'ast xs::TypeDefParticle<'input>,
    ) -> OutComplexType<'input> {
        let base = FullName::from_qname(attr_base, self.target_namespace);
        ComplexType::Extension(
            base,
            Box::new(self.process_type_def_particle(type_def_particle)),
        )
    }

    fn process_trivial_extension(
        &mut self,
        _attrs: &'ast HashMap<FullName<'input>, &'input str>,
        attr_base: &'ast QName<'input>,
    ) -> OutComplexType<'input> {
        let base = FullName::from_qname(&attr_base, self.target_namespace);
        ComplexType::Alias(base)
    }

    fn process_complete_content_model(
        &mut self,
        _open_content: &'ast Option<Box<xs::OpenContent<'input>>>,
        type_def_particle: &'ast Option<Box<xs::TypeDefParticle<'input>>>,
        attr_decls: &'ast xs::AttrDecls<'input>,
        _assertions: &'ast xs::Assertions<'input>,
    ) -> OutComplexType<'input> {
        let ty = match type_def_particle.as_ref() {
            Some(type_def_particle) => self.process_type_def_particle(type_def_particle),
            None => ComplexType::Empty,
        };
        ComplexType::Extra(ComplexTypeExtra::AttrDecl(
            self.process_attr_decls(attr_decls),
            Box::new(ty),
        ))
    }

    fn process_complex_restriction(
        &mut self,
        attr_base: &'ast QName<'input>,
        type_def_particle: &'ast xs::TypeDefParticle<'input>,
    ) -> OutComplexType<'input> {
        // TODO: use the base
        let base = FullName::from_qname(attr_base, self.target_namespace);
        let ty = self.process_type_def_particle(type_def_particle);
        ComplexType::Restriction(base, Box::new(ty))
    }

    fn process_type_def_particle(
        &mut self,
        particle: &'ast xs::TypeDefParticle<'input>,
    ) -> OutComplexType<'input> {
        match particle {
            xs::TypeDefParticle::Group(e) => self.process_group_ref(e),
            xs::TypeDefParticle::All(_) => unimplemented!("all"),
            xs::TypeDefParticle::Choice(e) => self.process_choice(e),
            xs::TypeDefParticle::Sequence(e) => self.process_sequence(e),
        }
    }

    fn process_group_ref(
        &mut self,
        group_ref: &'ast inline_elements::GroupRef<'input>,
    ) -> OutComplexType<'input> {
        let inline_elements::GroupRef {
            ref attr_ref,
            ref attr_min_occurs,
            ref attr_max_occurs,
            ..
        } = group_ref;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);
        let ref_ = FullName::from_qname(attr_ref, self.target_namespace);

        ComplexType::GroupRef(min_occurs, max_occurs, ref_)
    }

    fn process_choice(&mut self, choice: &'ast xs::Choice<'input>) -> OutComplexType<'input> {
        let xs::Choice {
            ref attr_min_occurs,
            ref attr_max_occurs,
            ref nested_particle,
            ..
        } = choice;
        let particles = nested_particle;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);

        let items = particles
            .iter()
            .map(|particle| self.process_nested_particle(particle))
            .collect();

        ComplexType::Choice(min_occurs, max_occurs, items)
    }

    fn process_sequence(&mut self, seq: &'ast xs::Sequence<'input>) -> OutComplexType<'input> {
        let xs::Sequence {
            ref attr_min_occurs,
            ref attr_max_occurs,
            ref nested_particle,
            ..
        } = seq;
        let particles = nested_particle;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);

        let items = particles
            .iter()
            .map(|particle| self.process_nested_particle(particle))
            .collect();

        ComplexType::Sequence(min_occurs, max_occurs, items)
    }

    fn process_nested_particle(
        &mut self,
        particle: &'ast xs::NestedParticle<'input>,
    ) -> OutComplexType<'input> {
        match particle {
            xs::NestedParticle::Element(e) => self.process_local_element(e),
            xs::NestedParticle::Group(e) => self.process_group_ref(e),
            xs::NestedParticle::Choice(e) => self.process_choice(e),
            xs::NestedParticle::Sequence(e) => self.process_sequence(e),
            xs::NestedParticle::Any(e) => self.process_any(e),
        }
    }

    fn process_any(&mut self, _any: &'ast xs::Any<'input>) -> OutComplexType<'input> {
        ComplexType::Any
    }

    fn process_local_element(
        &mut self,
        element: &'ast inline_elements::LocalElement<'input>,
    ) -> OutComplexType<'input> {
        let inline_elements::LocalElement {
            ref attr_name,
            ref attr_ref,
            ref attr_min_occurs,
            ref attr_max_occurs,
            ref attr_type,
            ref attr_form,
            ref attr_target_namespace,
            ref type_,
            ..
        } = element;
        let name = attr_name;
        let type_attr = attr_type;
        let min_occurs = parse_min_occurs(attr_min_occurs);
        let max_occurs = parse_max_occurs(attr_max_occurs);

        if let Some(ref_) = attr_ref {
            if let Some(name) = name {
                panic!("<element> has both ref={:?} and name={:?}", ref_, name);
            }
            if let Some(attr_target_namespace) = attr_target_namespace {
                panic!(
                    "<element> has both ref={:?} and target_namespace={:?}",
                    ref_, attr_target_namespace
                );
            }
            if let Some(attr_form) = attr_form {
                panic!("<element> has both ref={:?} and form={:?}", ref_, attr_form);
            }
            let ref_ = FullName::from_qname(ref_, self.target_namespace);
            ComplexType::ElementRef(min_occurs, max_occurs, ref_)
        } else {
            let name = name.as_ref().expect("<element> has no name.").0;

            // https://www.w3.org/TR/xmlschema11-1/#dcl.elt.local
            let qualified_form = match attr_form.as_ref().map(|x| ((x.0).0).0) {
                Some("qualified") => true,
                Some("unqualified") => false,
                None => self.element_form_default_qualified,
                _ => unreachable!(),
            };
            let namespace = match (attr_target_namespace, qualified_form) {
                (Some(AnyUri(target_namespace)), _) => Some(*target_namespace),
                (None, true) => self.target_namespace,
                (None, false) => None,
            };

            let t = match (type_attr, &type_) {
                (None, Some(enums::Type::SimpleType(ref e))) => {
                    ComplexType::Simple(self.process_local_simple_type(e))
                }
                (None, Some(enums::Type::ComplexType(ref e))) => self.process_local_complex_type(e),
                (Some(t), None) => {
                    let t = FullName::from_qname(t, self.target_namespace);
                    ComplexType::Alias(t)
                }
                (None, None) => ComplexType::Empty,
                (Some(ref t1), Some(ref t2)) => panic!(
                    "Element '{:?}' has both a type attribute ({:?}) and a child type ({:?}).",
                    name, t1, t2
                ),
            };
            ComplexType::Element(
                min_occurs,
                max_occurs,
                FullName::new(namespace, name),
                Attrs::new(),
                Box::new(t),
            )
        }
    }
}
