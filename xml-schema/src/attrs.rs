#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttrUse {
    Prohibited,
    Required,
    Optional,
}

pub mod with_refs {
    use names::FullName;

    use super::AttrUse;

    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Attrs<'input, TSimpleType: Clone> {
        pub named: Vec<(FullName<'input>, AttrUse, Option<TSimpleType>)>,
        pub refs: Vec<(Option<FullName<'input>>, AttrUse, FullName<'input>)>,
        pub group_refs: Vec<FullName<'input>>,
        pub any_attributes: bool,
    }

    impl<'input, TSimpleType> Attrs<'input, TSimpleType> where TSimpleType: Clone {
        pub fn new() -> Attrs<'input, TSimpleType> {
            Attrs {
                named: Vec::new(),
                refs: Vec::new(),
                group_refs: Vec::new(),
                any_attributes: false,
            }
        }

        pub fn extend(&mut self, other: Attrs<'input, TSimpleType>) {
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
}
