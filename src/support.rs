#[derive(Debug, PartialEq, Default)]
pub struct token<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct NMTOKEN<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct QName<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct anyURI<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct nonNegativeInteger<'input>(&'input str);

#[derive(Debug, PartialEq, Default)]
pub struct SUPPORT_ANY<'input>(&'input str);
