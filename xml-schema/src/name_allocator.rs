//! Transforms recursive types into flat types with unique names

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use asts;
use attrs::with_refs::Attrs;
use names::{name_from_hint, FullName, NameGenerator, NameHint};

use asts::non_recursive::ComplexType as NRComplexType;
use asts::non_recursive::ConcreteName;
use asts::non_recursive::SimpleType as NRSimpleType;
use asts::recursive::ComplexType as RComplexType;
use asts::recursive::SimpleType as RSimpleType;

use lift_attrs::OutAttrs as InAttrs;
use lift_attrs::OutComplexType as InComplexType;
use lift_attrs::OutSimpleType as InSimpleType;

pub type OutSimpleType<'input> = asts::non_recursive::SimpleType<'input>;
pub type OutComplexType<'input> =
    asts::non_recursive::ComplexType<'input, OutAttrs<'input>, ComplexTypeExtra<'input>>;
pub type OutAttrs<'input> = Attrs<'input, ConcreteName>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplexTypeExtra<'input> {
    GroupRef(usize, usize, FullName<'input>),
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
    complex_types: HashMap<ConcreteName, OutComplexType<'input>>,
    simple_types: HashMap<ConcreteName, OutSimpleType<'input>>,
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
        recursive_complex_type: &InComplexType<'input>,
    ) -> ConcreteName {
        let (concrete_name, ty) = match recursive_complex_type {
            RComplexType::Any => (
                self.allocate_anonymous(namespace, "any"),
                NRComplexType::Any,
            ),
            RComplexType::Empty => (
                self.allocate_anonymous(namespace, "empty"),
                NRComplexType::Empty,
            ),
            RComplexType::Alias(fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (referee.clone(), NRComplexType::Alias(referee))
            }
            RComplexType::Extension(base, inner) => {
                let base = self.allocate_fullname(*base);
                let inner = self.allocate_complex_type(namespace, inner);
                (
                    self.allocate_anonymous_compound(namespace, "extension", &[&base, &inner]),
                    NRComplexType::Extension(base, inner),
                )
            }
            RComplexType::Restriction(base, inner) => {
                let base = self.allocate_fullname(*base);
                let inner = self.allocate_complex_type(namespace, inner);
                (
                    self.allocate_anonymous_compound(namespace, "restriction", &[&base, &inner]),
                    NRComplexType::Restriction(base, inner),
                )
            }
            RComplexType::ElementRef(min_occurs, max_occurs, fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (
                    self.allocate_anonymous_compound(namespace, "elementref", &[&referee]),
                    NRComplexType::ElementRef(*min_occurs, *max_occurs, referee),
                )
            }
            RComplexType::Element(min_occurs, max_occurs, fullname, attrs, inner) => {
                let inner = self.allocate_complex_type(namespace, inner);
                let attrs = self.allocate_attrs(namespace, attrs);
                (
                    self.allocate_fullname(*fullname),
                    NRComplexType::Element(*min_occurs, *max_occurs, *fullname, attrs, inner),
                )
            }
            RComplexType::GroupRef(min_occurs, max_occurs, fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (
                    self.allocate_anonymous_compound(namespace, "groupref", &[&referee]),
                    NRComplexType::ElementRef(*min_occurs, *max_occurs, referee),
                )
            }
            RComplexType::Choice(min_occurs, max_occurs, inners) => {
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
                    NRComplexType::Choice(*min_occurs, *max_occurs, inners),
                )
            }
            RComplexType::Sequence(min_occurs, max_occurs, inners) => {
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
                    NRComplexType::Sequence(*min_occurs, *max_occurs, inners),
                )
            }
            RComplexType::Simple(inner) => {
                let inner = self.allocate_simple_type(namespace, inner);
                (inner.clone(), NRComplexType::Simple(inner))
            }
            RComplexType::Extra(_) => unreachable!("It's the bottom type!"),
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
        recursive_simple_type: &InSimpleType<'input>,
    ) -> ConcreteName {
        let (concrete_name, ty) = match recursive_simple_type {
            RSimpleType::Primitive(mod_name, type_name) => {
                let concrete_name = ConcreteName(mod_name.to_string(), type_name.to_string());
                (concrete_name.clone(), NRSimpleType::Alias(concrete_name))
            }
            RSimpleType::Alias(fullname) => {
                let referee = self.allocate_fullname(*fullname);
                (referee.clone(), NRSimpleType::Alias(referee))
            }
            RSimpleType::Restriction(base, facets) => {
                let base = self.allocate_fullname(*base);
                (
                    self.allocate_anonymous_compound(namespace, "simplerestriction", &[&base]),
                    NRSimpleType::Restriction(base, facets.clone()),
                )
            }
            RSimpleType::Union(inners) => {
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
                    NRSimpleType::Union(inners),
                )
            }
            RSimpleType::List(inner) => {
                let inner = self.allocate_simple_type(namespace, inner);
                (
                    self.allocate_anonymous_compound(namespace, "list", &[&inner]),
                    NRSimpleType::List(inner),
                )
            }
            RSimpleType::Empty => (
                self.allocate_anonymous(namespace, "empty"),
                NRSimpleType::Empty,
            ),
        };

        let entry = self.simple_types.entry(concrete_name.clone());
        if let Entry::Occupied(_) = entry {
            panic!("Duplicate name {:?}", concrete_name)
        }
        entry.or_insert(ty);
        concrete_name
    }

    fn allocate_attrs(
        &mut self,
        _namespace: Option<&'input str>,
        _attrs: &InAttrs<'input>,
    ) -> OutAttrs<'input> {
        unimplemented!()
    }
}
