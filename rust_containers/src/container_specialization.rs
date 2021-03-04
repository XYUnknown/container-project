use std::vec::Vec;
use std::ops::Deref;
use std::marker::PhantomData;

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

pub trait Vector<T, P: ?Sized> : Container<T, P> {
    fn get(&mut self, index: usize) -> Option<&T>;
    fn remove(&mut self, index: usize) -> T;
    // issue: insert is not meaningful for sored vector
    // fn v_insert(&mut self, index: usize, element: T);
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

// just for testing purposes
impl<T, P: ?Sized> Deref for VecWrapper<T, P> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
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

//impl<T: PartialEq, P: ?Sized> Vector<T, P> for VecWrapper<T, P> {
//
//}

fn get_vec<T: 'static + PartialEq + Ord + Sized, P: 'static + ?Sized> () -> Box<dyn Container<T, P>> {
    let vec = Box::new(VecWrapper::<T, P>::new());
    vec
}

#[cfg(test)]
mod tests {
    use crate::container_specialization::{Container, Unique, Sorted, UniqueSorted, VecWrapper, get_vec };

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
