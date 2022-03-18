/*LIBSPEC-NAME*
rust-eager-unique-vec-spec preprocess::library::eager_unique_vector::EagerUniqueVec
*ENDLIBSPEC-NAME*/

use std::vec::Vec;
use std::slice::Iter;
use std::ops::Deref;
use crate::traits::{Container, Stack, RandomAccess};

// A Unique Vector
pub struct EagerUniqueVec<T> {
    v: Vec<T>,
}

impl<T: PartialEq> EagerUniqueVec<T> {
    pub fn new() -> EagerUniqueVec<T> {
        EagerUniqueVec { v: Vec::new() }
    }

    pub fn len(&mut self) -> usize {
        self.v.len()
    }

    pub fn contains(&mut self, x: &T) -> bool {
        self.v.contains(x)
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    // Duplicated elements will be discarded
    pub fn push(&mut self, value: T) {
        if !self.contains(&value) {
            self.v.push(value);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.v.remove(index)
    }

    pub fn clear(&mut self) {
        self.v.clear()
    }

    pub fn first(&mut self) -> Option<&T> {
        self.v.first()
    }

    pub fn last(&mut self) -> Option<&T> {
        self.v.last()
    }

    pub fn iter(&mut self) -> Iter<'_, T> {
        self.v.iter()
    }
}

/*IMPL*
Container
*ENDIMPL*/
impl<T: PartialEq> Container<T> for EagerUniqueVec<T> {

    /*LIBSPEC*
    /*OPNAME*
    len spec-len pre-len post-len
    *ENDOPNAME*/
    (define (spec-len xs) (cons xs (length xs)))
    (define (pre-len xs) (equal? xs (remove-duplicates xs)))
    (define (post-len xs r) (equal? r (spec-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        EagerUniqueVec::len(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    contains spec-contains pre-contains post-contains
    *ENDOPNAME*/
    (define (spec-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    (define (pre-contains xs) (equal? xs (remove-duplicates xs)))
    (define (post-contains xs x r) (equal? r (spec-contains xs x)))
    *ENDLIBSPEC*/
    fn contains(&mut self, x: &T) -> bool {
        EagerUniqueVec::contains(self, x) // use fully qualified syntax to avoid function name collision
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty spec-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (spec-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) (equal? xs (remove-duplicates xs)))
    (define (post-is-empty xs r) (equal? r (spec-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&mut self) -> bool {
        EagerUniqueVec::is_empty(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    clear spec-clear pre-clear post-clear 
    *ENDOPNAME*/
    (define (spec-clear xs) null)
    (define (pre-clear xs) (equal? xs (remove-duplicates xs)))
    (define (post-clear xs r) (equal? r (spec-clear xs)))
    *ENDLIBSPEC*/
    fn clear(&mut self) {
        EagerUniqueVec::clear(self);
    }

    /*LIBSPEC*
    /*OPNAME*
    insert spec-insert pre-insert post-insert
    *ENDOPNAME*/
    (define (spec-insert xs x) (remove-duplicates (append xs (list x))))
    (define (pre-insert xs) (equal? xs (remove-duplicates xs)))
    (define (post-insert xs x ys) (equal? ys (spec-insert xs x)))
    *ENDLIBSPEC*/
    fn insert(&mut self, elt: T) {
        EagerUniqueVec::push(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    remove spec-remove pre-remove post-remove
    *ENDOPNAME*/
    (define (spec-remove xs x)
      (cond
        [(list? (member x xs)) (cons (remove x xs) x)]
        [else (cons xs null)]))
    (define (pre-remove xs) (equal? xs (remove-duplicates xs)))
    (define (post-remove xs r) (equal? r (spec-remove xs)))
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
impl<T: PartialEq> RandomAccess<T> for EagerUniqueVec<T> {
    /*LIBSPEC*
    /*OPNAME*
    first spec-first pre-first post-first
    *ENDOPNAME*/
    (define (spec-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    (define (pre-first xs) #t)
    (define (post-first xs r) (equal? r (spec-first xs)))
    *ENDLIBSPEC*/
    fn first(&mut self) -> Option<&T> {
        EagerUniqueVec::first(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    last spec-last pre-last post-last
    *ENDOPNAME*/
    (define (spec-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    (define (pre-last xs) #t)
    (define (post-last xs r) (equal? r (spec-last xs)))
    *ENDLIBSPEC*/
    fn last(&mut self) -> Option<&T> {
        EagerUniqueVec::last(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    nth spec-nth pre-nth post-nth
    *ENDOPNAME*/
    (define (spec-nth xs n)
      (cond
        [(>= n (length xs)) (cons xs null)]
        [(< n 0) (cons xs null)]
        [else (cons xs (list-ref xs n))]))
    (define (pre-nth xs) #t)
    (define (post-nth xs n r) (equal? r (spec-nth xs n)))
    *ENDLIBSPEC*/
    fn nth(&mut self, n: usize) -> Option<&T> {
        EagerUniqueVec::iter(self).nth(n)
    }                                      
}

#[cfg(test)]
mod tests {
    use crate::library::eager_unique_vector::EagerUniqueVec;
    /** Unique Vector*/
    #[test]
    fn unique_vec_creation() {
        let mut vec = EagerUniqueVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }
}