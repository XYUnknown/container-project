use crate::container_naive::container::{Property, type_of, Container};
use std::vec::Vec;
use crate::container_library::unique_vector::UniqueVec;
use crate::container_library::sorted_vector::SortedVec;

pub trait Vector<T> : Container<T> {
    fn v_get(&mut self, index: usize) -> Option<&T>;
    fn v_remove(&mut self, index: usize) -> T;
    // issue: insert is not meaningful for sored vector
    // fn v_insert(&mut self, index: usize, element: T);
}

impl<T: PartialEq> Vector<T> for Vec<T> {
    fn v_get(&mut self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn v_remove(&mut self, index: usize) -> T {
        self.remove(index)
    }
}

impl<T: PartialEq> Vector<T> for UniqueVec<T> {
    fn v_get(&mut self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn v_remove(&mut self, index: usize) -> T {
        self.remove(index)
    }
}

impl<T: PartialEq> Vector<T> for SortedVec<T> {
    fn v_get(&mut self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn v_remove(&mut self, index: usize) -> T {
        self.remove(index)
    }
}

fn get_vec<T: 'static + Ord + PartialEq + Sized> (prop: Option<Property>) -> Box<dyn Vector<T>> {
    match prop {
        Some(p) => {
            match p {
                Property::Unique => {
                    let vec = Box::new(UniqueVec::<T>::new());
                    vec
                },
                Property::Sorted => {
                    let vec = Box::new(SortedVec::<T>::new());
                    vec
                }
            }
        },
        None => {
            let vec = Box::new(SortedVec::<T>::new());
            vec
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::container_naive::sub_containers::{get_vec};
    #[test]
    fn get_vec_container() {
        let mut c = get_vec(None);
        c.c_push(2);
        assert_eq!(c.c_len(), 1);
        assert_eq!(c.v_get(0), Some(&2));
    }
}