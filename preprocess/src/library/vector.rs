/*LIBSPEC-NAME*
rust-vector-spec
*ENDLIBSPEC-NAME*/

use std::vec::Vec;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct Vector<T> {
    v: Vec<T>,
}

impl<T> Vector<T> {
    pub const fn new() -> Vector<T> {
        Vector { v: Vec::new() }
    }
    
    /*LIBSPEC*
    ; op-name
    len spec-len pre-len post-len
    ; end-op-name
    ; spec
    (define (spec-len xs) (cons xs (length xs)))
    ; end-spec
    ; pre
    (define (pre-len xs) #t)
    ; end-pre
    ; post
    (define (post-len xs r) (equal? r (spec-len xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /*LIBSPEC*
    ; op-name
    contains spec-contains pre-contains post-contains
    ; end-op-name
    ; spec
    (define (spec-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    ; end-spec
    ; pre
    (define (pre-contains xs) #t)
    ; end-pre
    ; post
    (define (post-contains xs x r) (equal? r (spec-contains xs x)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn contains(&self, x: &T) -> bool 
    where
        T: PartialEq<T>,
    {
        self.v.contains(x)
    }

    /*LIBSPEC*
    ; op-name
    is-empty spec-is-empty pre-is-empty post-is-empty
    ; end-op-name
    ; spec
    (define (spec-is-empty xs) (cons xs (null? xs)))
    ; end-spec
    ; pre
    (define (pre-is-empty xs) #t)
    ; end-pre
    ; post
    (define (post-is-empty xs r) (equal? r (spec-is-empty xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    /*LIBSPEC*
    ; op-name
    insert spec-insert pre-insert post-insert
    ; end-op-name
    ; spec
    (define (spec-insert xs x) (append xs (list x)))
    ; end-spec
    ; pre
    (define (pre-insert xs) #t)
    ; end-pre
    ; post
    (define (post-insert xs x ys) (equal? ys (spec-insert xs x)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn insert(&mut self, elt: T) {
        self.v.push(elt);
    }

    /*LIBSPEC*
    ; op-name
    pop spec-pop pre-pop post-pop
    ; end-op-name
    ; spec
    (define (spec-pop xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons (take xs (- (length xs) 1)) (last xs))]))
    ; end-spec
    ; pre
    (define (pre-pop xs) #t)
    ; end-pre
    ; post
    (define (post-pop xs r) (equal? r (spec-pop xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    /*LIBSPEC*
    ; op-name
    clear spec-clear pre-clear post-clear 
    ; end-op-name
    ; spec
    (define (spec-clear xs) null)
    ; end-spec
    ; pre
    (define (pre-clear xs) #t)
    ; end-pre
    ; post
    (define (post-clear xs r) (equal? r (spec-clear xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn clear(&mut self) {
        self.v.clear();
    }

    /*LIBSPEC*
    ; op-name
    first spec-first pre-first post-first
    ; end-op-name
    ; spec
    (define (spec-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    ; end-spec
    ; pre
    (define (pre-first xs) #t)
    ; end-pre
    ; post
    (define (post-first xs r) (equal? r (spec-first xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn first(&self) -> Option<&T> {
        self.v.first()
    }

    /*LIBSPEC*
    ; op-name
    last spec-last pre-last post-last
    ; end-op-name
    ; spec
    (define (spec-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    ; end-spec
    ; pre
    (define (pre-last xs) #t)
    ; end-pre
    ; post
    (define (post-last xs r) (equal? r (spec-last xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn last(&self) -> Option<&T> {
        self.v.last()
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}

impl<T: Clone> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector { v: self.v.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.v.clone_from(&source.v);
    }
}
