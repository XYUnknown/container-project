/*LIBSPEC-NAME*
rust-treeset-spec TreeSet
*ENDLIBSPEC-NAME*/

use std::collections::BTreeSet;
use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::btree_set::Iter;

pub struct TreeSet<T> {
    t: BTreeSet<T>,
}

impl<T: Ord> TreeSet<T> {
    pub fn new() -> TreeSet<T> {
        TreeSet { t: BTreeSet::new() }
    }
    
    /*LIBSPEC*
    /*OPNAME*
    len spec-len pre-len post-len
    *ENDOPNAME*/
    ; spec
    (define (spec-len xs) (cons xs (length xs)))
    ; end-spec
    ; pre
    (define (pre-len xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-len xs r) (equal? r (spec-len xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn len(&self) -> usize {
        self.t.len()
    }

    /*LIBSPEC*
    /*OPNAME*
    contains spec-contains pre-contains post-contains
    *ENDOPNAME*/
    ; spec
    (define (spec-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    ; end-spec
    ; pre
    (define (pre-contains xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-contains xs x r) (equal? r (spec-contains xs x)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn contains(&self, x: &T) -> bool 
    where
        T: PartialEq<T>,
    {
        self.t.contains(x)
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty spec-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    ; spec
    (define (spec-is-empty xs) (cons xs (null? xs)))
    ; end-spec
    ; pre
    (define (pre-is-empty xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-is-empty xs r) (equal? r (spec-is-empty xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn is_empty(&self) -> bool {
        self.t.is_empty()
    }

    /*LIBSPEC*
    /*OPNAME*
    insert spec-insert pre-insert post-insert
    *ENDOPNAME*/
    ; spec
    (define (spec-insert xs x) (remove-duplicates (sort (append xs (list x)) <)))
    ; end-spec
    ; pre
    (define (pre-insert xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-insert xs x ys) (equal? ys (spec-insert xs x)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn insert(&mut self, elt: T) {
        self.t.insert(elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    pop spec-pop pre-pop post-pop
    *ENDOPNAME*/
    ; spec
    (define (spec-pop xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons (take xs (- (length xs) 1)) (last xs))]))
    ; end-spec
    ; pre
    (define (pre-pop xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-pop xs r) (equal? r (spec-pop xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn pop(&mut self) -> Option<T> {
        self.t.pop_last()
    }

    /*LIBSPEC*
    /*OPNAME*
    clear spec-clear pre-clear post-clear 
    *ENDOPNAME*/
    ; spec
    (define (spec-clear xs) null)
    ; end-spec
    ; pre
    (define (pre-clear xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-clear xs r) (equal? r (spec-clear xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn clear(&mut self) {
        self.t.clear();
    }

    /*LIBSPEC*
    /*OPNAME*
    first spec-first pre-first post-first
    *ENDOPNAME*/
    ; spec
    (define (spec-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    ; end-spec
    ; pre
    (define (pre-first xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-first xs r) (equal? r (spec-first xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn first(&self) -> Option<&T> {
        self.t.first()
    }

    /*LIBSPEC*
    /*OPNAME*
    last spec-last pre-last post-last
    *ENDOPNAME*/
    ; spec
    (define (spec-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    ; end-spec
    ; pre
    (define (pre-last xs) (equal? xs (remove-duplicates (sort xs <))))
    ; end-pre
    ; post
    (define (post-last xs r) (equal? r (spec-last xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn last(&self) -> Option<&T> {
        self.t.last()
    }
}

impl<T> Deref for TreeSet<T> {
    type Target = BTreeSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T> DerefMut for TreeSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<T: Clone> Clone for TreeSet<T> {
    fn clone(&self) -> Self {
        TreeSet {t : self.t.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.t.clone_from(&source.t);
    }
}
