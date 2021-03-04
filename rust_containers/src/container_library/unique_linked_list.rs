use std::collections::LinkedList;
use std::ops::Deref;
use std::ops::DerefMut;
use std::collections::linked_list::Iter;

// A unique doubly linked-list
pub struct UniqueLinkedList<T> {
    ll: LinkedList<T>,
}

impl<T: PartialEq> UniqueLinkedList<T> {
    pub const fn new() -> UniqueLinkedList<T> {
        UniqueLinkedList { ll: LinkedList::new() }
    }
    
    pub fn len(&self) -> usize {
        self.ll.len()
    }

    pub fn contains(&self, x: &T) -> bool {
        self.ll.contains(x)
    }

    pub fn is_empty(&self) -> bool {
        self.ll.is_empty()
    }

    /**
     * Modifying the list
     */
    pub fn push_front(&mut self, elt: T) {
        if !self.contains(&elt) {
            self.ll.push_front(elt);
        }
    }

    pub fn push_back(&mut self, elt: T) {
        if !self.contains(&elt) {
            self.ll.push_back(elt);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.ll.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.ll.pop_back()
    }

    pub fn clear(&mut self) {
        self.ll.clear();
    }

    pub fn append(&mut self, other: &mut Self) {
        loop {
            match other.pop_front() {
                Some(x) => self.push_back(x),
                None => break
            }
        }
    }

    // Removes the element at the given index and returns it.
    // O(n) complexity
    // nightly feature in std
    //pub fn remove(&mut self, index: usize) -> T {
    //    self.ll.remove(index)
    //}

    /**
     * Accessing elements
     */
    pub fn front(&self) -> Option<&T> {
        self.ll.front()
    }

    //pub fn front_mut(&mut self) -> Option<&mut T> {
    //    self.ll.front_mut()
    //}

    pub fn back(&self) -> Option<&T> {
        self.ll.back()
    }

    //pub fn back_mut(&mut self) -> Option<&mut T> {
    //    self.ll.back_mut()
    //}

    pub fn iter(&self) -> Iter<'_, T> {
        self.ll.iter()
    }

    //pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    //    self.ll.iter_mut()
    //}
}

impl<T> Deref for UniqueLinkedList<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll
    }
}

impl<T> DerefMut for UniqueLinkedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ll
    }
}

impl<T: Clone> Clone for UniqueLinkedList<T> {
    fn clone(&self) -> Self {
        UniqueLinkedList { ll: self.ll.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ll.clone_from(&source.ll);
    }
}

#[cfg(test)]
mod tests {
    use crate::container_library::unique_linked_list::UniqueLinkedList;
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
}