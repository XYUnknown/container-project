/*LIBSPEC-NAME*
rust-vec-spec std::vec::Vec
*ENDLIBSPEC-NAME*/

use std::vec::Vec;
use crate::traits::{Container, Stack, RandomAccess};
use crate::proptest::*;

use proptest::prelude::*;
use proptest::collection::vec;

use im::conslist::{ConsList};
use im::conslist;
use std::sync::Arc;

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
    fn test_vec_len(ref mut v in vec(".*", 0..100)) {
        let abs_list = abstraction(v.clone());
        assert_eq!(Container::<String>::len(v), abs_list.len());
        assert_eq!(abstraction(v.clone()), abs_list);
    }

    #[test]
    fn test_vec_contains(ref mut v in vec(".*", 0..100), a in ".*") {
        let abs_list = abstraction(v.clone());
        assert_eq!(Container::<String>::contains(v, &a), contains(&abs_list, &a));
        assert_eq!(abstraction(v.clone()), abs_list);
    }

    #[test]
    fn test_vec_is_empty(ref mut v in vec(".*", 0..100)) {
        let abs_list = abstraction(v.clone());
        assert_eq!(Container::<String>::is_empty(v), abs_list.is_empty());
        assert_eq!(abstraction(v.clone()), abs_list);
    }

    #[test]
    fn test_vec_insert(ref mut v in vec(".*", 0..100), a in ".*") {
        let abs_list = abstraction(v.clone());
        let after_list = abs_list.append(conslist![a.clone()]);
        Container::<String>::insert(v, a.clone());
        assert_eq!(abstraction(v.clone()), after_list);
    }

    #[test]
    fn test_vec_clear(ref mut v in vec(".*", 0..100)) {
        let abs_list = abstraction(v.clone());
        let after_list = clear(&abs_list);
        Container::<String>::clear(v);
        assert_eq!(abstraction(v.clone()), after_list);
    }

    #[test]
    fn test_vec_remove(ref mut v in vec(".*", 0..100), a in ".*") {
        let abs_list = abstraction(v.clone());
        let (after_list, abs_elem) = remove(&abs_list, a.clone());
        let elem = Container::<String>::remove(v, a.clone());
        assert_eq!(abstraction(v.clone()), after_list);
        assert_eq!(elem, abs_elem);
    }

    #[test]
    fn test_vec_first(ref mut v in vec(".*", 0..100)) {
        let abs_list = abstraction(v.clone());
        let elem = RandomAccess::<String>::first(v);
        let abs_first = first(&abs_list);
        assert_eq!(elem, abs_first);
        assert_eq!(abstraction(v.clone()), abs_list);
    }

    #[test]
    fn test_vec_last(ref mut v in vec(".*", 0..100)) {
        let abs_list = abstraction(v.clone());
        let elem = RandomAccess::<String>::last(v);
        let abs_last = last(&abs_list);
        assert_eq!(elem, abs_last);
        assert_eq!(abstraction(v.clone()), abs_list);
    }

    #[test]
    fn test_vec_nth(ref mut v in vec(".*", 0..100), n in 0usize..100) {
        let abs_list = abstraction(v.clone());
        let elem = RandomAccess::<String>::nth(v, n.clone());
        let abs_nth = nth(&abs_list, n);
        assert_eq!(elem, abs_nth);
        assert_eq!(abstraction(v.clone()), abs_list);
    }

    #[test]
    fn test_vec_push(ref mut v in vec(".*", 0..100), a in ".*") {
        let abs_list = abstraction(v.clone());
        let after_list = push(&abs_list, a.clone());
        Stack::<String>::push(v, a.clone());
        assert_eq!(abstraction(v.clone()), after_list);
    }

    #[test]
    fn test_vec_pop(ref mut v in vec(".*", 0..100)) {
        let abs_list = abstraction(v.clone());
        let (after_list, abs_elem) = pop(&abs_list);
        let elem = Stack::<String>::pop(v);
        assert_eq!(abstraction(v.clone()), after_list);
        assert_eq!(elem.map(|x| Arc::new(x)), abs_elem);
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