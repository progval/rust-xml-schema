use std::collections::HashMap;

use support::QName;

const KEYWORDS: &[&'static str] = &["override"];
fn escape_keyword(name: &str) -> String {
    if KEYWORDS.contains(&name) {
        format!("{}_", name)
    }
    else {
        name.to_string()
    }
}

pub(crate) struct NameGenerator(HashMap<String, usize>);

impl NameGenerator {
    pub fn new() -> NameGenerator {
        NameGenerator(HashMap::new())
    }

    pub fn gen_name(&mut self, name: String) -> String {
        let nb_uses = self.0.get(&name).cloned().unwrap_or(0);
        let ret = format!("{}{}", name, "_".repeat(nb_uses));
        self.0.insert(name, nb_uses+1);
        ret
    }
}

/*
macro_rules! str_alias {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name<'input>(&'input str);
        impl<'input> $name<'input> {
            pub fn new(s: &'input str) -> $name<'input> {
                $name(s)
            }
            pub fn as_str(&self) -> &'input str {
                self.0
            }
        }
    }
}
*/

#[derive(Debug)]
pub(crate) struct Namespaces<'input> {
    pub target_namespace: &'input str,
    pub namespaces: HashMap<&'input str, &'input str>, // namespace -> URI
    pub module_names: HashMap<&'input str, &'input str>, // URI -> module name
}

impl<'input> Namespaces<'input> {
    pub fn new(mut namespaces: HashMap<&'input str, &'input str>, target_namespace: &'input str) -> Namespaces<'input> {
        if let Some(uri) = namespaces.insert("xml", "xml") {
            panic!("Cannot have a namespaces named \"xml\": {}", uri);
        }
        if let Some(uri) = namespaces.insert("xmlns", "xmlns") {
            panic!("Cannot have a namespaces named \"xmlns\": {}", uri);
        }
        let mut module_names = HashMap::new();
        for (ns, uri) in namespaces.iter() {
            module_names.insert(*uri, *ns);
        }
        Namespaces {
            target_namespace,
            namespaces,
            module_names,
        }
    }

    pub fn expand_prefix(&self, prefix: Option<&'input str>) -> &'input str {
        match prefix {
            Some(prefix) => self.namespaces.get(prefix).expect(&format!("Unknown prefix: {:?}", prefix)),
            None => self.target_namespace,
        }
    }
    pub fn expand_qname(&self, qname: QName<'input>) -> FullName<'input> {
        FullName::new(self.expand_prefix(qname.0), qname.1)
    }
    pub fn parse_qname(&self, s: &'input str) -> FullName<'input> {
        self.expand_qname(QName::from(s))
    }
    pub fn qname_eq(&self, qname1: QName<'input>, qname2: QName<'input>) -> bool {
        qname1.1 == qname2.1 && self.expand_prefix(qname1.0) == self.expand_prefix(qname2.0)
    }

    pub fn get_module_name(&self, qname: FullName<'input>) -> &'input str {
        let (prefix, _) = qname.as_tuple();
        self.module_names.get(prefix).cloned().unwrap_or("UNQUAL")
    }

    pub fn name_from_hint(&self, hint: &NameHint<'input>) -> Option<String> {
        if hint.tokens.len() > 0 {
            Some(hint.tokens.iter().map(|&s| escape_keyword(s)).collect::<Vec<_>>().join("_"))
        }
        else {
            None
        }
    }
    pub fn name_from_hints<'a: 'input, T>(&self, hints: T) -> Option<String>
            where T: Iterator<Item=&'a NameHint<'a>>{
        let mut candidates = Vec::new();
        for hint in hints {
            if let Some(candidate) = self.name_from_hint(&hint) {
                candidates.push(candidate);
            }
        }
        if candidates.len() > 0 {
            candidates.sort_by_key(|c| c.len());
            Some(candidates.swap_remove(0))
        }
        else {
            None
        }
    }
    pub fn modules(&self) -> impl Iterator<Item=(&&'input str, &&'input str)> {
        self.namespaces.iter()
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FullName<'input>(&'input str, &'input str);

impl<'input> FullName<'input> {
    pub fn new(ns: &'input str, name: &'input str) -> FullName<'input> {
        FullName(ns, name)
    }
    pub fn as_tuple(&self) -> (&'input str, &'input str) {
        (self.0, self.1)
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
