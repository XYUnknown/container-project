use std::any::{Any, TypeId};


pub enum Property {
    Unique,
    Sorted,
}

pub trait Container<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self);
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn contains(&self, x: &T) -> bool;
    fn is_empty(&self) -> bool;
}

pub trait Vector<T> : Container<T> {

}

pub fn type_of<T: ?Sized + Any>(_s: &T) -> TypeId {
    TypeId::of::<String>()
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
    use crate::container::{Property, type_of};

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
        let v = get_vec!(u32; Property::Unique);
        let v1 = UniqueVec::<u32>::new();
        assert!(v.is_empty());
        assert_eq!(type_of(&v), type_of(&v1));
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
}