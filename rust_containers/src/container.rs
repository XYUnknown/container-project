use std::any::{Any, TypeId};
use std::vec::Vec;
use crate::unique_vector::UniqueVec;
use crate::sorted_vector::SortedVec;

pub enum Property {
    Unique,
    Sorted,
}

pub trait Container<T> {
    fn c_push(&mut self, value: T);
    fn c_pop(&mut self) -> Option<T>;
    fn c_clear(&mut self);
    fn c_len(&self) -> usize;
    fn c_contains(&self, x: &T) -> bool;
    fn c_is_empty(&self) -> bool;
}

// pub trait Vector<T> : Container<T> {}

pub fn type_of<T: ?Sized + Any>(_s: &T) -> TypeId {
    TypeId::of::<T>()
}

impl<T: PartialEq> Container<T> for Vec<T> {
    fn c_push(&mut self, value: T) {
        self.push(value);
    }

    fn c_pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn c_clear(&mut self) {
        self.clear();
    }

    fn c_len(&self) -> usize {
        self.len()
    }

    fn c_contains(&self, x: &T) -> bool {
        self.contains(x)
    }

    fn c_is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: PartialEq> Container<T> for UniqueVec<T> {
    fn c_push(&mut self, value: T) {
        self.push(value);
    }

    fn c_pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn c_clear(&mut self) {
        self.clear();
    }

    fn c_len(&self) -> usize {
        self.len()
    }

    fn c_contains(&self, x: &T) -> bool {
        self.contains(x)
    }

    fn c_is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: PartialEq> Container<T> for SortedVec<T> {
    fn c_push(&mut self, value: T) {
        self.push(value);
    }

    fn c_pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn c_clear(&mut self) {
        self.clear();
    }

    fn c_len(&self) -> usize {
        self.len()
    }

    fn c_contains(&self, x: &T) -> bool {
        self.contains(x)
    }

    fn c_is_empty(&self) -> bool {
        self.is_empty()
    }
}

fn get_vec<T: 'static + Ord + PartialEq + Sized> (prop: Option<Property>) -> Box<dyn Container<T>> {
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
// experiment on macro
// get a vector according to specific property(-ies)
#[macro_export]
// this does not work, error: `match` arms have incompatible types
// uncomment to see the error
/*macro_rules! get_vec {
    ($t:ty) => { Vec::<$t>::new() }; // an ordinary vector
    ($t:ty; $p:expr) => {
        {
            match $p {
                Property::Unique => {
                    let vec = UniqueVec::<$t>::new();
                    vec
                },
                Property::Sorted => {
                    let vec = SortedVec::<$t>::new();
                    vec
                }
            }
        }
    };
}*/

// this works
macro_rules! get_vec {
    ($t:ty) => { Vec::<$t>::new() }; // an ordinary vector
    ($t:ty; Property::Unique) => { UniqueVec::<$t>::new() };
    ($t:ty; Property::Sorted) => { SortedVec::<$t>::new() };
    ($t:ty; Property::Unique + Property::Sorted) => { UniqueSortedVec::<$t>::new() };
    ($t:ty; Property::Sorted + Property::Unique) => { UniqueSortedVec::<$t>::new() };
}

#[cfg(test)]
mod tests {
    use crate::unique_vector::UniqueVec;
    use crate::sorted_vector::SortedVec;
    use crate::unique_sorted_vector::UniqueSortedVec;
    use crate::container::{Property, type_of, Container, get_vec};

    // test the type of
    #[test]
    fn test_type_of_works() {
        let v = Vec::<u32>::new();
        let v1 = UniqueVec::<u32>::new();
        let v2 = UniqueVec::<u32>::new();
        assert_eq!(type_of(&v1), type_of(&v2));
        assert_ne!(type_of(&v), type_of(&v1));
    }
    
    // macro get_vec test
    #[test]
    fn get_vec_works() {
        let v = get_vec!(u32);
        let v1 = Vec::<u32>::new();
        assert!(v.is_empty());
        assert_eq!(type_of(&v), type_of(&v1));
    }

    #[test]
    fn get_vec_unique_works() {
        let mut v = get_vec!(u32; Property::Unique);
        let v1 = UniqueVec::<u32>::new();
        let v2 = SortedVec::<u32>::new();
        assert!(v.is_empty());
        assert_eq!(type_of(&v), type_of(&v1));
        assert_ne!(type_of(&v), type_of(&v2));
        for x in 0..10000 {
            v.push(x);
            v.push(x);
        }
        assert_eq!(v.len(), 10000); // no duplication
    } 

    #[test]
    fn get_vec_sorted_works() {
        let v = get_vec!(u32; Property::Sorted);
        let v1 = SortedVec::<u32>::new();
        assert!(v.is_empty());
        assert_eq!(type_of(&v), type_of(&v1));
    }

    #[test]
    fn get_vec_unique_sorted_works() {
        let v = get_vec!(u32; Property::Unique + Property::Sorted);
        let v1 = UniqueSortedVec::<u32>::new();
        assert!(v.is_empty());
        assert_eq!(type_of(&v), type_of(&v1));
    }

    #[test]
    fn get_vec_unique_sorted_alter_works() {
        let v = get_vec!(u32; Property::Sorted + Property::Unique);
        let v1 = UniqueSortedVec::<u32>::new();
        assert!(v.is_empty());
        assert_eq!(type_of(&v), type_of(&v1));
    }

    #[test]
    fn get_unique_container() {
        let mut c = get_vec(Some(Property::Unique));
        let v = UniqueVec::<u32>::new();
        for x in 0..10000 {
            c.c_push(x);
            c.c_push(x);
        }
        assert_eq!(c.c_len(), 10000); // no duplication
        assert_ne!(type_of(&c), type_of(&v));
    }
}