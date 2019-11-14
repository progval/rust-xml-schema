
pub mod non_recursive {
    use names::FullName;
    use support::Facets;

    // TODO: Make this use &str so it can implement Copy, and spare clones later in the code
    #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ConcreteName(pub String, pub String);

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ComplexType<'input, TAttrs, TExtra> {
        Any,
        Empty,
        Alias(ConcreteName),
        Extension(ConcreteName, ConcreteName),
        Restriction(ConcreteName, ConcreteName),
        ElementRef(usize, usize, ConcreteName),
        Element(
            usize,
            usize,
            FullName<'input>,
            TAttrs,
            ConcreteName,
        ),
        Choice(usize, usize, Vec<ConcreteName>),
        Sequence(usize, usize, Vec<ConcreteName>),
        Simple(ConcreteName),
        Extra(TExtra),
    }

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SimpleType<'input> {
        Alias(ConcreteName),
        Restriction(ConcreteName, Facets<'input>),
        List(ConcreteName),
        Union(Vec<ConcreteName>),
        Empty,
    }
}

pub mod recursive {
    use names::FullName;
    use support::Facets;

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ComplexType<'input, TAttrs, TExtra> {
        Any,
        Empty,
        Alias(FullName<'input>),
        Extension(FullName<'input>, Box<ComplexType<'input, TAttrs, TExtra>>),
        Restriction(FullName<'input>, Box<ComplexType<'input, TAttrs, TExtra>>),
        ElementRef(usize, usize, FullName<'input>),
        Element(
            usize,
            usize,
            FullName<'input>,
            TAttrs,
            Box<ComplexType<'input, TAttrs, TExtra>>,
        ),
        GroupRef(usize, usize, FullName<'input>),
        Choice(usize, usize, Vec<ComplexType<'input, TAttrs, TExtra>>),
        Sequence(usize, usize, Vec<ComplexType<'input, TAttrs, TExtra>>),
        Extra(TExtra),
        Simple(SimpleType<'input>),
    }

    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SimpleType<'input> {
        Primitive(&'static str, &'static str),
        Alias(FullName<'input>),
        Restriction(FullName<'input>, Facets<'input>),
        List(Box<SimpleType<'input>>),
        Union(Vec<SimpleType<'input>>),
        Empty,
    }
}
