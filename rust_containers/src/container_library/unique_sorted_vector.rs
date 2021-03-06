use std::vec::Vec;
use std::slice::Iter;
use std::ops::Deref;
use std::cmp::Ordering;
use core::slice::SliceIndex;
use crate::container_library::sorted_vector::SortedVec;
use crate::container_library::unique_vector::UniqueVec;

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

    // Append and then remove consecutive repeated elements
    pub fn append(&mut self, other: &mut Self) {
        self.v.append(&mut other.v);
        self.v.dedup();
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

// An alternative implementation of unique and sorted vector
pub struct UniqueSortedVecAlt<T> {
    v: UniqueVec<T>,
}

impl<T: Ord + PartialEq> UniqueSortedVecAlt<T> {
    pub const fn new() -> UniqueSortedVecAlt<T> {
        UniqueSortedVecAlt { v: UniqueVec::new() }
    }

    pub fn with_capacity(capacity: usize) -> UniqueSortedVecAlt<T> {
        UniqueSortedVecAlt { v: UniqueVec::with_capacity(capacity) }
    }

    pub fn from_vec(src: &mut Vec<T>) -> UniqueSortedVecAlt<T> 
    where
        T: Clone
        {
            src.sort();
            UniqueSortedVecAlt { v: UniqueVec::from_vec(src) }
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
        let index = self.v.binary_search(&value).unwrap_or_else(|i| i);
        self.v.insert(index, value);
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

    // Merge them into one vector
    pub fn append(&mut self, other: &mut Self) {
        let mut temp = UniqueVec::new();
        loop {
            match self.first() {
                Some(x1) => {
                    match other.first() {
                        Some(x2) => {
                            match x1.cmp(x2) {
                                Ordering::Less => temp.push(self.v.remove(0)),
                                Ordering::Greater => temp.push(other.v.remove(0)),
                                Ordering::Equal => {
                                    temp.push(self.v.remove(0));
                                    other.v.remove(0);
                                }
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
        self.v = temp;
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

// Accessing the single field of a UniqueSortedVec
impl<T> Deref for UniqueSortedVecAlt<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v.deref()
    }
}

impl<T: Clone> Clone for UniqueSortedVecAlt<T> {
    fn clone(&self) -> Self {
        UniqueSortedVecAlt { v: self.v.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.v.clone_from(&source.v);
    }
}

#[macro_export]
// UniqueSortedVec creations
macro_rules! unique_sorted_vec {
    ($($x:expr),*) => { // e.g., unique_sorted_vec![1, 2, 3]
        {
            let mut vec = UniqueSortedVec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
    ($elem:expr; $n:expr) => { // e.g., unique_sorted_vec![1; 3]
        {
            let mut vec = UniqueSortedVec::new();
            vec.push($elem);
            vec
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::container_library::unique_sorted_vector::UniqueSortedVec;
    use crate::container_library::unique_sorted_vector::UniqueSortedVecAlt;

    /* Unique Sorted Vector */
    #[test]
    fn unique_sorted_vec_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let vec = UniqueSortedVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3]);
    }

    #[test]
    fn unique_sorted_vec_macro_one_works() {
        let vec = unique_sorted_vec![3, 7, 2, 1, 5, 4, 3];
        assert_eq!(*vec, [1, 2, 3, 4, 5, 7])
    }

    #[test]
    fn unique_sorted_vec_macro_two_works() {
        let vec = unique_sorted_vec![1; 3];
        assert_eq!(*vec, [1])
    }

    #[test]
    fn unique_sorted_vec_push_works() {
        let mut src = vec![0, 1, 2, 3, 4];
        let mut vec = UniqueSortedVec::from_vec(&mut src);
        vec.push(5);
        vec.push(4);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn unique_sorted_vec_append_works() {
        let mut vec = UniqueSortedVec::new();
        let mut other = UniqueSortedVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5, 6])
    }

     /* Unique Sorted Vector Alternative */
     #[test]
     fn unique_sorted_vec_alt_creation_from_vec_works() {
         let mut src = vec![3, 1, 2, 3];
         let vec = UniqueSortedVecAlt::from_vec(&mut src);
         assert_eq!(*vec, [1, 2, 3]);
     }
 
     #[test]
     fn unique_sorted_vec_alt_push_works() {
         let mut src = vec![0, 1, 2, 3, 4];
         let mut vec = UniqueSortedVecAlt::from_vec(&mut src);
         vec.push(5);
         vec.push(4);
         assert_eq!(*vec, [0, 1, 2, 3, 4, 5]);
     }
 
     #[test]
     fn unique_sorted_vec_alt_append_works() {
         let mut vec = UniqueSortedVecAlt::new();
         let mut other = UniqueSortedVecAlt::new();
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