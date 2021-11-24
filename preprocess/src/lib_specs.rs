use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::vec::Vec;

use crate::parser::{Description};

/* A temporary lib specification*/
const HASHSET: &str = "std::collections::HashSet<T>";
const BTREESET: &str = "std::collections::BTreeSet<T>";
const VEC: &str = "std::vec::Vec<T>";

type StructName = String;

#[derive(Clone, Debug)]
pub struct LibSpec {
    lib_spec: HashMap<StructName, Vec<Description>>,
}

impl LibSpec {
    pub fn new() -> LibSpec {
        LibSpec {
            lib_spec: HashMap::new(),
        }
    }

    pub fn get(&self, name: &StructName) -> Option<&Vec<Description>> {
        self.lib_spec.get(name)
    }

    pub fn get_mut(&mut self, name: &StructName) -> Option<&mut Vec<Description>> {
        self.lib_spec.get_mut(name)
    }

    pub fn put(&mut self, name: StructName, descs: Vec<Description>) {
        self.lib_spec.insert(name, descs);
    }

    pub fn put_desc(&mut self, name: StructName, desc: Description) {
        match self.get_mut(&name) {
            Some(descs) => descs.push(desc),
            None => self.put(name, vec![desc])
        }
    }

    pub fn contains(&self, name: &StructName, desc: &Description) -> bool {
        match self.get(&name) {
            Some(descs) => descs.contains(desc),
            None => false
        }
    }

    pub fn contains_descs(&self, name: &StructName, descs: &Vec<Description>) -> bool {
        match self.get(&name) {
            Some(lib_descs) => {
                let mut result = true;
                for decl in descs.iter() {
                    result = lib_descs.contains(decl);
                    if !result {
                        return result;
                    }
                }
                result
            }
            None => false
        }
    }

    pub fn iter(&self) -> Iter<'_, StructName, Vec<Description>> {
        self.lib_spec.iter()
    }
}

pub fn construct_spec() -> LibSpec {
    let mut spec = LibSpec::new();
    spec.put(HASHSET.to_string(), vec!["unique".to_string()]);
    spec.put(BTREESET.to_string(), vec!["unique".to_string(), "ascending".to_string()]);
    spec.put(VEC.to_string(), vec![]);
    spec
}