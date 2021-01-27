use std::collections::LinkedList;
use std::ops::Deref;
use std::collections::linked_list::Iter;
//use std::collections::linked_list::IterMut;

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

impl<T: Clone> Clone for UniqueLinkedList<T> {
    fn clone(&self) -> Self {
        UniqueLinkedList { ll: self.ll.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ll.clone_from(&source.ll);
    }
}