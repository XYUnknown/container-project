// The push property for Vec
// We would like to inspect how the composition of properties works
use std::vec::Vec;
use std::marker::PhantomData;
use std::any::{Any, TypeId};

pub fn type_of<T: ?Sized + Any>(_s: &T) -> TypeId {
    TypeId::of::<T>()
}

pub trait PushProperty<T> {
    type R : 'static;
    fn pre(vec: &Vec<T>, value: &T) -> Self::R;
    fn exec(vec: &mut Vec<T>, cond: Self::R, value: T); 
    fn assert(vec: &Vec<T>) -> bool;
  
    fn p_push(vec: &mut Vec<T>, value: T) {
        let cond = Self::pre(vec, &value);
        Self::exec(vec, cond, value);
        assert!(Self::assert(vec));
    }
}

pub struct Unique {}

impl<T: PartialEq> PushProperty<T> for Unique {
    type R = bool;
    fn pre(vec: &Vec<T>, value: &T) -> bool {
        !vec.contains(value)
    }

    fn exec(vec: &mut Vec<T>, cond: bool, value: T) {
        if cond {
            vec.push(value);
        }
    }

    fn assert(vec: &Vec<T>) -> bool {
        !(1..vec.len()).any(|i| vec[i..].contains(&vec[i - 1]))
    }
}

pub struct Default {}

impl<T> PushProperty<T> for Default {
    type R = ();
    fn pre(vec: &Vec<T>, value: &T) -> () {
        ()
    }

    fn exec(vec: &mut Vec<T>, cond: (), value: T) {
        vec.push(value);
    }

    fn assert(vec: &Vec<T>) -> bool {
        true
    }
}

pub struct Sorted {}

impl<T: PartialEq + Ord> PushProperty<T> for Sorted {
    type R = usize;
    fn pre(vec: &Vec<T>, value: &T) -> usize {
        vec.binary_search(&value).unwrap_or_else(|i| i)
    }

    fn exec(vec: &mut Vec<T>, cond: usize, value: T) {
        vec.insert(cond, value);
    }

    fn assert(vec: &Vec<T>) -> bool {
        vec.iter().is_sorted()
    }
}

pub struct And<P1, P2> {
    property1: PhantomData<P1>,
    property2: PhantomData<P2> 
}

impl<T: PartialEq + Ord, P1: PushProperty<T>, P2: PushProperty<T>> PushProperty<T> for And <P1, P2> {
    type R = (P1::R, P2::R);

    fn pre(vec: &Vec<T>, value: &T) -> (P1::R, P2::R) {
        let r1 = P1::pre(vec, value);
        let r2 = P2::pre(vec, value);
        (r1, r2)
    }

    fn exec(vec: &mut Vec<T>, cond: (P1::R, P2::R), value: T) {
        //how to specify the execution satisfying/according to both conditions?
        
    }

    fn assert(vec: &Vec<T>) -> bool {
        let a1 = P1::assert(vec);
        let a2 = P2::assert(vec);
        a1 && a2
    }
}

pub struct VecWrapper<T, P> {
    v: Vec<T>,
    property: PhantomData<P> 
}

impl<T: PartialEq, P: PushProperty<T>> VecWrapper<T, P> {
    pub const fn new() -> VecWrapper<T, P> {
        VecWrapper { v: Vec::new(), property: PhantomData }
    }

    pub fn push(&mut self, value: T) {
        P::p_push(&mut self.v, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::container_specialization::property_v2::*;

    #[test]
    fn test_vec_unique_prop_works() {
        let mut vec = VecWrapper::<u32, Unique>::new();
        for x in 0..100 {
            vec.push(x);
            vec.push(x);
        }
        assert_eq!(vec.v.len(), 100);
    }

    #[test]
    fn test_vec_sorted_prop_works() {
        let mut vec = VecWrapper::<u32, Sorted>::new();
        for x in 0..5 {
            vec.push(4 - x);
            vec.push(4 - x);
        }
        assert_eq!(vec.v, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
    }

    /*#[test]
    fn test_vec_unique_sorted_prop_works() {
        let mut vec = VecWrapper::<u32, And<Unique, Sorted>>::new();
        for x in 0..5 {
            vec.push(4 - x);
            vec.push(4 - x);
        }
        assert_eq!(vec.v, [0, 1, 2, 3, 4]);
    }*/
}