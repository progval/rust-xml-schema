//! Transforms recursive types into flat types with unique names

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use names::{name_from_hint, FullName, NameGenerator, NameHint};
use processor2::{RecursiveComplexType, RecursiveSimpleType};
use support::Facets;

// TODO: Make this use &str so it can implement Copy, and spare clones later in the code
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConcreteName(String, String);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplexType<'input> {
    Any,
    Empty,
    Alias(ConcreteName),
    Extension(ConcreteName, ConcreteName),
    Restriction(ConcreteName, ConcreteName),
    ElementRef(usize, usize, ConcreteName),
    Element(usize, usize, FullName<'input>, ConcreteName),
    GroupRef(usize, usize, ConcreteName),
    Choice(usize, usize, Vec<ConcreteName>),
    Sequence(usize, usize, Vec<ConcreteName>),
    Simple(ConcreteName),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleType<'input> {
    Alias(ConcreteName),
    Restriction(ConcreteName, Facets<'input>),
    List(ConcreteName),
    Union(Vec<ConcreteName>),
    Empty,
}

fn allocate_namespace<'a, 'input>(
    module_name_gen: &'a mut NameGenerator,
    module_names: &'a mut HashMap<Option<&'input str>, (String, NameGenerator)>,
    namespace: Option<&'input str>,
) -> (String, &'a mut NameGenerator) {
    let (ref mod_name, ref mut name_gen) = module_names.entry(namespace).or_insert_with(|| {
        let mod_name = module_name_gen.gen_name(namespace.unwrap_or("unqualified").to_string());
        (mod_name, NameGenerator::new())
    });
    (mod_name.to_string(), name_gen)
}

#[derive(Debug)]
pub struct NameAllocator<'input> {
    module_name_gen: NameGenerator,
    module_names: HashMap<Option<&'input str>, (String, NameGenerator)>, // namespace -> (mod_name, name_gen)
    fullname_to_concrete_name: HashMap<FullName<'input>, ConcreteName>,
    complex_types: HashMap<ConcreteName, ComplexType<'input>>,
    simple_types: HashMap<ConcreteName, SimpleType<'input>>,
}

