/*LIBSPEC-NAME*
rust-lazy-unique-vec-spec preprocess::library::lazy_unique_vector::LazyUniqueVec
*ENDLIBSPEC-NAME*/

use std::vec::Vec;
use std::slice::Iter;
use std::ops::Deref;
use crate::traits::{Container, Stack, RandomAccess};

// A Unique Vector
pub struct LazyUniqueVec<T> {
    v: Vec<T>,
    modified: bool
}

impl<T: Ord> LazyUniqueVec<T> {
    pub fn new() -> LazyUniqueVec<T> {
        LazyUniqueVec { 
            v: Vec::new(),
            modified: false
        }
    }

    pub fn len(&mut self) -> usize {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.len()
    }

    pub fn contains(&mut self, x: &T) -> bool {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.binary_search(x).is_ok()
    }

    pub fn is_empty(&mut self) -> bool {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.len() == 0
    }

    // Duplicated elements will be discarded
    pub fn push(&mut self, value: T) {
        self.v.push(value);
        self.modified = true;
    }

    pub fn pop(&mut self) -> Option<T> {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.pop()
    }

    pub fn remove(&mut self, index: usize) -> T {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.remove(index)
    }

    pub fn clear(&mut self) {
        self.v.clear()
    }

    pub fn first(&mut self) -> Option<&T> {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.first()
    }

    pub fn last(&mut self) -> Option<&T> {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.last()
    }

    pub fn iter(&mut self) -> Iter<'_, T> {
        if (self.modified) {
            self.v.sort();
            self.v.dedup();
            self.modified = false;
        }
        self.v.iter()
    }
}

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord> Container<T> for LazyUniqueVec<T> {

    /*LIBSPEC*
    /*OPNAME*
    len op-len pre-len post-len
    *ENDOPNAME*/
    (define (op-len xs) (cons xs (length xs)))
    (define (pre-len xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-len xs r) (equal? r (op-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        LazyUniqueVec::len(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    contains op-contains pre-contains post-contains
    *ENDOPNAME*/
    (define (op-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    (define (pre-contains xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-contains xs x r) (equal? r (op-contains xs x)))
    *ENDLIBSPEC*/
    fn contains(&mut self, x: &T) -> bool {
        LazyUniqueVec::contains(self, x) // use fully qualified syntax to avoid function name collision
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty op-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (op-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-is-empty xs r) (equal? r (op-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&mut self) -> bool {
        LazyUniqueVec::is_empty(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    clear op-clear pre-clear post-clear 
    *ENDOPNAME*/
    (define (op-clear xs) null)
    (define (pre-clear xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-clear xs r) (equal? r (op-clear xs)))
    *ENDLIBSPEC*/
    fn clear(&mut self) {
        LazyUniqueVec::clear(self);
    }

    /*LIBSPEC*
    /*OPNAME*
    insert op-insert pre-insert post-insert
    *ENDOPNAME*/
    (define (op-insert xs x) (remove-duplicates (sort (append xs (list x)) <)))
    (define (pre-insert xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-insert xs x ys) (equal? ys (op-insert xs x)))
    *ENDLIBSPEC*/
    fn insert(&mut self, elt: T) {
        LazyUniqueVec::push(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    remove op-remove pre-remove post-remove
    *ENDOPNAME*/
    (define (op-remove xs x)
      (cond
        [(list? (member x xs)) (cons (remove x xs) x)]
        [else (cons xs null)]))
    (define (pre-remove xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-remove xs r) (equal? r (op-remove xs)))
    *ENDLIBSPEC*/
    fn remove(&mut self, elt: T) -> Option<T> {
        match self.iter().position(|x| *x == elt) {
            Some(index) => {
                Some(self.remove(index))
            },
            None => None
        }
    }
}

/*IMPL*
RandomAccess
*ENDIMPL*/
impl<T: Ord> RandomAccess<T> for LazyUniqueVec<T> {
    /*LIBSPEC*
    /*OPNAME*
    first op-first pre-first post-first
    *ENDOPNAME*/
    (define (op-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    (define (pre-first xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-first xs r) (equal? r (op-first xs)))
    *ENDLIBSPEC*/
    fn first(&mut self) -> Option<&T> {
        LazyUniqueVec::first(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    last op-last pre-last post-last
    *ENDOPNAME*/
    (define (op-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    (define (pre-last xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-last xs r) (equal? r (op-last xs)))
    *ENDLIBSPEC*/
    fn last(&mut self) -> Option<&T> {
        LazyUniqueVec::last(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    nth op-nth pre-nth post-nth
    *ENDOPNAME*/
    (define (op-nth xs n)
      (cond
        [(>= n (length xs)) (cons xs null)]
        [(< n 0) (cons xs null)]
        [else (cons xs (list-ref xs n))]))
    (define (pre-nth xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-nth n xs r) (equal? r (op-nth xs n)))
    *ENDLIBSPEC*/
    fn nth(&mut self, n: usize) -> Option<&T> {
        LazyUniqueVec::iter(self).nth(n)
    }                                      
}

#[cfg(test)]
mod tests {
    use crate::library::lazy_unique_vector::LazyUniqueVec;
    /** Unique Vector*/
    #[test]
    fn unique_vec_creation() {
        let mut vec = LazyUniqueVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_vec_contains() {
        let mut vec = LazyUniqueVec::<u32>::new();
        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.contains(&10), false);
        assert_eq!(vec.contains(&6), true);
    }
}