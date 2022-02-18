use std::collections::HashMap;
use std::collections::hash_map::Iter;

use crate::parser::{Id};
use crate::types::{Type};

pub type Description = String;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Tag {
    Prop(Box<Description>), // analysis of a property
    Interface(Box<Vec<Description>>),
    Con(Box<Vec<Tag>>) // analysis of a container type with refinements
}

impl Tag {
    pub fn is_prop_tag(&self) -> bool {
        match self {
            Tag::Prop(_) => true,
            _ => false
        }
    }

    pub fn is_interface_tag(&self) -> bool {
        match self {
            Tag::Interface(_) => true,
            _ => false
        }
    }

    pub fn extract_desc(&self) -> Description {
        match self {
            Tag::Prop(desc) => desc.to_string(),
            _ => String::new()
        }
    }

    pub fn extract_descs(&self) -> Vec<Description> {
        match self {
            Tag::Interface(descs) => descs.to_vec(),
            _ => Vec::new()
        }
    }
}

#[derive(Clone, Debug)]
pub struct InforMap {
    infor_map: HashMap<Id, Tag>,
}

impl InforMap {
    pub fn new() -> InforMap {
        InforMap {
            infor_map: HashMap::new(),
        }
    }

    pub fn put(&mut self, id: Id, tag: Tag) -> bool {
        if self.infor_map.contains_key(&id) {
            false
        } else {
            self.infor_map.insert(id, tag);
            true
        }
    }

    pub fn get_id(&self, id: Id) -> Option<&Tag> {
        self.infor_map.get(&id)
    }

    pub fn iter(&self) -> Iter<'_, Id, Tag> {
        self.infor_map.iter()
    }

    fn sz(&self) -> usize {
        self.infor_map.len()
    }
}