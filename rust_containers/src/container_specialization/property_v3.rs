use std::vec::Vec;
use std::marker::PhantomData;

pub trait PushProperty<T>  {
    fn post(vec: &mut Vec<T>);
    fn assert(vec: &Vec<T>) -> bool;
  
    fn p_push(vec: &mut Vec<T>, value: T) {
        vec.push(value);
        Self::post(vec);
        assert!(Self::assert(vec));
    }
}

pub struct Unique {}

impl<T: PartialEq> PushProperty<T> for Unique {
    fn post(vec: &mut Vec<T>) { // a very inefficient implementation
        for i in 0..(vec.len() - 1) {
            let index = vec.len() - 1 - i;
            if vec[0..index].contains(&vec[index]) {
                vec.remove(index);
            }
        }
    }

    fn assert(vec: &Vec<T>) -> bool {
        !(1..vec.len()).any(|i| vec[i..].contains(&vec[i - 1]))
    }
}

pub struct Sorted {}

impl<T: PartialEq + Ord> PushProperty<T> for Sorted {
    fn post(vec: &mut Vec<T>) {
        vec.sort();
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

    fn post(vec: &mut Vec<T>) {
        P1::post(vec);
        P2::post(vec); 
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
    use crate::container_specialization::property_v3::*;

    #[test]
    fn test_v3_vec_unique_prop_works() {
        let mut vec = VecWrapper::<u32, Unique>::new();
        for x in 0..5 {
            vec.push(x);
            vec.push(x);
        }
        assert_eq!(vec.v.len(), 5);
    }

    #[test]
    fn test_v3_vec_sorted_prop_works() {
        let mut vec = VecWrapper::<u32, Sorted>::new();
        for x in 0..5 {
            vec.push(4 - x);
            vec.push(4 - x);
        }
        assert_eq!(vec.v, [0, 0, 1, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[test]
    fn test_v3_vec_unique_sorted_prop_works() {
        let mut vec = VecWrapper::<u32, And<Unique, Sorted>>::new();
        for x in 0..5 {
            vec.push(4 - x);
            vec.push(4 - x);
        }
        assert_eq!(vec.v, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_v3_vec_sorted_unique_prop_works() {
        let mut vec = VecWrapper::<u32, And<Sorted, Unique>>::new();
        for x in 0..5 {
            vec.push(4 - x);
            vec.push(4 - x);
        }
        assert_eq!(vec.v, [0, 1, 2, 3, 4]);
    }
}