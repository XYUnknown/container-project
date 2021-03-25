//use crate::container_specialization::container::{Container};
use std::vec::Vec;
use std::ops::{Deref, DerefMut};
use std::marker::PhantomData;

pub trait Property {}
pub trait Unique : Property {
    type S;
    fn u_test(&self, value: &Self::S) -> bool;
    fn u_exec(&mut self, cond: bool, value: Self::S);
    // fn u_assert(&self) -> bool;

    fn unique_push(&mut self, value: Self::S) {
        let cond = self.u_test(&value);
        self.u_exec(cond, value);
        //assert!(self.u_assert());
    }

    /* Something more appealing but not work
    fn unique_push(&mut self, value: Self::T, t: &dyn Fn(&mut Self, Self::T) -> bool, f: &dyn Fn(&mut Self, Self::T)) {
        if !self.t(value) {
            self.f(value);
        }
    }*/
}

pub trait Sorted : Property {
    type S;
    type R;
    fn s_find(&self, value: &Self::S) -> Self::R;
    fn s_exec(&mut self, location: Self::R, value: Self::S);
    fn s_assert(&self) -> bool;

    fn sorted_push(&mut self, value: Self::S) {
        // assert!(self.s_assert());
        let loc = self.s_find(&value);
        self.s_exec(loc, value);
        assert!(self.s_assert());
    }
}

impl<T: PartialEq> Property for Vec<T> {}

impl<T: PartialEq> Unique for Vec<T> {
    type S = T;
    fn u_test(&self, value: &T) -> bool {
        !self.contains(&value)
    }

    fn u_exec(&mut self, cond: bool, value: Self::S) {
        if cond {
            self.push(value);
        }
    }

    /*fn u_assert(&self) -> bool {
        let mut result = true;
        self.iter().for_each(|(x, y)| {
            result = (x != y);
        });
        result
    }*/
}

impl<T: PartialEq+Ord> Sorted for Vec<T> {
    type S = T;
    type R = usize;
    fn s_find(&self, value: &T) -> Self::R {
        self.binary_search(&value).unwrap_or_else(|i| i)
    }

    fn s_exec(&mut self, location: Self::R, value: Self::S) {
        self.insert(location, value);
    }

    fn s_assert(&self) -> bool {
        self.iter().is_sorted()
    }
}

pub trait And<P1: Property, P2: Property> : Property {}

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

impl<T: PartialEq> Container<T, dyn Unique<S=T>> for VecWrapper<T, dyn Unique<S=T>>
{
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
    use crate::container_specialization::property::*;

    #[test]
    fn test_unique_prop_push_works() {
        let mut v = Vec::new();
        for x in 0..100 {
            v.unique_push(x);
            v.unique_push(x);
        }
        assert_eq!(v.len(), 100);
    }

    #[test]
    fn test_sorted_prop_push_works() {
        let mut v = Vec::new();
        for x in 0..5 {
            v.sorted_push(4 - x);
            v.sorted_push(4 - x);
        }
        assert_eq!(*v, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
    }

        /*#[test]
    fn vec_specialize_unique_works() {
        let mut c = VecWrapper::<u32, dyn Unique<S=u32>>::new();
        assert!(c.is_empty());
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100); // no duplication
    }*/

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

    /*#[test]
    fn vec_specialize_sorted_works() {
        let mut c = VecWrapper::<u32, dyn Sorted<S=u32, R=usize>>::new();
        assert!(c.is_empty());
        for x in 0..5 {
            c.push(4 - x);
            c.push(4 - x);
        }
        assert_eq!(*c, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
    }*/

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