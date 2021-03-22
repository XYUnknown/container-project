// The push property for Vec
// We would like to inspect how the composition of properties works

use std::vec::Vec;
use std::marker::PhantomData;

pub trait PushProperty<T> {
    type R;
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
        true // for now
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

pub struct And<P1, P2> {}

/*impl<T: PartialEq + Ord, P1: PushProperty<T>, PushProperty<T>> PushProperty<T> for And <P1, P2> {

}*/


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
}