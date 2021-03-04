use std::vec::Vec;
use std::collections::LinkedList;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use std::cmp::Ordering;
use std::collections::linked_list::Cursor;
use std::collections::linked_list::CursorMut;

// Properties as marker traits
pub trait Unique {}
pub trait Sorted {}
pub trait UniqueSorted: Unique + Sorted {}

pub trait Container<T, P: ?Sized> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn contains(&self, x: &T) -> bool;
    fn is_empty(&self) -> bool;
    fn first(&mut self) -> Option<&T>;
    fn last(&mut self) -> Option<&T>;
}

pub trait VectorT<T, P: ?Sized> : Container<T, P> {
    fn get(&mut self, index: usize) -> Option<&T>;
    fn remove(&mut self, index: usize) -> T;
    // issue: insert is not meaningful for sored vector
    // fn v_insert(&mut self, index: usize, element: T);
}

pub trait LinkedListT<T, P: ?Sized> : Container<T, P> {
    fn push_front(&mut self, elt: T);
    fn pop_front(&mut self) -> Option<T>;
}

pub struct VecWrapper<T, P: ?Sized> {
    v: Vec<T>,
    property: PhantomData<P> // just a marker
}

impl<T: PartialEq, P: ?Sized> VecWrapper<T, P> {
    pub const fn new() -> VecWrapper<T, P> {
        VecWrapper { v: Vec::new(), property: PhantomData, }
    }
}

pub struct LinkedListWrapper<T, P: ?Sized> {
    ll: LinkedList<T>,
    property: PhantomData<P>
}

impl<T: PartialEq, P: ?Sized> LinkedListWrapper<T, P> {
    pub const fn new() -> LinkedListWrapper<T, P> {
        LinkedListWrapper { ll: LinkedList::new(), property: PhantomData }
    }
}

// just for testing purposes
impl<T, P: ?Sized> Deref for VecWrapper<T, P> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T, P: ?Sized> Deref for LinkedListWrapper<T, P> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll
    }
}

impl<T, P: ?Sized> DerefMut for LinkedListWrapper<T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ll
    }
}

impl<T: PartialEq, P: ?Sized> Container<T, P> for VecWrapper<T, P> {
    default fn push(&mut self, value: T) {
        self.v.push(value);
    }

    default fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    default fn clear(&mut self) {
        self.v.clear();
    }

    default fn len(&self) -> usize {
        self.v.len()
    }

    default fn contains(&self, x: &T) -> bool {
        self.v.contains(x)
    }

    default fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    default fn first(&mut self) -> Option<&T> {
        self.v.first()
    }

    default fn last(&mut self) -> Option<&T> {
        self.v.last()
    }
}

impl<T: PartialEq> Container<T, dyn Unique> for VecWrapper<T, dyn Unique> {
    fn push(&mut self, value: T) {
        if !self.v.contains(&value) {
            self.v.push(value);
        }
    }
}

impl<T: PartialEq + Ord> Container<T, dyn Sorted> for VecWrapper<T, dyn Sorted> {
    fn push(&mut self, value: T) {
        let index = self.v.binary_search(&value).unwrap_or_else(|i| i);
        self.v.insert(index, value);
    }
}

impl<T: PartialEq + Ord> Container<T, dyn UniqueSorted> for VecWrapper<T, dyn UniqueSorted> {
    fn push(&mut self, value: T) {
        if !self.v.contains(&value) {
            let index = self.v.binary_search(&value).unwrap_or_else(|i| i);
            self.v.insert(index, value);
        }
    }
}

impl<T: PartialEq, P: ?Sized> Container<T, P> for LinkedListWrapper<T, P> {
    default fn push(&mut self, value: T) {
        self.ll.push_back(value);
    }

    default fn pop(&mut self) -> Option<T> {
        self.ll.pop_back()
    }

    default fn clear(&mut self) {
        self.ll.clear();
    }

    default fn len(&self) -> usize {
        self.ll.len()
    }

    default fn contains(&self, x: &T) -> bool {
        self.ll.contains(x)
    }

    default fn is_empty(&self) -> bool {
        self.ll.is_empty()
    }

    default fn first(&mut self) -> Option<&T> {
        self.ll.front()
    }

    default fn last(&mut self) -> Option<&T> {
        self.ll.back()
    }
}

impl<T: PartialEq> Container<T, dyn Unique> for LinkedListWrapper<T, dyn Unique> {
    fn push(&mut self, value: T) {
        if !self.ll.contains(&value) {
            self.ll.push_back(value);
        }
    }
}

impl<T: Ord + PartialEq> Container<T, dyn Sorted> for LinkedListWrapper<T, dyn Sorted> {
    fn push(&mut self, elt: T) {
        let mut c = self.cursor_back_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less => {
                            c.move_prev();
                        },
                        Ordering::Greater | Ordering::Equal => {
                            c.insert_after(elt);
                            break;
                        } 
                    }
                },
                None => { // empty list
                    c.insert_after(elt);
                    break;
                }
            }
        }
    }
}

