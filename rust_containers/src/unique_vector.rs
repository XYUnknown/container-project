use std::vec::Vec;
use std::slice::Iter;
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
    
    pub fn iter(&self) -> Iter<'_, T> {
        self.v.iter()
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

#[macro_export]
// UniqueVec creations
macro_rules! unique_vec { // e.g., unique_vec![1, 2, 3]
    ($($x:expr),*) => {
        {
            let mut vec = UniqueVec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
    ($elem:expr; $n:expr) => { // e.g., unique_vec![1; 3]
        {
            let mut vec = UniqueVec::new();
            vec.push($elem);
            vec
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::unique_vector::UniqueVec;
    /** Unique Vector*/
    #[test]
    fn unique_vec_creation_works() {
        let vec = UniqueVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_vec_creation_with_capacity_works() {
        let vec = UniqueVec::<u32>::with_capacity(10);
        assert_eq!(vec.capacity(), 10);
    }

    #[test]
    fn unique_vec_creation_from_vec() {
        let mut src = vec![1, 2, 3];
        let vec = UniqueVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3]);
    }

    #[test]
    fn unique_vec_contains_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        assert_eq!(vec.contains(&1), true);
        assert_eq!(vec.contains(&2), false);
    }

    #[test]
    fn unique_vec_push_works() {
        let mut vec = UniqueVec::new();
        for x in 0..10000 {
            vec.push(x);
            vec.push(x);
        }
        assert_eq!(vec.len(), 10000); // no duplication
    }

    #[test]
    fn unique_vec_pop_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn unique_vec_pop_some_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
    }

    #[test]
    fn unique_vec_insert_works() {
        let mut vec = UniqueVec::new();
        vec.insert(0, 10);
        vec.insert(1, 10);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_remove_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.remove(0), 1);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_truncate_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        vec.truncate(3);
        assert_eq!(*vec, [0, 1, 2]);
    }

    #[test]
    fn unique_vec_clear_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_vec_first_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.first(), None);
    }

    #[test]
    fn unique_vec_first_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.first(), Some(&0));
    }

    #[test]
    fn unique_vec_last_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.last(), None);
    }

    #[test]
    fn unique_vec_last_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.last(), Some(&4));
    }

    #[test]
    fn unique_vec_get_element_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.get(3), Some(&3));
    }

    #[test]
    fn unique_vec_get_slice_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.get(0..2), Some(&[0, 1][..]));
    }

    #[test]
    fn unique_vec_macro_one_works() {
        let mut vec = unique_vec![1, 1];
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_macro_two_works() {
        let mut vec = unique_vec![1; 4];
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_append_works() {
        let mut vec = UniqueVec::new();
        let mut other = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5, 6])
    }
}