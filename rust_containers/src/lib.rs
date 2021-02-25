#![feature(const_fn)] // enabling impl<T: Ord> 
#![allow(unused)]
#![feature(linked_list_cursors)]
#![feature(test)]

pub mod unique_vector;
pub mod sorted_vector;
pub mod unique_sorted_vector;
pub mod unique_linked_list;
pub mod sorted_linked_list;
pub mod unique_sorted_linked_list;
pub mod binary_search_tree;
pub mod container;

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

// UniqueSortedVec creations
macro_rules! unique_sorted_vec {
    ($($x:expr),*) => { // e.g., unique_sorted_vec![1, 2, 3]
        {
            let mut vec = UniqueSortedVec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
    ($elem:expr; $n:expr) => { // e.g., unique_sorted_vec![1; 3]
        {
            let mut vec = UniqueSortedVec::new();
            vec.push($elem);
            vec
        }
    };
}

// get a vector according to specific property(-ies)
macro_rules! match_vec_property {
    (Property::Unique) => {
        let vec = UniqueVec::new();
        vec
    };
    (Property::Sorted) => {
        let vec = SortedVec::new();
        vec
    }
}


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
    use crate::sorted_vector::SortedVecAlt;
    use crate::unique_sorted_vector::UniqueSortedVec;
    use crate::unique_sorted_vector::UniqueSortedVecAlt;
    use crate::unique_linked_list::UniqueLinkedList;
    use crate::sorted_linked_list::SortedLinkedList;
    use crate::unique_sorted_linked_list::UniqueSortedLinkedList;
    use crate::unique_sorted_linked_list::UniqueSortedLinkedListAlt;
    use crate::binary_search_tree::BinarySearchTree;
    use crate::container::{Property, type_of};
    //use test::Bencher;
    use std::collections::BTreeSet;

    /** Unique Vector*/
    #[test]
    fn unique_vec_creation_works() {
        let vec = UniqueVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_vec_creation_with_capacity_works() {
        let vec = UniqueVec::<u32>::with_capacity(10);
        assert_eq!(vec.capacity(), 10);
    }

    #[test]
    fn unique_vec_creation_from_vec() {
        let mut src = vec![1, 2, 3];
        let vec = UniqueVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3]);
    }

    #[test]
    fn unique_vec_contains_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        assert_eq!(vec.contains(&1), true);
        assert_eq!(vec.contains(&2), false);
    }

    #[test]
    fn unique_vec_push_works() {
        let mut vec = UniqueVec::new();
        for x in 0..10000 {
            vec.push(x);
            vec.push(x);
        }
        assert_eq!(vec.len(), 10000); // no duplication
    }

    #[test]
    fn unique_vec_pop_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn unique_vec_pop_some_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
    }

    #[test]
    fn unique_vec_insert_works() {
        let mut vec = UniqueVec::new();
        vec.insert(0, 10);
        vec.insert(1, 10);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_remove_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.remove(0), 1);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_truncate_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        vec.truncate(3);
        assert_eq!(*vec, [0, 1, 2]);
    }

    #[test]
    fn unique_vec_clear_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        vec.clear();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_vec_first_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.first(), None);
    }

    #[test]
    fn unique_vec_first_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.first(), Some(&0));
    }

    #[test]
    fn unique_vec_last_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.last(), None);
    }

    #[test]
    fn unique_vec_last_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.last(), Some(&4));
    }

    #[test]
    fn unique_vec_get_element_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.get(3), Some(&3));
    }

    #[test]
    fn unique_vec_get_slice_works() {
        let mut vec = UniqueVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        assert_eq!(vec.get(0..2), Some(&[0, 1][..]));
    }

    #[test]
    fn unique_vec_macro_one_works() {
        let mut vec = unique_vec![1, 1];
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_macro_two_works() {
        let mut vec = unique_vec![1; 4];
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_vec_append_works() {
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

    /* Sorted Vector */
    #[test]
    fn sorted_vec_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let vec = SortedVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3, 3]);
    }

    #[test]
    fn sorted_vec_macro_one_works() {
        let vec = sorted_vec![3, 7, 2, 1, 5, 4, 3];
        assert_eq!(*vec, [1, 2, 3, 3, 4, 5, 7])
    }

    #[test]
    fn sorted_vec_macro_two_works() {
        let vec = sorted_vec![1; 3];
        assert_eq!(*vec, [1, 1, 1])
    }

    #[test]
    fn sorted_vec_contains_works() {
        let mut vec = SortedVec::<u32>::new();
        assert_eq!(vec.contains(&1), false);
    }

    #[test]
    fn sorted_vec_push_works() {
        let mut vec = SortedVec::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(*vec, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn sorted_vec_append_works() {
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

    /* Unique Sorted Vector */
    #[test]
    fn unique_sorted_vec_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let vec = UniqueSortedVec::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3]);
    }

    #[test]
    fn unique_sorted_vec_macro_one_works() {
        let vec = unique_sorted_vec![3, 7, 2, 1, 5, 4, 3];
        assert_eq!(*vec, [1, 2, 3, 4, 5, 7])
    }

    #[test]
    fn unique_sorted_vec_macro_two_works() {
        let vec = unique_sorted_vec![1; 3];
        assert_eq!(*vec, [1])
    }

    #[test]
    fn unique_sorted_vec_push_works() {
        let mut src = vec![0, 1, 2, 3, 4];
        let mut vec = UniqueSortedVec::from_vec(&mut src);
        vec.push(5);
        vec.push(4);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn unique_sorted_vec_append_works() {
        let mut vec = UniqueSortedVec::new();
        let mut other = UniqueSortedVec::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5, 6])
    }

    /* Unique List */
    #[test]
    fn unique_ll_creation_works() {
        let l = UniqueLinkedList::<u32>::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn unique_ll_push_front_works() {
        let mut l = UniqueLinkedList::new();
        for x in 0..10000 {
            l.push_front(x);
            l.push_front(x);
        }
        assert_eq!(l.len(), 10000); // no duplication
    }

    #[test]
    fn unique_ll_push_back_works() {
        let mut l = UniqueLinkedList::new();
        for x in 0..10000 {
            l.push_back(x);
            l.push_back(x);
        }
        assert_eq!(l.len(), 10000); // no duplication
    }

    #[test]
    fn unique_ll_append_works() {
        let mut l = UniqueLinkedList::new();
        let mut other = UniqueLinkedList::new();
        for x in 0..5 {
            l.push_back(x);
        }
        for x in 2..7 {
            other.push_back(x);
        }
        l.append(&mut other);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }
    
    #[test]
    fn unique_ll_clear_works() {
        let mut l = UniqueLinkedList::new();
        for x in 0..5 {
            l.push_back(x);
        }
        l.clear();
        assert!(l.is_empty());
    }

    /* Sorted List */
    #[test]
    fn sorted_ll_creation_works() {
        let l = SortedLinkedList::<u32>::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn sorted_ll_push_front_works() {
        let mut l = SortedLinkedList::<u32>::new();
        l.push_front(4);
        l.push_front(0);
        l.push_front(3);
        l.push_front(1);
        l.push_front(2);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn sorted_ll_push_back_works() {
        let mut l = SortedLinkedList::<u32>::new();
        l.push_back(4);
        l.push_back(0);
        l.push_back(3);
        l.push_back(1);
        l.push_back(2);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn sorted_ll_append_works() {
        let mut l = SortedLinkedList::new();
        let mut other = SortedLinkedList::new();
        for x in 0..5 {
            l.push_back(x);
        }
        for x in 2..7 {
            other.push_back(x);
        }
        l.append(&mut other); // 0->1->2->2->3->3->4->4->5->6
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn sorted_ll_clear_works() {
        let mut l = SortedLinkedList::new();
        for x in 0..5 {
            l.push_back(x);
        }
        l.clear();
        assert!(l.is_empty());
    }

    /* Unique Sorted List */
    #[test]
    fn unique_sorted_ll_creation_works() {
        let l = UniqueSortedLinkedList::<u32>::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn unique_sorted_ll_push_front_works() {
        let mut l = UniqueSortedLinkedList::<u32>::new();
        l.push_front(4);
        l.push_front(4);
        l.push_front(0);
        l.push_front(3);
        l.push_front(3);
        l.push_front(1);
        l.push_front(1);
        l.push_front(2);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn unique_sorted_ll_push_back_works() {
        let mut l = UniqueSortedLinkedList::<u32>::new();
        l.push_back(4);
        l.push_back(4);
        l.push_back(0);
        l.push_back(0);
        l.push_back(3);
        l.push_back(1);
        l.push_back(3);
        l.push_back(2);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn unique_sorted_ll_append_works() {
        let mut l = UniqueSortedLinkedList::new();
        let mut other = UniqueSortedLinkedList::new();
        for x in 0..5 {
            other.push_back(x);
        }
        for x in 2..7 {
            l.push_back(x);
        }
        l.append(&mut other); // 0->1->2->3->4->5->6
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn unique_sorted_ll_clear_works() {
        let mut l = UniqueSortedLinkedList::new();
        for x in 0..5 {
            l.push_back(x);
        }
        l.clear();
        assert!(l.is_empty());
    }

    /* Test Binary Search Tree (Allow Duplication) */
    #[test]
    fn bst_creation_works() {
        let t = BinarySearchTree::<u32>::new();
        assert!(t.is_empty());
    }

    #[test]
    fn bst_insertion_works() {
        let mut t = BinarySearchTree::<u32>::new();
        for x in 0..5 {
            t.insert(x);
        }
        t.insert(0);
        t.insert(6);
        t.insert(4);
        assert_eq!(t.to_vec(), [0, 0, 1, 2, 3, 4, 4, 6]);
        assert_eq!(t.len(), 8);
    }

    #[test]
    fn bst_insertion_many_works() {
        let mut t = BinarySearchTree::<u32>::new();
        for x in 0..10000 {
            t.insert(x);
        }
        let vec: Vec<u32> = (0..10000).collect();
        assert_eq!(t.to_vec(), vec);
    }

    #[test]
    fn bst_deletion_works() {
        let mut t = BinarySearchTree::<u32>::new();
        for x in 0..5 {
            t.insert(x);
        }
        t.insert(0);
        t.insert(6);
        t.insert(4);
        t.remove(&4);
        assert_eq!(t.to_vec(), [0, 0, 1, 2, 3, 6]);
        assert_eq!(t.len(), 6);
    }

    #[test]
    fn bst_deletion_all_works() {
        let mut t = BinarySearchTree::<u32>::new();
        for x in 0..5 {
            t.insert(6);
        }
        t.remove(&6);
        assert_eq!(t.to_vec(), []);
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn bst_contains_works() {
        let mut t = BinarySearchTree::<u32>::new();
        for x in 0..5 {
            t.insert(x);
        }
        assert!(t.contains(&4));
        assert!(!t.contains(&5));
    }

    #[test]
    fn bst_clear_works() {
        let mut t = BinarySearchTree::<u32>::new();
        for x in 0..5 {
            t.insert(x);
        }
        t.clear();
        assert!(t.is_empty());
    }

    #[test]
    fn bst_first_works() {
        let mut t = BinarySearchTree::<u32>::new();
        assert_eq!(t.first(), None);
        for x in 0..100 {
            t.insert(x);
        }
        assert_eq!(t.first(), Some(&0));
    }

    #[test]
    fn bst_last_works() {
        let mut t = BinarySearchTree::<u32>::new();
        assert_eq!(t.last(), None);
        for x in 0..100 {
            t.insert(x);
        }
        assert_eq!(t.last(), Some(&99));
    }

    #[test]
    fn bst_pop_first_works() {
        let mut t = BinarySearchTree::new();
        for x in 0..10 {
            t.insert(x);
        }
        t.insert(0);
        assert_eq!(t.pop_first(), Some(0));
        assert_eq!(*t.to_vec(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(t.len(), 10);
        assert_eq!(t.pop_first(), Some(0));
        assert_eq!(t.pop_first(), Some(1));
        assert_eq!(*t.to_vec(), [2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(t.len(), 8);
    }

    #[test]
    fn bst_pop_last_works() {
        let mut t = BinarySearchTree::new();
        for x in 0..10 {
            t.insert(x);
        }
        t.insert(9);
        assert_eq!(t.pop_last(), Some(9));
        assert_eq!(*t.to_vec(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(t.len(), 10);
        assert_eq!(t.pop_last(), Some(9));
        assert_eq!(*t.to_vec(), [0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(t.pop_last(), Some(8));
        assert_eq!(t.len(), 8);
    }

    /* SortedVecAlt */
    #[test]
    fn sorted_vec_alt_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let mut vec = SortedVecAlt::from_vec(&mut src);
        assert_eq!(*vec.to_vec(), [1, 2, 3, 3]);
    }

    #[test]
    fn sorted_vec_alt_contains_works() {
        let mut vec = SortedVecAlt::<u32>::new();
        assert_eq!(vec.contains(&1), false);
    }

    #[test]
    fn sorted_vec_alt_push_works() {
        let mut vec = SortedVecAlt::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(*vec.to_vec(), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn sorted_vec_alt_pop_works() {
        let mut vec = SortedVecAlt::new();
        for x in 0..5 {
            vec.push(4 - x);
        }
        assert_eq!(vec.pop(), Some(4));
    }

    #[test]
    fn sorted_vec_alt_append_works() {
        let mut vec = SortedVecAlt::new();
        let mut other = SortedVecAlt::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec.to_vec(), [0, 1, 2, 2, 3, 3, 4, 4, 5, 6])
    }

    /* Unique Sorted Vector Alternative */
    #[test]
    fn unique_sorted_vec_alt_creation_from_vec_works() {
        let mut src = vec![3, 1, 2, 3];
        let vec = UniqueSortedVecAlt::from_vec(&mut src);
        assert_eq!(*vec, [1, 2, 3]);
    }

    #[test]
    fn unique_sorted_vec_alt_push_works() {
        let mut src = vec![0, 1, 2, 3, 4];
        let mut vec = UniqueSortedVecAlt::from_vec(&mut src);
        vec.push(5);
        vec.push(4);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn unique_sorted_vec_alt_append_works() {
        let mut vec = UniqueSortedVecAlt::new();
        let mut other = UniqueSortedVecAlt::new();
        for x in 0..5 {
            vec.push(x);
        }
        for x in 2..7 {
            other.push(x);
        }
        vec.append(&mut other);
        assert_eq!(*vec, [0, 1, 2, 3, 4, 5, 6])
    }

    /* Unique Sorted List Alternative */
    #[test]
    fn unique_sorted_ll_alt_creation_works() {
        let l = UniqueSortedLinkedListAlt::<u32>::new();
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn unique_sorted_ll_alt_push_front_works() {
        let mut l = UniqueSortedLinkedListAlt::<u32>::new();
        l.push_front(4);
        l.push_front(4);
        l.push_front(0);
        l.push_front(3);
        l.push_front(3);
        l.push_front(1);
        l.push_front(1);
        l.push_front(2);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn unique_sorted_ll_alt_push_back_works() {
        let mut l = UniqueSortedLinkedListAlt::<u32>::new();
        l.push_back(4);
        l.push_back(4);
        l.push_back(0);
        l.push_back(0);
        l.push_back(3);
        l.push_back(1);
        l.push_back(3);
        l.push_back(2);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn unique_sorted_ll_alt_append_works() {
        let mut l = UniqueSortedLinkedListAlt::new();
        let mut other = UniqueSortedLinkedListAlt::new();
        for x in 0..5 {
            other.push_back(x);
        }
        for x in 2..7 {
            l.push_back(x);
        }
        l.append(&mut other); // 0->1->2->3->4->5->6
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn unique_sorted_ll_alt_clear_works() {
        let mut l = UniqueSortedLinkedListAlt::new();
        for x in 0..5 {
            l.push_back(x);
        }
        l.clear();
        assert!(l.is_empty());
    }

    // macro get_vec test
    #[test]
    fn get_vec_works() {
        let v = get_vec!(u32);
        assert!(v.is_empty());
        assert_eq!(v, Vec::<u32>::new())
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