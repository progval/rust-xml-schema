use std::collections::HashMap;
use std::fmt;

use primitives::QName;

const KEYWORDS: &[&'static str] = &["override"];
fn escape_keyword(name: &str) -> String {
    if KEYWORDS.contains(&name) {
        format!("{}_", name)
    }
    else {
        name.to_string()
    }
}

#[derive(Debug, Default)]
pub(crate) struct NameGenerator(HashMap<String, usize>);

impl NameGenerator {
    pub fn new() -> NameGenerator {
        NameGenerator(HashMap::new())
    }

    pub fn gen_name(&mut self, name: String) -> String {
        let nb_uses = self.0.get(&name).cloned().unwrap_or(1);
        let ret = if nb_uses > 1 {
            format!("{}{}", name, nb_uses)
        }
        else {
            name.to_string()
        };
        self.0.insert(name, nb_uses+1);
        ret
    }
}

pub fn name_from_hint<'input>(hint: &NameHint<'input>) -> Option<String> {
    if hint.tokens.len() > 0 {
        Some(hint.tokens.iter().map(|&s| escape_keyword(s)).collect::<Vec<_>>().join("_"))
    }
    else {
        None
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FullName<'input>(Option<&'input str>, &'input str);

impl<'input> FullName<'input> {
    pub fn new(ns: Option<&'input str>, name: &'input str) -> FullName<'input> {
        FullName(ns, name)
    }
    pub fn namespace(&self) -> Option<&'input str> {
        self.0
    }
    pub fn local_name(&self) -> &'input str {
        self.1
    }
}

impl<'input> FullName<'input> {
    pub fn from_qname(qn: &QName<'input>, default_namespace: Option<&'input str>) -> FullName<'input> {
        FullName(qn.namespace.or(default_namespace), qn.local_name)
    }
}

impl<'input> fmt::Display for FullName<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(prefix) => write!(f, "{}:{}", prefix, self.1),
            None => write!(f, "{}", self.1),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameHint<'input> {
    tokens: Vec<&'input str>,
}
impl<'input> NameHint<'input> {
    pub fn new_empty() -> NameHint<'input> {
        NameHint { tokens: Vec::new() }
    }
    pub fn new(s: &'input str) -> NameHint<'input> {
        NameHint { tokens: vec![s] }
    }
    pub fn from_fullname(name: &FullName<'input>) -> NameHint<'input> {
        NameHint::new(name.1)
    }
    pub fn push(&mut self, s: &'input str) {
        self.tokens.push(s);
    }
    pub fn extend(&mut self, other: &NameHint<'input>) {
        self.tokens.extend(other.tokens.iter())
    }
}
