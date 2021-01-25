use std::vec::Vec;
use std::ops::Deref;
use std::cmp::Ordering;
use core::slice::SliceIndex;
use crate::sorted_vector::SortedVec;

// A Unique and sorted vector
pub struct UniqueSortedVec<T> {
    v: SortedVec<T>,
}

impl<T: Ord + PartialEq> UniqueSortedVec<T> {
    pub const fn new() -> UniqueSortedVec<T> {
        UniqueSortedVec { v: SortedVec::new() }
    }

    pub fn with_capacity(capacity: usize) -> UniqueSortedVec<T> {
        UniqueSortedVec { v: SortedVec::with_capacity(capacity) }
    }

    pub fn from_vec(src: &mut Vec<T>) -> UniqueSortedVec<T> 
    where
        T: Clone
        {
            src.sort();
            src.dedup(); // remove repeated elemnets
            UniqueSortedVec { v: SortedVec::from_vec(src) }
        }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn capacity(&self) -> usize {
        self.v.capacity()
    }

    pub fn contains(&self, x: &T) -> bool{
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

    pub fn pop(&mut self) {
        self.v.pop();
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
    
    // Append and then remove consecutive repeated elements
    pub fn append(&mut self, other: &mut Self) {
        self.v.append(&mut other.v);
        self.v.dedup();
    }
}

// Accessing the single field of a UniqueSortedVec
impl<T> Deref for UniqueSortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v.deref()
    }
}

impl<T: Clone> Clone for UniqueSortedVec<T> {
    fn clone(&self) -> Self {
        UniqueSortedVec { v: self.v.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.v.clone_from(&source.v);
    }
}