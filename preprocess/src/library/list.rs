/*LIBSPEC-NAME*
rust-linked-list-spec std::collections::LinkedList
*ENDLIBSPEC-NAME*/

use std::collections::LinkedList;
use std::cmp::Ordering;
use std::marker::PhantomData;
// nightly features
use std::collections::linked_list::CursorMut;
use crate::traits::{Container, Stack, WithPosition};

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord> Container<T> for LinkedList<T> {

    /*LIBSPEC*
    /*OPNAME*
    len spec-len pre-len post-len
    *ENDOPNAME*/
    (define (spec-len xs) (cons xs (length xs)))
    (define (pre-len xs) #t)
    (define (post-len xs r) (equal? r (spec-len xs)))
    *ENDLIBSPEC*/
    fn len(&self) -> usize {
        LinkedList::len(self)
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
    fn contains(&self, x: &T) -> bool {
        LinkedList::contains(self, x)
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty spec-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (spec-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) #t)
    (define (post-is-empty xs r) (equal? r (spec-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&self) -> bool {
        LinkedList::is_empty(self)
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
        LinkedList::clear(self);
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
        LinkedList::push_back(self, elt);
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
    push spec-push pre-push post-push
    *ENDOPNAME*/
    (define (spec-push xs x) (append xs (list x)))
    (define (pre-push xs) #t)
    (define (post-push xs x ys) (equal? ys (spec-push xs x)))
    *ENDLIBSPEC*/
    fn push(&mut self, elt: T) {
        LinkedList::push_back(self, elt);
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
        LinkedList::pop_back(self)
    }
}

/*IMPL*
WithPosition
*ENDIMPL*/
impl<T> WithPosition<T> for LinkedList<T> {
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
    fn first(&self) -> Option<&T> {
        LinkedList::front(self)
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
    fn last(&self) -> Option<&T> {
        LinkedList::back(self)
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
    fn nth(&self, n: usize) -> Option<&T> {
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

#[cfg(test)]
mod tests {
    use crate::traits::{Container, Stack, WithPosition};
    use crate::library::list::{Constructor, Con};
    use std::collections::LinkedList;

    #[test]
    fn test_list_container_trait() {
        //type Foo<T> = dyn Container<T>;
        //let list : &mut Foo<u32> = &mut LinkedList::<u32>::new();
        let list : &mut dyn Container<u32> = &mut LinkedList::<u32>::new();
        assert_eq!(list.len(), 0);
        list.insert(1);
        list.insert(4);
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove(9), None);
        assert_eq!(list.remove(1), Some(1));
        assert_eq!(list.len(), 1);
        assert!(list.contains(&4));
        list.clear();
        assert_eq!(list.len(), 0);
        //assert_eq!(list.pop_back(), None); // error
    }

    #[test]
    fn test_list_constructor() {
        let mut list = Con::<u32>::new();
        assert_eq!(list.len(), 0);
        list.insert(1);
        // assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_list_combo_trait() {
        trait ContainerStack<T> : Container<T> + Stack<T> {}
        impl<T: Ord> ContainerStack<T> for LinkedList<T> {}
        let list : &mut dyn ContainerStack<u32> = &mut LinkedList::<u32>::new();
        assert_eq!(list.len(), 0);
        list.insert(1);
        list.insert(4);
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove(9), None);
        assert_eq!(list.remove(1), Some(1));
        assert_eq!(list.len(), 1);
        assert!(list.contains(&4));
        list.clear();
        assert_eq!(list.len(), 0);
        //assert_eq!(list.pop_back(), None); // error
    }

    #[test]
    fn test_list_with_position() {
        trait ContainerWithPosition<T> : Container<T> + WithPosition<T> {}
        impl<T: Ord> ContainerWithPosition<T> for LinkedList<T> {}
        let list : &mut dyn ContainerWithPosition<u32> = &mut LinkedList::<u32>::new();
        list.insert(1);
        list.insert(4);
        list.insert(2);
        assert_eq!(list.first(), Some(&1));
        assert_eq!(list.last(), Some(&2));
        assert_eq!(list.nth(1), Some(&4));
    }
}