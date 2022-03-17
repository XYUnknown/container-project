/*LIBSPEC-NAME*
rust-vec-spec std::vec::Vec
*ENDLIBSPEC-NAME*/

use std::vec::Vec;
use crate::traits::{Container, Stack, RandomAccess};

/*IMPL*
Container
*ENDIMPL*/
impl<T: PartialEq> Container<T> for Vec<T> {

    /*LIBSPEC*
    /*OPNAME*
    len spec-len pre-len post-len
    *ENDOPNAME*/
    (define (spec-len xs) (cons xs (length xs)))
    (define (pre-len xs) #t)
    (define (post-len xs r) (equal? r (spec-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        Vec::len(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    contains spec-contains pre-contains post-contains
    *ENDOPNAME*/
    (define (spec-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    (define (pre-contains xs) #t)
    (define (post-contains xs x r) (equal? r (spec-contains xs x)))
    *ENDLIBSPEC*/
    fn contains(&mut self, x: &T) -> bool {
        <[T]>::contains(self, x) // use fully qualified syntax to avoid function name collision
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty spec-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (spec-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) #t)
    (define (post-is-empty xs r) (equal? r (spec-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&mut self) -> bool {
        Vec::is_empty(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    clear spec-clear pre-clear post-clear 
    *ENDOPNAME*/
    (define (spec-clear xs) null)
    (define (pre-clear xs) #t)
    (define (post-clear xs r) (equal? r (spec-clear xs)))
    *ENDLIBSPEC*/
    fn clear(&mut self) {
        Vec::clear(self);
    }

    /*LIBSPEC*
    /*OPNAME*
    insert spec-insert pre-insert post-insert
    *ENDOPNAME*/
    (define (spec-insert xs x) (append xs (list x)))
    (define (pre-insert xs) #t)
    (define (post-insert xs x ys) (equal? ys (spec-insert xs x)))
    *ENDLIBSPEC*/
    fn insert(&mut self, elt: T) {
        Vec::push(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    remove spec-remove pre-remove post-remove
    *ENDOPNAME*/
    (define (spec-remove xs x)
      (cond
        [(list? (member x xs)) (cons (remove x xs) x)]
        [else (cons xs null)]))
    (define (pre-remove xs) #t)
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
Stack
*ENDIMPL*/
impl<T> Stack<T> for Vec<T> {
    /*LIBSPEC*
    /*OPNAME*
    push spec-push pre-push post-push
    *ENDOPNAME*/
    (define (spec-push xs x) (append xs (list x)))
    (define (pre-push xs) #t)
    (define (post-push xs x ys) (equal? ys (spec-push xs x)))
    *ENDLIBSPEC*/
    fn push(&mut self, elt: T) {
        Vec::push(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    pop spec-pop pre-pop post-pop
    *ENDOPNAME*/
    (define (spec-pop xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons (take xs (- (length xs) 1)) (last xs))]))
    (define (pre-pop xs) #t)
    (define (post-pop xs r) (equal? r (spec-pop xs)))
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
        <[T]>::first(self)
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
        <[T]>::last(self)
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
        <[T]>::iter(self).nth(n)
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