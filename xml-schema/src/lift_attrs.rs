//! Pushes attr definitions from the inner types of an element to the
//! top-level element
//!
//! This module must compute a transitive closure in case of circular
//! references:
//! "Circular reference is not disallowed. [...] The effect is to take the
//! transitive closure of the reference relation"
//! https://www.w3.org/TR/xmlschema11-1/#declare-attributeGroup-core

use std::collections::{HashMap, HashSet};

use asts;
use asts::recursive::ComplexType as RComplexType;
use names::FullName;
use utils::Bottom;

use processor2::ComplexTypeExtra as InComplexTypeExtra;
use processor2::OutAttrs as InAttrs;
use processor2::OutComplexType as InComplexType;
use processor2::OutSimpleType as InSimpleType;

pub type OutSimpleType<'input> = InSimpleType<'input>;
pub type OutComplexType<'input> = asts::recursive::ComplexType<'input, OutAttrs<'input>, Bottom>;
pub type OutAttrs<'input> = InAttrs<'input>;

pub struct AttrsLifter<'input> {
    /// For each type name, stores a list of types that reference it.
    /// So if the former's list of attrs is updated, then the latter's
    /// must be updated as well.
    reverse_deps: HashMap<FullName<'input>, HashSet<FullName<'input>>>,

    /// (A subset of) the attrs of each complex type. Can converge to the full
    /// attrs by calling `make_transitive_closure()`
    attrs_of_complex_type: HashMap<FullName<'input>, OutAttrs<'input>>,

    /// Set of elements of `attrs_of_complex_type` that are strict subset of
    /// what they should be, and therefore should be updated.
    outdated_complex_types: HashSet<FullName<'input>>,

    complex_types: HashMap<FullName<'input>, InComplexType<'input>>,
}

impl<'input> AttrsLifter<'input> {
    pub fn with_capacity(capacity: usize) -> AttrsLifter<'input> {
        AttrsLifter {
            reverse_deps: HashMap::with_capacity(capacity),
            attrs_of_complex_type: HashMap::with_capacity(capacity),
            outdated_complex_types: HashSet::with_capacity(capacity),
            complex_types: HashMap::with_capacity(capacity),
        }
    }

    pub fn add_complex_type(
        &mut self,
        name: FullName<'input>,
        complex_type: InComplexType<'input>,
    ) {
        self.add_reverse_deps(name, &complex_type);
        self.complex_types.insert(name, complex_type);
        self.outdated_complex_types.insert(name);
    }

    pub fn make_transitive_closure(&mut self) {
        while let Some(&name) = self.outdated_complex_types.iter().next() {
            let complex_type = self.complex_types.get(&name).expect("Name {} was supposed to be updated, but it missing from AttrsLifter.complex_types.");
            let attrs = self.get_attrs_step(complex_type);
            if self.attrs_of_complex_type.get(&name) != attrs.as_ref() {
                self.outdated_complex_types.insert(name);
                let attrs = attrs.expect(
                    "attrs were Some() but became None while computing transitive closure.",
                );
                self.attrs_of_complex_type.insert(name, attrs);
            }
        }
    }

    fn add_reverse_deps(
        &mut self,
        my_name: FullName<'input>,
        complex_type: &InComplexType<'input>,
    ) {
        let add_rev_dep = &mut |rev_dep| {
            self.reverse_deps
                .get_mut(rev_dep)
                .expect(&format!("Reverse deps map is missing {:?} entry", rev_dep))
                .insert(my_name);
        };

        match complex_type {
            // Trivial cases
            RComplexType::Any => {}
            RComplexType::Empty => {}
            RComplexType::Alias(fullname) => add_rev_dep(fullname),
            // The actual work
            RComplexType::Extension(base, inner) | RComplexType::Restriction(base, inner) => {
                add_rev_dep(base);
                self.add_reverse_deps(my_name, inner);
            }
            RComplexType::ElementRef(_min_occurs, _max_occurs, fullname) => add_rev_dep(fullname),
            RComplexType::Choice(_min_occurs, _max_occurs, inners)
            | RComplexType::Sequence(_min_occurs, _max_occurs, inners) => {
                for inner in inners {
                    self.add_reverse_deps(my_name, inner);
                }
            }
            RComplexType::Simple(_simple_type) => {}

            RComplexType::Element(_min_occurs, _max_occurs, _fullname, _attrs, inner) => {
                self.add_reverse_deps(my_name, inner);
            }
            RComplexType::GroupRef(_min_occurs, _max_occurs, _fullname) => unimplemented!(),
            RComplexType::Extra(InComplexTypeExtra::AttrDecl(_attrs, inner)) => {
                self.add_reverse_deps(my_name, inner);
            }
        }
    }

    fn get_attrs_step(&self, complex_type: &InComplexType<'input>) -> Option<OutAttrs<'input>> {
        let merge_attrs = |attrs1: Option<&OutAttrs<'input>>, attrs2| match (attrs1, attrs2) {
            (None, attrs2) => attrs2,
            (Some(attrs1), None) => Some(attrs1.clone()),
            (Some(attrs1), Some(attrs2)) => {
                let mut attrs: OutAttrs<'input> = attrs1.clone();
                attrs.extend(attrs2);
                Some(attrs)
            }
        };
        match complex_type {
            RComplexType::Any => None,
            RComplexType::Empty => None,
            RComplexType::Alias(fullname) => self.attrs_of_complex_type.get(fullname).cloned(),

            RComplexType::Extension(base, inner) => merge_attrs(
                self.attrs_of_complex_type.get(base),
                self.get_attrs_step(inner),
            ),
            RComplexType::Restriction(base, inner) => {
                // Attributes are inherited from the base:
                // "However, attribute declarations do not need to be repeated in the derived type definition"
                // https://www.w3.org/TR/xmlschema-0/#DerivByRestrict
                merge_attrs(
                    self.attrs_of_complex_type.get(base),
                    self.get_attrs_step(inner),
                )
            }
            RComplexType::ElementRef(_min_occurs, _max_occurs, _fullname) => None,

            RComplexType::Choice(_min_occurs, _max_occurs, inners)
            | RComplexType::Sequence(_min_occurs, _max_occurs, inners) => {
                for inner in inners {
                    if self.get_attrs_step(inner).is_some() {
                        unimplemented!(
                            "Sequence/choice got attribute declaration. \
                             I don't know what to do with that."
                        );
                    }
                }
                None
            }
            RComplexType::Simple(_simple_type) => None,

            RComplexType::Element(_min_occurs, _max_occurs, _fullname, _attrs, _inner) => {
                // Elements capture the attrs for themselves and don't pass any up
                None
            }
            RComplexType::GroupRef(_min_occurs, _max_occurs, fullname) => {
                self.attrs_of_complex_type.get(fullname).cloned()
            }
            RComplexType::Extra(InComplexTypeExtra::AttrDecl(attrs, inner)) => {
                match self.get_attrs_step(inner) {
                    Some(inner_attrs) => {
                        let mut attrs = attrs.clone();
                        attrs.extend(inner_attrs);
                        Some(attrs)
                    }
                    None => Some(attrs.clone()),
                }
            }
        }
    }
}
