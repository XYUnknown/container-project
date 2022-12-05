/*LIBSPEC-NAME*
rust-linked-list-spec std::collections::LinkedList
*ENDLIBSPEC-NAME*/

use std::collections::LinkedList;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::iter::FromIterator;
// nightly features
use std::collections::linked_list::CursorMut;
use crate::traits::{Container, Stack, RandomAccess};
use crate::proptest::*;
use proptest::prelude::*;
use proptest::collection::linked_list;

use im::conslist::{ConsList};
use im::conslist;
use std::sync::Arc;

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord> Container<T> for LinkedList<T> {

    /*LIBSPEC*
    /*OPNAME*
    len op-len pre-len post-len
    *ENDOPNAME*/
    (define (op-len xs) (cons xs (length xs)))
    (define (pre-len xs) #t)
    (define (post-len xs r) (equal? r (op-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        LinkedList::len(self)
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
        LinkedList::contains(self, x)
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
        LinkedList::is_empty(self)
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
        LinkedList::clear(self);
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
        LinkedList::push_back(self, elt);
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
        let mut c = self.cursor_front_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Equal => {
                            return c.remove_current()
                        },
                        _ => c.move_next()
                    }
                },
                None => { // empty list
                    return None;
                }
            }
        }
    }
}

/*IMPL*
Stack
*ENDIMPL*/
impl<T> Stack<T> for LinkedList<T> {
    /*LIBSPEC*
    /*OPNAME*
    push push pre-push post-push
    *ENDOPNAME*/
    (define (push xs x) (append xs (list x)))
    (define (pre-push xs) #t)
    (define (post-push xs x ys) (equal? ys (push xs x)))
    *ENDLIBSPEC*/
    fn push(&mut self, elt: T) {
        LinkedList::push_back(self, elt);
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
        LinkedList::pop_back(self)
    }
}

/*IMPL*
RandomAccess
*ENDIMPL*/
impl<T> RandomAccess<T> for LinkedList<T> {
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
        LinkedList::front(self)
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
        LinkedList::back(self)
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
        LinkedList::iter(self).nth(n)
    }                                      
}

struct Con<T> {
    elem_t: PhantomData<T>
}

pub trait Constructor {
    type Impl: ?Sized;
    type Interface: ?Sized;
    fn new() -> Box<Self::Interface>;
}

impl<T: 'static + Ord> Constructor for Con<T> {
    type Impl = LinkedList::<T>;
    type Interface = dyn Container<T>;
    fn new() -> Box<Self::Interface> {
        Box::new(Self::Impl::new())
    }
}

