/*LIBSPEC-NAME*
rust-vec-spec std::vec::Vec
*ENDLIBSPEC-NAME*/

use std::vec::Vec;
use crate::traits::{Container, Stack, RandomAccess};

use proptest::prelude::*;
use proptest::collection::vec;

use im::conslist::{ConsList};

/*IMPL*
Container
*ENDIMPL*/
impl<T: PartialEq> Container<T> for Vec<T> {

    /*LIBSPEC*
    /*OPNAME*
    len op-len pre-len post-len
    *ENDOPNAME*/
    (define (op-len xs) (cons xs (length xs)))
    (define (pre-len xs) #t)
    (define (post-len xs r) (equal? r (op-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        Vec::len(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    contains op-contains pre-contains post-contains
    *ENDOPNAME*/
    (define (op-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    (define (pre-contains xs) #t)
    (define (post-contains xs x r) (equal? r (op-contains xs x)))
    *ENDLIBSPEC*/
    fn contains(&mut self, x: &T) -> bool {
        <[T]>::contains(self, x) // use fully qualified syntax to avoid function name collision
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty op-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (op-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) #t)
    (define (post-is-empty xs r) (equal? r (op-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&mut self) -> bool {
        Vec::is_empty(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    clear op-clear pre-clear post-clear 
    *ENDOPNAME*/
    (define (op-clear xs) null)
    (define (pre-clear xs) #t)
    (define (post-clear xs r) (equal? r (op-clear xs)))
    *ENDLIBSPEC*/
    fn clear(&mut self) {
        Vec::clear(self);
    }

    /*LIBSPEC*
    /*OPNAME*
    insert op-insert pre-insert post-insert
    *ENDOPNAME*/
    (define (op-insert xs x) (append xs (list x)))
    (define (pre-insert xs) #t)
    (define (post-insert xs x ys) (equal? ys (op-insert xs x)))
    *ENDLIBSPEC*/
    fn insert(&mut self, elt: T) {
        Vec::push(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    remove op-remove pre-remove post-remove
    *ENDOPNAME*/
    (define (op-remove xs x)
      (cond
        [(list? (member x xs)) (cons (remove x xs) x)]
        [else (cons xs null)]))
    (define (pre-remove xs) #t)
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
Stack
*ENDIMPL*/
impl<T> Stack<T> for Vec<T> {
    /*LIBSPEC*
    /*OPNAME*
    push push pre-push post-push
    *ENDOPNAME*/
    (define (push xs x) (append xs (list x)))
    (define (pre-push xs) #t)
    (define (post-push xs x ys) (equal? ys (push xs x)))
    *ENDLIBSPEC*/
    fn push(&mut self, elt: T) {
        Vec::push(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    pop pop pre-pop post-pop
    *ENDOPNAME*/
    (define (pop xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons (take xs (- (length xs) 1)) (last xs))]))
    (define (pre-pop xs) #t)
    (define (post-pop xs r) (equal? r (pop xs)))
    *ENDLIBSPEC*/
    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }
}

/*IMPL*
RandomAccess
*ENDIMPL*/
impl<T> RandomAccess<T> for Vec<T> {
    /*LIBSPEC*
    /*OPNAME*
    first op-first pre-first post-first
    *ENDOPNAME*/
    (define (op-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    (define (pre-first xs) #t)
    (define (post-first xs r) (equal? r (op-first xs)))
    *ENDLIBSPEC*/
    fn first(&mut self) -> Option<&T> {
        <[T]>::first(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    last op-last pre-last post-last
    *ENDOPNAME*/
    (define (op-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    (define (pre-last xs) #t)
    (define (post-last xs r) (equal? r (op-last xs)))
    *ENDLIBSPEC*/
    fn last(&mut self) -> Option<&T> {
        <[T]>::last(self)
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
    (define (pre-nth xs) #t)
    (define (post-nth xs n r) (equal? r (op-nth xs n)))
    *ENDLIBSPEC*/
    fn nth(&mut self, n: usize) -> Option<&T> {
        <[T]>::iter(self).nth(n)
    }                                      
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abstraction<T>(v: Vec<T>) -> ConsList<T> {
    let list: ConsList<T> = ConsList::from(v);
    list
}

proptest! {
    #[test]
    fn test_vec_strategy(ref v in vec(".*", 10..100)) {
        assert!(v.len() < 100);
        assert!(v.len() >= 10);
    }

    #[test]
    fn test_vec_len(ref v in vec(".*", 10..100)) {
        assert_eq!(v.len(), abstraction(v.clone()).len());
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{Container, RandomAccess};
    use std::vec::Vec;

    #[test]
    fn test_vec_container_trait() {
        let vec : &mut dyn Container<u32> = &mut Vec::<u32>::new();
        assert_eq!(vec.len(), 0);
        vec.insert(1);
        vec.insert(4);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.remove(9), None);
        assert_eq!(vec.remove(1), Some(1));
        assert_eq!(vec.len(), 1);
        assert!(vec.contains(&4));
        vec.clear();
        assert_eq!(vec.len(), 0);
        //assert_eq!(vec.pop(), None); // error
    }

    #[test]
    fn test_vec_with_position() {
        trait ContainerRandomAccess<T> : Container<T> + RandomAccess<T> {}
        impl<T: Ord> ContainerRandomAccess<T> for Vec<T> {}
        let vec : &mut dyn ContainerRandomAccess<u32> = &mut Vec::<u32>::new();
        vec.insert(1);
        vec.insert(4);
        vec.insert(2);
        assert_eq!(vec.first(), Some(&1));
        assert_eq!(vec.last(), Some(&2));
        assert_eq!(vec.nth(1), Some(&4));
    }
}