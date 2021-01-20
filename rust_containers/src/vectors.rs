use std::vec::Vec;
use core::slice::SliceIndex;
//use std::cmp::Ordering;

pub struct UniqueVec<T> {
    v: Vec<T>,
}

impl<T> UniqueVec<T> {
    pub const fn new() -> UniqueVec<T> {
        UniqueVec { v: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> UniqueVec<T> {
        UniqueVec { v: Vec::with_capacity(capacity) }
    }

    pub fn content(&self) -> &Vec<T> {
        &self.v
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn capacity(&self) -> usize {
        self.v.capacity()
    }

    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>, 
        {
            self.v.contains(x)
        }

    /**
     * Modifying the vector
     */
    pub fn push(&mut self, value: T) 
    where
        T: PartialEq<T>, 
        {
            if !self.contains(&value) {
                self.v.push(value);
            }
        }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    pub fn insert(&mut self, index: usize, element: T) 
    where
        T: PartialEq<T>, 
        {
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