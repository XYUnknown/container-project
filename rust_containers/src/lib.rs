#![feature(const_fn)] // enabling impl<T: Ord> 
#![allow(unused)]

mod unique_vector;
mod sorted_vector;
mod unique_sorted_vector;

#[macro_export]
// UniqueVec creations
macro_rules! unique_vec { // e.g., unique_vec![1, 2, 3]
    ($($x:expr),*) => {
        {
            let mut vec = UniqueVec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
    ($elem:expr; $n:expr) => { // e.g., unique_vec![1; 3]
        {
            let mut vec = UniqueVec::new();
            vec.push($elem);
            vec
        }
    };
}

// SortedVec creations
macro_rules! sorted_vec {
    ($($x:expr),*) => { // e.g., sorted_vec![1, 2, 3]
        {
            let mut vec = SortedVec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
    ($elem:expr; $n:expr) => { // e.g., sorted_vec![1; 3]
        {
            let mut src = std::vec::from_elem($elem, $n);
            let mut vec = SortedVec::from_vec(&mut src);
            vec
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::unique_vector::UniqueVec;
    use crate::sorted_vector::SortedVec;
    use std::cmp::Reverse;
    //use crate::unique_sorted_vector::UniqueSortedVec;
    #[test]
    fn unique_creation_works() {
        let vec = UniqueVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_creation_with_capacity_works() {
        let vec = UniqueVec::<u32>::with_capacity(10);
        assert_eq!(vec.capacity(), 10);
    }

    #[test]
    fn unique_creation_from_vec() {
        let mut src = vec![1, 2, 3];
        let vec = UniqueVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3]);
    }

    #[test]
    fn unique_contains_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        assert_eq!(vec.contains(&1), true);
        assert_eq!(vec.contains(&2), false);
    }

    #[test]
    fn unique_push_works() {
        let mut vec = UniqueVec::new();
        for x in 0..10000 {
            vec.push(x);
            vec.push(x);
        }
        assert_eq!(vec.len(), 10000); // no duplication
    }

    #[test]
    fn unique_pop_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn unique_pop_some_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
    }

    #[test]
    fn unique_insert_works() {
        let mut vec = UniqueVec::new();
        vec.insert(0, 10);
        vec.insert(1, 10);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_remove_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.remove(0), 1);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_truncate_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        vec.truncate(3);
        assert_eq!(*vec, [0, 1, 2]);
    }

    #[test]
    fn unique_clear_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_first_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.first(), None);
    }

    #[test]
    fn unique_first_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.first(), Some(&0));
    }

    #[test]
    fn unique_last_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.last(), None);
    }

    #[test]
    fn unique_last_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.last(), Some(&4));
    }

    #[test]
    fn unique_get_element_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.get(3), Some(&3));
    }

    #[test]
    fn unique_get_slice_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.get(0..2), Some(&[0, 1][..]));
    }

    #[test]
    fn unique_macro_one_works() {
        let mut vec = unique_vec![1, 1];
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_macro_two_works() {
        let mut vec = unique_vec![1; 4];
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_append_works() {
        let mut vec = UniqueVec::new();
        let mut other = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn sorted_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let vec = SortedVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3, 3]);
    }

    fn sorted_macro_one_works() {
        let vec = sorted_vec![3, 7, 2, 1, 5, 4, 3];
        assert_eq!(*vec, [1, 2, 3, 3, 4, 5, 7])
    }

    fn sorted_macro_two_works() {
        let vec = sorted_vec![1; 3];
        assert_eq!(*vec, [1, 1, 1])
    }

    #[test]
    fn sorted_contains_works() {
        let mut vec = SortedVec::<u32>::new();
        assert_eq!(vec.contains(&1), false);
    }

    #[test]
    fn sorted_push_works() {
        let mut vec = SortedVec::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(*vec, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn sorted_append_works() {
        let mut vec = SortedVec::new();
        let mut other = SortedVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 2, 3, 3, 4, 4, 5, 6])
    }

}