impl<T: Ord + PartialEq> Container<T, dyn UniqueSorted> for LinkedListWrapper<T, dyn UniqueSorted> {
    fn push(&mut self, elt: T) {
        let mut c = self.cursor_back_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less => {
                            c.move_prev();
                        },
                        Ordering::Greater => {
                            c.insert_after(elt);
                            break;
                        }
                        Ordering::Equal => break
                    }
                },
                None => { // empty list
                    c.insert_after(elt);
                    break;
                }
            }
        }
    }
}

impl<T: PartialEq, P: ?Sized> VectorT<T, P> for VecWrapper<T, P> {
    default fn get(&mut self, index: usize) -> Option<&T> {
        self.v.get(index)
    }
    
    default fn remove(&mut self, index: usize) -> T {
        self.v.remove(index)
    }
}

impl<T: PartialEq, P: ?Sized> LinkedListT<T, P> for LinkedListWrapper<T, P> {
    default fn push_front(&mut self, elt: T) {
        self.ll.push_front(elt);
    }

    default fn pop_front(&mut self) -> Option<T> {
        self.ll.pop_front()
    }
}

impl<T: Ord + PartialEq> LinkedListT<T, dyn Sorted> for LinkedListWrapper<T, dyn Sorted> {
    fn push_front(&mut self, elt: T) {
        let mut c = self.cursor_front_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less | Ordering::Equal => {
                            c.insert_before(elt);
                            break;
                        },
                        Ordering::Greater => c.move_next()
                    }
                },
                None => { // empty list
                    c.insert_before(elt);
                    break;
                }
            }
        }
    }
}

impl<T: Ord + PartialEq> LinkedListT<T, dyn UniqueSorted> for LinkedListWrapper<T, dyn UniqueSorted> {
    fn push_front(&mut self, elt: T) {
        let mut c = self.cursor_front_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less => {
                            c.insert_before(elt);
                            break;
                        },
                        Ordering::Equal => break,
                        Ordering::Greater => c.move_next()
                    }
                },
                None => { // empty list
                    c.insert_before(elt);
                    break;
                }
            }
        }
    }
}

fn get_vec<T: 'static + PartialEq + Ord + Sized, P: 'static + ?Sized> () -> Box<dyn Container<T, P>> {
    let vec = Box::new(VecWrapper::<T, P>::new());
    vec
}

fn get_list<T: 'static + PartialEq + Ord + Sized, P: 'static + ?Sized> () -> Box<dyn Container<T, P>> {
    let ll = Box::new(LinkedListWrapper::<T, P>::new());
    ll
}

#[cfg(test)]
mod tests {
    use crate::container_specialization::container::{Container, Unique, Sorted, UniqueSorted, VecWrapper, LinkedListWrapper, VectorT, get_vec, get_list };

    #[test]
    fn vec_specialize_works() {
        let mut c = VecWrapper::<u32, ()>::new();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 200); // duplication allowed
    }

    #[test]
    fn vec_specialize_unique_works() {
        let mut c = VecWrapper::<u32, dyn Unique>::new();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn vec_specialize_sorted_works() {
        let mut c = VecWrapper::<u32, dyn Sorted>::new();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        assert_eq!(*c, [0, 1, 2, 3, 4]); // increasing
    }

    #[test]
    fn vec_specialize_unique_sorted_works() {
        let mut c = VecWrapper::<u32, dyn UniqueSorted>::new();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        assert_eq!(*c, [0, 1, 2, 3, 4]); // increasing
        c.clear();
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn get_vec_specialize_works() {
        let mut c = get_vec::<u32, ()>();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 200); // duplication allowed
    }

    #[test]
    fn get_vec_specialize_unique_works() {
        let mut c = get_vec::<u32, dyn Unique>();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn get_vec_specialize_sorted_works() {
        let mut c = get_vec::<u32, dyn Sorted>();
        //let mut c = get_vec::<_, dyn Sorted>(); also valid
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
    }

    #[test]
    fn get_vec_specialize_unique_sorted_works() {
        let mut c = get_vec::<_, dyn UniqueSorted>();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn ll_specialize_works() {
        let mut c = LinkedListWrapper::<u32, ()>::new();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 200); // duplication allowed
    }

    #[test]
    fn ll_specialize_unique_works() {
        let mut c = LinkedListWrapper::<u32, dyn Unique>::new();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn ll_specialize_sorted_works() {
        let mut c = LinkedListWrapper::<_, dyn Sorted>::new();
        assert!(c.is_empty());
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
    }

    #[test]
    fn ll_specialize_unique_sorted_works() {
        let mut c = LinkedListWrapper::<_, dyn UniqueSorted>::new();
        assert!(c.is_empty());
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn get_list_specialize_works() {
        let mut c = get_list::<u32, ()>();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 200); // duplication allowed
    }

    #[test]
    fn get_list_specialize_unique_works() {
        let mut c = get_list::<u32, dyn Unique>();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn get_list_specialize_sorted_works() {
        let mut c = get_list::<u32, dyn Sorted>();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
    }

    #[test]
    fn get_list_specialize_unique_sorted_works() {
        let mut c = get_list::<_, dyn UniqueSorted>();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }


}

/* INVALID
struct Foo<A, B>(A, B);

impl<A> Foo<A,A> {
    fn foo(&self, _: u32) {}
}

impl<A,B> Foo<A,B> {
    fn foo(&self, _: bool) {}
}

fn use_foo<A, B>(f: Foo<A,B>) {
    f.foo(true)
}*/