impl<'input> NameAllocator<'input> {
    pub fn new() -> NameAllocator<'input> {
        NameAllocator {
            module_name_gen: NameGenerator::new(),
            module_names: HashMap::new(),
            fullname_to_concrete_name: HashMap::new(),
            complex_types: HashMap::new(),
            simple_types: HashMap::new(),
        }
    }

    pub fn allocate_fullname(&mut self, fullname: FullName<'input>) -> ConcreteName {
        let NameAllocator {
            ref mut module_name_gen,
            ref mut module_names,
            ..
        } = self;
        let concrete_name = self
            .fullname_to_concrete_name
            .entry(fullname)
            .or_insert_with(|| {
                let (module_name, name_gen) =
                    allocate_namespace(module_name_gen, module_names, fullname.namespace());
                let type_name = name_gen.gen_name(fullname.local_name().to_string());
                ConcreteName(module_name, type_name)
            });
        concrete_name.clone()
    }

    /// Allocates names for anonymous types not made of other types
    fn allocate_anonymous(&mut self, namespace: Option<&'input str>, name: &str) -> ConcreteName {
        let name_hint = NameHint::new(name);
        let (module_name, name_gen) =
            allocate_namespace(&mut self.module_name_gen, &mut self.module_names, namespace);
        let type_name = name_gen.gen_name(name_from_hint(&name_hint).unwrap());
        ConcreteName(module_name, type_name)
    }

    /// Allocates names for anonymous types made of other types (possibly
    /// anonymous themselves)
    fn allocate_anonymous_compound(
        &mut self,
        namespace: Option<&'input str>,
        prefix: &str,
        subtypes: &[&ConcreteName],
    ) -> ConcreteName {
        let mut name_hint = NameHint::new(prefix);
        for ConcreteName(_subtype_mod_name, subtype_type_name) in subtypes.iter() {
            name_hint.push(subtype_type_name);
        }
        let (module_name, name_gen) =
            allocate_namespace(&mut self.module_name_gen, &mut self.module_names, namespace);
        let type_name = name_gen.gen_name(name_from_hint(&name_hint).unwrap());
        ConcreteName(module_name, type_name)
    }

    pub fn allocate_complex_type(
        &mut self,
        namespace: Option<&'input str>,
        recursive_complex_type: &RecursiveComplexType<'input>,
    ) -> ConcreteName {
        let (concrete_name, ty) = match recursive_complex_type {
            RecursiveComplexType::Any => {
                (self.allocate_anonymous(namespace, "any"), ComplexType::Any)
            }
            RecursiveComplexType::Empty => (
                self.allocate_anonymous(namespace, "empty"),
                ComplexType::Empty,
            ),
            RecursiveComplexType::Alias(fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (referee.clone(), ComplexType::Alias(referee))
            }
            RecursiveComplexType::Extension(base, inner) => {
                let base = self.allocate_fullname(*base);
                let inner = self.allocate_complex_type(namespace, inner);
                (
                    self.allocate_anonymous_compound(namespace, "extension", &[&base, &inner]),
                    ComplexType::Extension(base, inner),
                )
            }
            RecursiveComplexType::Restriction(base, inner) => {
                let base = self.allocate_fullname(*base);
                let inner = self.allocate_complex_type(namespace, inner);
                (
                    self.allocate_anonymous_compound(namespace, "restriction", &[&base, &inner]),
                    ComplexType::Restriction(base, inner),
                )
            }
            RecursiveComplexType::ElementRef(min_occurs, max_occurs, fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (
                    self.allocate_anonymous_compound(namespace, "elementref", &[&referee]),
                    ComplexType::ElementRef(*min_occurs, *max_occurs, referee),
                )
            }
            RecursiveComplexType::Element(min_occurs, max_occurs, fullname, inner) => {
                let inner = self.allocate_complex_type(namespace, inner);
                (
                    self.allocate_fullname(*fullname),
                    ComplexType::Element(*min_occurs, *max_occurs, *fullname, inner),
                )
            }
            RecursiveComplexType::GroupRef(min_occurs, max_occurs, fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (
                    self.allocate_anonymous_compound(namespace, "groupref", &[&referee]),
                    ComplexType::ElementRef(*min_occurs, *max_occurs, referee),
                )
            }
            RecursiveComplexType::Choice(min_occurs, max_occurs, inners) => {
                let inners: Vec<_> = inners
                    .iter()
                    .map(|inner| self.allocate_complex_type(namespace, inner))
                    .collect();
                (
                    self.allocate_anonymous_compound(
                        namespace,
                        "choice",
                        &inners.iter().collect::<Vec<_>>(),
                    ),
                    ComplexType::Choice(*min_occurs, *max_occurs, inners),
                )
            }
            RecursiveComplexType::Sequence(min_occurs, max_occurs, inners) => {
                let inners: Vec<_> = inners
                    .iter()
                    .map(|inner| self.allocate_complex_type(namespace, inner))
                    .collect();
                (
                    self.allocate_anonymous_compound(
                        namespace,
                        "sequence",
                        &inners.iter().collect::<Vec<_>>(),
                    ),
                    ComplexType::Sequence(*min_occurs, *max_occurs, inners),
                )
            }
            RecursiveComplexType::Simple(inner) => {
                let inner = self.allocate_simple_type(namespace, inner);
                (inner.clone(), ComplexType::Simple(inner))
            }
        };
        let entry = self.complex_types.entry(concrete_name.clone());
        if let Entry::Occupied(_) = entry {
            panic!("Duplicate name {:?}", concrete_name)
        }
        entry.or_insert(ty);
        concrete_name
    }

    pub fn allocate_simple_type(
        &mut self,
        namespace: Option<&'input str>,
        recursive_simple_type: &RecursiveSimpleType<'input>,
    ) -> ConcreteName {
        let (concrete_name, ty) = match recursive_simple_type {
            RecursiveSimpleType::Primitive(mod_name, type_name) => {
                let concrete_name = ConcreteName(mod_name.to_string(), type_name.to_string());
                (concrete_name.clone(), SimpleType::Alias(concrete_name))
            }
            RecursiveSimpleType::Alias(fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (referee.clone(), SimpleType::Alias(referee))
            }
            RecursiveSimpleType::Restriction(base, facets) => {
                let base = self.allocate_fullname(*base);
                (
                    self.allocate_anonymous_compound(namespace, "simplerestriction", &[&base]),
                    SimpleType::Restriction(base, facets.clone()),
                )
            }
            RecursiveSimpleType::Union(inners) => {
                let inners: Vec<_> = inners
                    .iter()
                    .map(|inner| self.allocate_simple_type(namespace, inner))
                    .collect();
                (
                    self.allocate_anonymous_compound(
                        namespace,
                        "union",
                        &inners.iter().collect::<Vec<_>>(),
                    ),
                    SimpleType::Union(inners),
                )
            }
            RecursiveSimpleType::List(inner) => {
                let inner = self.allocate_simple_type(namespace, inner);
                (
                    self.allocate_anonymous_compound(
                        namespace,
                        "list",
                        &[&inner],
                    ),
                    SimpleType::List(inner),
                )
            }
            RecursiveSimpleType::Empty => (
                self.allocate_anonymous(namespace, "empty"),
                SimpleType::Empty,
            ),
        };

        let entry = self.simple_types.entry(concrete_name.clone());
        if let Entry::Occupied(_) = entry {
            panic!("Duplicate name {:?}", concrete_name)
        }
        entry.or_insert(ty);
        concrete_name
    }
}
