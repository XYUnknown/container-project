//use crate::container_specialization::container::{Container};
use std::vec::Vec;
pub trait Property {}
pub trait Unique : Property {
    type T;
    fn test(&self, value: &Self::T) -> bool;
    fn exec(&mut self, value: Self::T);
    //fn assert(&mut self);

    fn unique_push(&mut self, value: Self::T) {
        if !self.test(&value) {
            self.exec(value);
        }
    }

    /* Something more appealing but not work
    fn unique_push(&mut self, value: Self::T, t: &dyn Fn(&mut Self, Self::T) -> bool, f: &dyn Fn(&mut Self, Self::T)) {
        if !self.t(value) {
            self.f(value);
        }
    }*/
}

impl<T: PartialEq> Property for Vec<T> {}

impl<T: PartialEq> Unique for Vec<T> {
    type T = T;
    fn test(&self, value: &T) -> bool {
        self.contains(&value)
    }

    fn exec(&mut self, value: Self::T) {
        self.push(value);
    }
}

pub trait Sorted : Property {}
pub trait And<P1: Property, P2: Property> : Property + Property {}

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

}