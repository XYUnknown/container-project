use std::vec::Vec;
//use std::cmp::Ordering;

pub struct UniqueVec<T> {
    v: Vec<T>,
}

impl<T: PartialEq> UniqueVec<T> {
    pub const fn new() -> UniqueVec<T> {
        UniqueVec {v: Vec::new()}
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }
    pub fn push(&mut self, value: T) {
        if !self.v.contains(&value) {
            self.v.push(value);
        }
    }
}