/*LIBSPEC-NAME*
rust-list-spec
*ENDLIBSPEC-NAME*/

use std::collections::LinkedList;
use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::linked_list::Iter;

pub struct List<T> {
    ll: LinkedList<T>,
}

impl<T> List<T> {
    pub const fn new() -> List<T> {
        List { ll: LinkedList::new() }
    }
    
    /*LIBSPEC*
    ; op-name
    ; len
    ; end-op-name
    ; spec
    (define (spec-length xs) (cons xs (length xs)))
    ; end-spec
    ; pre
    (define (pre-length xs) #t)
    ; end-pre
    ; post
    (define (post-length xs r) (equal? r (spec-length xs)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn len(&self) -> usize {
        self.ll.len()
    }

    /*LIBSPEC*
    ; op-name
    ; contains
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
        self.ll.contains(x)
    }

    /*LIBSPEC*
    ; op-name
    ; is-empty
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
        self.ll.is_empty()
    }

    /*LIBSPEC*
    ; op-name
    ; push
    ; end-op-name
    ; spec
    (define (spec-push xs x) (append xs (list x)))
    ; end-spec
    ; pre
    (define (pre-push xs) #t)
    ; end-pre
    ; post
    (define (post-push xs x ys) (equal? ys (spec-push xs x)))
    ; end-post 
    *ENDLIBSPEC*/
    pub fn push(&mut self, elt: T) {
        self.ll.push_back(elt);
    }

    /*LIBSPEC*
    ; op-name
    ; pop
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
        self.ll.pop_back()
    }

    /*LIBSPEC*
    ; op-name
    ; clear
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
        self.ll.clear();
    }

    /*LIBSPEC*
    ; op-name
    ; first
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
        self.ll.front()
    }

    /*LIBSPEC*
    ; op-name
    ; last
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
        self.ll.back()
    }
}

impl<T> Deref for List<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ll
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List { ll: self.ll.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ll.clone_from(&source.ll);
    }
}