fn abstraction<T>(l: LinkedList<T>) -> ConsList<T> {
    let list: ConsList<T> = ConsList::from_iter(l);
    list
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 100, .. ProptestConfig::default()
      })]
    
    #[test]
    fn test_list_len(ref mut l in linked_list(".*", 0..100)) {
        let abs_list = abstraction(l.clone());
        assert_eq!(Container::<String>::len(l), abs_list.len());
        assert_eq!(abstraction(l.clone()), abs_list);
    }

    #[test]
    fn test_list_contains(ref mut l in linked_list(".*", 0..100), a in ".*") {
        let abs_list = abstraction(l.clone());
        assert_eq!(Container::<String>::contains(l, &a), contains(&abs_list, &a));
        assert_eq!(abstraction(l.clone()), abs_list);
    }

    #[test]
    fn test_list_is_empty(ref mut l in linked_list(".*", 0..100)) {
        let abs_list = abstraction(l.clone());
        assert_eq!(Container::<String>::is_empty(l), abs_list.is_empty());
        assert_eq!(abstraction(l.clone()), abs_list);
    }

    #[test]
    fn test_list_insert(ref mut l in linked_list(".*", 0..100), a in ".*") {
        let abs_list = abstraction(l.clone());
        let after_list = abs_list.append(conslist![a.clone()]);
        Container::<String>::insert(l, a.clone());
        assert_eq!(abstraction(l.clone()), after_list);
    }

    #[test]
    fn test_list_clear(ref mut l in linked_list(".*", 0..100)) {
        let abs_list = abstraction(l.clone());
        let after_list = clear(&abs_list);
        Container::<String>::clear(l);
        assert_eq!(abstraction(l.clone()), after_list);
    }

    #[test]
    fn test_list_remove(ref mut l in linked_list(".*", 0..100), a in ".*") {
        let abs_list = abstraction(l.clone());
        let (after_list, abs_elem) = remove(&abs_list, a.clone());
        let elem = Container::<String>::remove(l, a.clone());
        assert_eq!(abstraction(l.clone()), after_list);
        assert_eq!(elem, abs_elem);
    }

    #[test]
    fn test_list_first(ref mut l in linked_list(".*", 0..100)) {
        let abs_list = abstraction(l.clone());
        let elem = RandomAccess::<String>::first(l);
        let abs_first = first(&abs_list);
        assert_eq!(elem, abs_first);
        assert_eq!(abstraction(l.clone()), abs_list);
    }

    #[test]
    fn test_list_last(ref mut l in linked_list(".*", 0..100)) {
        let abs_list = abstraction(l.clone());
        let elem = RandomAccess::<String>::last(l);
        let abs_last = last(&abs_list);
        assert_eq!(elem, abs_last);
        assert_eq!(abstraction(l.clone()), abs_list);
    }

    #[test]
    fn test_list_nth(ref mut l in linked_list(".*", 0..100), n in 0usize..100) {
        let abs_list = abstraction(l.clone());
        let elem = RandomAccess::<String>::nth(l, n.clone());
        let abs_nth = nth(&abs_list, n);
        assert_eq!(elem, abs_nth);
        assert_eq!(abstraction(l.clone()), abs_list);
    }

    #[test]
    fn test_list_push(ref mut l in linked_list(".*", 0..100), a in ".*") {
        let abs_list = abstraction(l.clone());
        let after_list = push(&abs_list, a.clone());
        Stack::<String>::push(l, a.clone());
        assert_eq!(abstraction(l.clone()), after_list);
    }

    #[test]
    fn test_list_pop(ref mut l in linked_list(".*", 0..100)) {
        let abs_list = abstraction(l.clone());
        let (after_list, abs_elem) = pop(&abs_list);
        let elem = Stack::<String>::pop(l);
        assert_eq!(abstraction(l.clone()), after_list);
        assert_eq!(elem.map(|x| Arc::new(x)), abs_elem);
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::traits::{Container, Stack, RandomAccess};
//     use crate::library::list::{Constructor, Con};
//     use std::collections::LinkedList;

//     #[test]
//     fn test_list_container_trait() {
//         //type Foo<T> = dyn Container<T>;
//         //let list : &mut Foo<u32> = &mut LinkedList::<u32>::new();
//         let list : &mut dyn Container<u32> = &mut LinkedList::<u32>::new();
//         assert_eq!(list.len(), 0);
//         list.insert(1);
//         list.insert(4);
//         assert_eq!(list.len(), 2);
//         assert_eq!(list.remove(9), None);
//         assert_eq!(list.remove(1), Some(1));
//         assert_eq!(list.len(), 1);
//         assert!(list.contains(&4));
//         list.clear();
//         assert_eq!(list.len(), 0);
//         //assert_eq!(list.pop_back(), None); // error
//     }

//     #[test]
//     fn test_list_constructor() {
//         let mut list = Con::<u32>::new();
//         assert_eq!(list.len(), 0);
//         list.insert(1);
//         // assert_eq!(list.pop_back(), None);
//     }

//     #[test]
//     fn test_list_combo_trait() {
//         trait ContainerStack<T> : Container<T> + Stack<T> {}
//         impl<T: Ord> ContainerStack<T> for LinkedList<T> {}
//         let list : &mut dyn ContainerStack<u32> = &mut LinkedList::<u32>::new();
//         assert_eq!(list.len(), 0);
//         list.insert(1);
//         list.insert(4);
//         assert_eq!(list.len(), 2);
//         assert_eq!(list.remove(9), None);
//         assert_eq!(list.remove(1), Some(1));
//         assert_eq!(list.len(), 1);
//         assert!(list.contains(&4));
//         list.clear();
//         assert_eq!(list.len(), 0);
//         //assert_eq!(list.pop_back(), None); // error
//     }

//     #[test]
//     fn test_list_with_position() {
//         trait ContainerRandomAccess<T> : Container<T> + RandomAccess<T> {}
//         impl<T: Ord> ContainerRandomAccess<T> for LinkedList<T> {}
//         let list : &mut dyn ContainerRandomAccess<u32> = &mut LinkedList::<u32>::new();
//         list.insert(1);
//         list.insert(4);
//         list.insert(2);
//         assert_eq!(list.first(), Some(&1));
//         assert_eq!(list.last(), Some(&2));
//         assert_eq!(list.nth(1), Some(&4));
//     }
// }