use std::vec::Vec;
use std::iter::Iterator;
use core::slice::SliceIndex;
//use std::cmp::Ordering;


// A Unique Vector
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

    // Duplicated push will be discarded
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

    // Duplicated insertion will be discarded
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

    /* TODO fix this
    pub fn append(&mut self, other: &mut UniqueVec<T>) 
    where
        T: PartialEq<T>,
        {
            for val in other.content().iter() {
                self.v.push(val);
            }
        }
    */

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
impl<T> std::ops::Deref for UniqueVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

// A Sorted Vector
pub struct SortedVec<T> {
    v: Vec<T>
}

impl<T> SortedVec<T> {
    pub const fn new() -> SortedVec<T> {
        SortedVec { v: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> SortedVec<T> {
        SortedVec { v: Vec::with_capacity(capacity) }
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
}