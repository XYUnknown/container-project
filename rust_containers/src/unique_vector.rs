use std::vec::Vec;
use std::ops::Deref;
use core::slice::SliceIndex;

// A Unique Vector
pub struct UniqueVec<T> {
    v: Vec<T>,
}

impl<T: PartialEq> UniqueVec<T> {
    pub const fn new() -> UniqueVec<T> {
        UniqueVec { v: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> UniqueVec<T> {
        UniqueVec { v: Vec::with_capacity(capacity) }
    }

    pub fn from_vec(src: &mut Vec<T>) -> UniqueVec<T> {
        let mut vec = Vec::new();
        while !src.is_empty() {
            let x = src.remove(0);
            if !vec.contains(&x) {
                vec.push(x);
            }
        }
        UniqueVec { v: vec }
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn capacity(&self) -> usize {
        self.v.capacity()
    }

    pub fn contains(&self, x: &T) -> bool {
        self.v.contains(x)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /**
     * Modifying the vector
     */

    // Duplicated elements will be discarded
    pub fn push(&mut self, value: T) {
        if !self.contains(&value) {
            self.v.push(value);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    // Duplicated elements will be discarded
    pub fn insert(&mut self, index: usize, element: T) {
        if !self.contains(&element) {
            self.v.insert(index, element);
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.v.remove(index)
    }

    pub fn truncate(&mut self, len: usize) {
        self.v.truncate(len);
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    // TODO: make this faster
    pub fn append(&mut self, other: &mut Self) {
        while !other.is_empty() {
            self.push(other.remove(0)); // preserve ordering
        }
    }

    /**
     * Accessing elements
     */
    pub fn first(&mut self) -> Option<&T> {
        self.v.first()
    }

    pub fn last(&mut self) -> Option<&T> {
        self.v.last()
    }

    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
        {
            self.v.get(index)
        }
}

// Accessing the single field of a UniqueVec
impl<T> Deref for UniqueVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T: Clone> Clone for UniqueVec<T> {
    fn clone(&self) -> Self {
        UniqueVec { v: self.v.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.v.clone_from(&source.v);
    }
}
