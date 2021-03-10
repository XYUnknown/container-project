//use crate::container_specialization::container::{Container};
use std::vec::Vec;
pub trait Property {}
pub trait Unique : Property {
    type S;
    fn u_test(&self, value: &Self::S) -> bool;
    fn u_exec(&mut self, value: Self::S);
    //fn assert(&mut self);

    fn unique_push(&mut self, value: Self::S) {
        if !self.u_test(&value) {
            self.u_exec(value);
        }
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
    //fn assert(&mut self);

    fn sorted_push(&mut self, value: Self::S) {
        let loc = self.s_find(&value);
        self.s_exec(loc, value);
    }
}

impl<T: PartialEq> Property for Vec<T> {}

impl<T: PartialEq> Unique for Vec<T> {
    type S = T;
    fn u_test(&self, value: &T) -> bool {
        self.contains(&value)
    }

    fn u_exec(&mut self, value: Self::S) {
        self.push(value);
    }
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
}

pub trait And<P1: Property, P2: Property> : Property{}

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
}