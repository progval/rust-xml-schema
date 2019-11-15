/// Replaces `ComplexType::GroupRef`, `Attrs.refs` with the content of their target. (ie. inlines them)
use std::collections::HashMap;

use asts;
use asts::non_recursive::ComplexType as NRComplexType;
use asts::non_recursive::ConcreteName;
use asts::non_recursive::SimpleType as NRSimpleType;
use attrs::with_refs::Attrs as InAttrs;
use attrs::AttrUse;
use names::FullName;
use utils::Bottom;

use name_allocator::ComplexTypeExtra as InComplexTypeExtra;
use name_allocator::OutComplexType as InComplexType;
use name_allocator::OutSimpleType as InSimpleType;

pub type OutSimpleType<'input> = InSimpleType<'input>;
pub type OutComplexType<'input> =
    asts::non_recursive::ComplexType<'input, Attrs<'input, ConcreteName>, Bottom>;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attrs<'input, TSimpleType: Clone> {
    pub named: Vec<(FullName<'input>, AttrUse, Option<TSimpleType>)>,
    pub refs: Vec<(Option<FullName<'input>>, AttrUse, FullName<'input>)>,
    pub group_refs: Vec<FullName<'input>>,
    pub any_attributes: bool,
}

impl<'input, TSimpleType> Attrs<'input, TSimpleType>
where
    TSimpleType: Clone,
{
    fn extend(&mut self, other: Attrs<'input, TSimpleType>) {
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

pub type OutAttrs<'input> = Attrs<'input, OutSimpleType<'input>>;

pub fn ungroup_complex_type<'input>(
    _fullname_to_concrete_name: &HashMap<FullName<'input>, ConcreteName>,
    _groups: HashMap<FullName<'input>, &InComplexType<'input>>,
    complex_type: InComplexType<'input>,
) -> OutComplexType<'input> {
    match complex_type {
        // Trivial cases
        NRComplexType::Any => NRComplexType::Any,
        NRComplexType::Empty => NRComplexType::Empty,
        NRComplexType::Alias(cn) => NRComplexType::Alias(cn),
        NRComplexType::Extension(cn1, cn2) => NRComplexType::Extension(cn1, cn2),
        NRComplexType::Restriction(cn1, cn2) => NRComplexType::Restriction(cn1, cn2),
        NRComplexType::ElementRef(min_occurs, max_occurs, cn) => {
            NRComplexType::ElementRef(min_occurs, max_occurs, cn)
        }
        NRComplexType::Choice(min_occurs, max_occurs, cns) => {
            NRComplexType::Choice(min_occurs, max_occurs, cns)
        }
        NRComplexType::Sequence(min_occurs, max_occurs, cns) => {
            NRComplexType::Sequence(min_occurs, max_occurs, cns)
        }
        NRComplexType::Simple(cn) => NRComplexType::Simple(cn),

        // The actual work
        NRComplexType::Element(min_occurs, max_occurs, fullname, attrs, cn) => {
            NRComplexType::Element(min_occurs, max_occurs, fullname, attrs, cn)
        }
        NRComplexType::Extra(InComplexTypeExtra::GroupRef(_min_occurs, _max_occurs, _cn)) => {
            unimplemented!()
        }
    }
}