use std::vec::Vec;
use std::ops::Deref;
use std::marker::PhantomData;

// Properties as marker traits
pub trait Unique {}
pub trait Sorted {}
pub trait UniqueSorted: Unique + Sorted {}

pub trait Container<T, P: ?Sized> {
    fn c_push(&mut self, value: T);
    fn c_pop(&mut self) -> Option<T>;
    fn c_clear(&mut self);
    fn c_len(&self) -> usize;
    fn c_contains(&self, x: &T) -> bool;
    fn c_is_empty(&self) -> bool;
    fn c_first(&mut self) -> Option<&T>;
    fn c_last(&mut self) -> Option<&T>;
}

impl<T: PartialEq, P: ?Sized> Container<T, P> for Vec<T> {
    default fn c_push(&mut self, value: T) {
        self.push(value);
    }

    default fn c_pop(&mut self) -> Option<T> {
        self.pop()
    }

    default fn c_clear(&mut self) {
        self.clear();
    }

    default fn c_len(&self) -> usize {
        self.len()
    }

    default fn c_contains(&self, x: &T) -> bool {
        self.contains(x)
    }

    default fn c_is_empty(&self) -> bool {
        self.is_empty()
    }

    default fn c_first(&mut self) -> Option<&T> {
        self.first()
    }

    default fn c_last(&mut self) -> Option<&T> {
        self.last()
    }
}

impl<T: PartialEq> Container<T, dyn Unique> for Vec<T> {
    fn c_push(&mut self, value: T) {
        if !self.contains(&value) {
            self.push(value);
        }
    }
}

impl<T: PartialEq + Ord> Container<T, dyn Sorted> for Vec<T> {
    fn c_push(&mut self, value: T) {
        let index = self.binary_search(&value).unwrap_or_else(|i| i);
        self.insert(index, value);
    }
}

impl<T: PartialEq + Ord> Container<T, dyn UniqueSorted> for Vec<T> {
    fn c_push(&mut self, value: T) {
        if !self.contains(&value) {
            let index = self.binary_search(&value).unwrap_or_else(|i| i);
            self.insert(index, value);
        }
    }
}

fn get_vec<T: 'static + PartialEq + Ord + Sized, P: 'static + ?Sized> () -> Box<dyn Container<T, P>> {
    let vec = Box::new(Vec::<T>::new());
    vec
}

#[cfg(test)]
mod tests {
    use crate::container_specialization_no_wrapper::{Container, Unique, Sorted, UniqueSorted, get_vec };

    #[test]
    fn get_vec_no_wrapper_works() {
        let mut c = get_vec::<u32, ()>();
        assert!(c.c_is_empty());
        for x in 0..100 {
            c.c_push(x);
            c.c_push(x);
        }
        assert_eq!(c.c_len(), 200); // duplication allowed
    }

    #[test]
    fn get_vec_no_wrapper_unique_works() {
        let mut c = get_vec::<_, dyn Unique>();
        assert!(c.c_is_empty());
        for x in 0..100 {
            c.c_push(x);
            c.c_push(x);
        }
        assert_eq!(c.c_len(), 100); // no duplication
    }

    #[test]
    fn get_vec_no_wrapper_sorted_works() {
        let mut c = get_vec::<_, dyn Sorted>();
        assert!(c.c_is_empty());
        for x in 0..5 {
            c.c_push(4 - x);
        }
        // increasing
        assert_eq!(c.c_pop(), Some(4));
        assert_eq!(c.c_pop(), Some(3));
        assert_eq!(c.c_pop(), Some(2));
        assert_eq!(c.c_pop(), Some(1));
        assert_eq!(c.c_pop(), Some(0));
        assert_eq!(c.c_pop(), None);
    }

    #[test]
    fn get_vec_no_wrapper_unique_sorted_works() {
        let mut c = get_vec::<_, dyn UniqueSorted>();
        assert!(c.c_is_empty());
        for x in 0..5 {
            c.c_push(4 - x);
        }
        // increasing
        assert_eq!(c.c_pop(), Some(4));
        assert_eq!(c.c_pop(), Some(3));
        assert_eq!(c.c_pop(), Some(2));
        assert_eq!(c.c_pop(), Some(1));
        assert_eq!(c.c_pop(), Some(0));
        assert_eq!(c.c_pop(), None);

        for x in 0..100 {
            c.c_push(x);
            c.c_push(x);
        }
        assert_eq!(c.c_len(), 100); // no duplication
    }
}

