use std::vec::Vec;
use std::collections::LinkedList;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;
use std::cmp::Ordering;
use std::collections::linked_list::Cursor;
use std::collections::linked_list::CursorMut;
use crate::container_specialization::property::*;

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

pub struct VecWrapper<T, P: ?Sized> {
    v: Vec<T>,
    property: PhantomData<P> // just a marker
}

impl<T: PartialEq, P: ?Sized> VecWrapper<T, P> {
    pub const fn new() -> VecWrapper<T, P> {
        VecWrapper { v: Vec::new(), property: PhantomData, }
    }
}

impl<T, P: ?Sized> Deref for VecWrapper<T, P> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T, P: ?Sized> DerefMut for VecWrapper<T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
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

impl<T: PartialEq> Container<T, dyn Unique<S=T>> for VecWrapper<T, dyn Unique<S=T>> {
    fn push(&mut self, value: T) {
        self.v.unique_push(value);
    }
}

impl<T: PartialEq + Ord> Container<T, dyn Sorted<S=T, R=usize>> for VecWrapper<T, dyn Sorted<S=T, R=usize>> {
    fn push(&mut self, value: T) {
        self.v.sorted_push(value);
    }
}

/*
impl<T: PartialEq + Ord> Container<T, dyn And<dyn Sorted, dyn Unique<T=T>>> for VecWrapper<T, dyn And<dyn Sorted, dyn Unique<T=T>>> {
    fn push(&mut self, value: T) {
        if !self.v.contains(&value) {
            let index = self.v.binary_search(&value).unwrap_or_else(|i| i);
            self.v.insert(index, value);
        }
    }
}*/

fn get_vec<T: 'static + PartialEq + Ord + Sized, P: 'static + ?Sized> () -> Box<dyn Container<T, P>> {
    let vec = Box::new(VecWrapper::<T, P>::new());
    vec
}

#[cfg(test)]
mod tests {
    use std::vec::Vec;
    use crate::container_specialization::container_mini::{get_vec, VecWrapper};
    use crate::container_specialization::property::*;

    #[test]
    fn vec_specialize_unique_works() {
        let mut c = VecWrapper::<u32, dyn Unique<S=u32>>::new();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn get_vec_specialize_unique_works() {
        let mut c = get_vec::<u32, dyn Unique<S=u32>>();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }

    #[test]
    fn vec_specialize_sorted_works() {
        let mut c = VecWrapper::<u32, dyn Sorted<S=u32, R=usize>>::new();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
            c.push(4 - x);
        }
        assert_eq!(*c, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[test]
    fn get_vec_specialize_sorted_works() {
        let mut c = get_vec::<u32, dyn Sorted<S=u32, R=usize>>();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
            c.push(4 - x);
        }
        // increasing
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(4));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(3));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(2));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(1));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), Some(0));
        assert_eq!(c.pop(), None);
    }
}