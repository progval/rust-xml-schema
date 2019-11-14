//! Pushes attr definitions from the inner types of an element to the top-level element

use asts;
use names::FullName;
use processor2::AttrUse;
use utils::Bottom;

use processor2::OutComplexType as InComplexType;
use processor2::OutSimpleType as InSimpleType;

pub type OutSimpleType<'input> = InSimpleType<'input>;
pub type OutComplexType<'input> =
    asts::recursive::ComplexType<'input, Attrs<'input, OutSimpleType<'input>>, Bottom>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attrs<'input, TSimpleType> {
    pub named: Vec<(FullName<'input>, AttrUse, Option<TSimpleType>)>,
    pub refs: Vec<(Option<FullName<'input>>, AttrUse, FullName<'input>)>,
    pub group_refs: Vec<FullName<'input>>,
    pub any_attributes: bool,
}
