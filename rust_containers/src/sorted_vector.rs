use std::vec::Vec;
use std::slice::Iter;
use std::ops::Deref;
use std::ops::DerefMut;
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
    pub fn contains(&self, x: &T) -> bool {
        match self.v.binary_search(x) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /**
     * Modifying the vector
     */
    pub fn push(&mut self, value: T) {
        let index = self.v.binary_search(&value).unwrap_or_else(|i| i);
        self.v.insert(index, value);
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

    // Removes consecutive repeated elements 
    pub fn dedup(&mut self) {
        self.v.dedup();
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
                                Ordering::Less | Ordering::Equal => temp.push(self.remove(0)),
                                Ordering::Greater => temp.push(other.remove(0))
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

// Accessing the single field of a SortedVec
impl<T> Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for SortedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
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

// An Alternative Sorted Vector
pub struct SortedVecAlt<T> {
    v: Vec<T>,
    is_sorted: bool
}

impl<T: Ord> SortedVecAlt<T> {
    pub const fn new() -> SortedVecAlt<T> {
        SortedVecAlt { v: Vec::new(), is_sorted: true }
    }

    pub fn with_capacity(capacity: usize) -> SortedVecAlt<T> {
        SortedVecAlt { v: Vec::with_capacity(capacity), is_sorted: true }
    }

    pub fn from_vec(src: &mut Vec<T>) -> SortedVecAlt<T> 
    where
        T: Clone
        {
            SortedVecAlt { v: src.to_vec(), is_sorted: false }
        }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn capacity(&self) -> usize {
        self.v.capacity()
    }

    // Not sorted, O(n)
    pub fn contains(&self, x: &T) -> bool {
        self.v.contains(x)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /**
     * Modifying the vector
     */
    pub fn push(&mut self, value: T) {
        self.v.push(value);
        self.is_sorted = false;
    }

    
    // Sort and then pop
    // By default get the maximum element  
    pub fn pop(&mut self) -> Option<T> {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.pop()
    }

    // It is not very meaningful to remove by index
    // given the index does not maintain the insertion order
    // but can be mainful for operations like looking for the median
    pub fn remove(&mut self, index: usize) -> T {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.remove(index)
    }

    pub fn truncate(&mut self, len: usize) {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.truncate(len);
    }

    pub fn clear(&mut self) {
        self.v.truncate(0);
    }

    // Removes consecutive repeated elements 
    pub fn dedup(&mut self) {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.dedup();
    }

    // Merge them into one vector
    pub fn append(&mut self, other: &mut Self) {
        self.v.append(&mut other.v);
        self.is_sorted = false;
    }
    
    /**
     * Accessing elements
     */
     pub fn first(&mut self) -> Option<&T> {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.first()
    }

    pub fn last(&mut self) -> Option<&T> {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.last()
    }

    pub fn get<I>(&mut self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
        {
            if !self.is_sorted{
                self.v.sort();
                self.is_sorted = true;
            }
            self.v.get(index)
        }
    
    pub fn iter(&mut self) -> Iter<'_, T> {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        self.v.iter()
    }

    pub fn to_vec(&mut self) -> &Vec<T> {
        if !self.is_sorted{
            self.v.sort();
            self.is_sorted = true;
        }
        &self.v
    }
}

impl<T: Clone> Clone for SortedVecAlt<T> {
    fn clone(&self) -> Self {
        SortedVecAlt { v: self.v.clone(), is_sorted: self.is_sorted.clone()  }
    }

    fn clone_from(&mut self, source: &Self) {
        self.v.clone_from(&source.v);
    }
}

#[macro_export]
// SortedVec creations
macro_rules! sorted_vec {
    ($($x:expr),*) => { // e.g., sorted_vec![1, 2, 3]
        {
            let mut vec = SortedVec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
    ($elem:expr; $n:expr) => { // e.g., sorted_vec![1; 3]
        {
            let mut src = std::vec::from_elem($elem, $n);
            let mut vec = SortedVec::from_vec(&mut src);
            vec
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::sorted_vector::SortedVec;
    use crate::sorted_vector::SortedVecAlt;

    /* Sorted Vector */
    #[test]
    fn sorted_vec_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let vec = SortedVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3, 3]);
    }

    #[test]
    fn sorted_vec_macro_one_works() {
        let vec = sorted_vec![3, 7, 2, 1, 5, 4, 3];
        assert_eq!(*vec, [1, 2, 3, 3, 4, 5, 7])
    }

    #[test]
    fn sorted_vec_macro_two_works() {
        let vec = sorted_vec![1; 3];
        assert_eq!(*vec, [1, 1, 1])
    }

    #[test]
    fn sorted_vec_contains_works() {
        let mut vec = SortedVec::<u32>::new();
        assert_eq!(vec.contains(&1), false);
    }

    #[test]
    fn sorted_vec_push_works() {
        let mut vec = SortedVec::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(*vec, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn sorted_vec_append_works() {
        let mut vec = SortedVec::new();
        let mut other = SortedVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 2, 3, 3, 4, 4, 5, 6])
    }

    /* SortedVecAlt */
    #[test]
    fn sorted_vec_alt_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let mut vec = SortedVecAlt::from_vec(&mut src);
        assert_eq!(*vec.to_vec(), [1, 2, 3, 3]);
    }

    #[test]
    fn sorted_vec_alt_contains_works() {
        let mut vec = SortedVecAlt::<u32>::new();
        assert_eq!(vec.contains(&1), false);
    }

    #[test]
    fn sorted_vec_alt_push_works() {
        let mut vec = SortedVecAlt::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(*vec.to_vec(), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn sorted_vec_alt_pop_works() {
        let mut vec = SortedVecAlt::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(vec.pop(), Some(4));
    }

    #[test]
    fn sorted_vec_alt_append_works() {
        let mut vec = SortedVecAlt::new();
        let mut other = SortedVecAlt::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec.to_vec(), [0, 1, 2, 2, 3, 3, 4, 4, 5, 6])
    }

}
