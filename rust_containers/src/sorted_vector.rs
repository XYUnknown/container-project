use std::vec::Vec;
use std::ops::Deref;
use std::cmp::Ordering;
use core::slice::SliceIndex;

// A Sorted Vector
pub struct SortedVec<T> {
    v: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub const fn new() -> SortedVec<T> {
        SortedVec { v: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> SortedVec<T> {
        SortedVec { v: Vec::with_capacity(capacity) }
    }

    pub fn from_vec(src: &mut Vec<T>) -> SortedVec<T> 
    where
        T: Clone
        {
            src.sort();
            let vec: Vec<T> = src.to_vec();
            SortedVec { v: vec }
        }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn capacity(&self) -> usize {
        self.v.capacity()
    }

    /** 
     * Use binary search to check if a given element is in the sorted vector
     * O(log n)
     */
    pub fn contains(&self, x: &T) -> bool{
        match self.v.binary_search(x) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn push(&mut self, x: T) {
        let index = self.v.binary_search(&x).unwrap_or_else(|i| i);
        self.v.insert(index, x);
    }

    // By default get the maximum element
    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    // It is not very meaningful to remove by index
    // given the index does not maintain the insertion order
    // but can be mainful for operations like looking for the median
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

    // Merge them into one vector
    pub fn append(&mut self, other: &mut Self) {
        let mut temp = Vec::new();
        loop {
            match self.first() {
                Some(x1) => {
                    match other.first() {
                        Some(x2) => {
                            match x1.cmp(x2) {
                                Ordering::Less => temp.push(self.remove(0)),
                                Ordering::Greater => temp.push(other.remove(0)),
                                Ordering::Equal => temp.push(self.remove(0)),
                            }
                        },
                        None => {
                            temp.append(&mut self.v);
                            break;
                        }
                    }
                },
                None => {
                    temp.append(&mut other.v);
                    break;
                }
            }
        }
        self.v = temp.drain(..).collect();
    }
}

// Accessing the single field of a SortedVec
impl<T> Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T: Clone> Clone for SortedVec<T> {
    fn clone(&self) -> Self {
        SortedVec { v: self.v.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.v.clone_from(&source.v);
    }
}