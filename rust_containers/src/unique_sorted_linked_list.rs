use std::collections::LinkedList;
use std::ops::Deref;
use std::cmp::Ordering;
use std::collections::linked_list::Iter;
// nightly features
use std::collections::linked_list::Cursor;
use std::collections::linked_list::CursorMut;

use crate::sorted_linked_list::SortedLinkedList;

// A unique and sorted doubly linked-list
pub struct UniqueSortedLinkedList<T> {
    ll: SortedLinkedList<T>,
}

impl<T: Ord + PartialEq> UniqueSortedLinkedList<T> {
    pub const fn new() -> UniqueSortedLinkedList<T> {
        UniqueSortedLinkedList { ll: SortedLinkedList::new() }
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
        let mut temp = LinkedList::new();
        loop {
            match self.front() {
                Some(x1) => {
                    match other.front() {
                        Some(x2) => {
                            match x1.cmp(x2) {
                                Ordering::Less => {  
                                    match self.pop_front() {
                                        Some(elem) => temp.push_back(elem),
                                        None => continue // not possible
                                    }
                                },
                                Ordering::Greater => {
                                    match other.pop_front() {
                                        Some(elem) => temp.push_back(elem),
                                        None => continue // not possible
                                    }
                                },
                                Ordering::Equal => {
                                    match self.pop_front() {
                                        Some(elem) => temp.push_back(elem),
                                        None => continue // not possible
                                    }
                                    other.pop_front(); // discard dupilcation
                                }
                            }
                        },
                        None => {
                            temp.append(&mut self.ll);
                            break;
                        }
                    }
                },
                None => {
                    temp.append(&mut other.ll);
                    break;
                }
            }
        }
        *self.ll = temp;
    }

    /**
     * Accessing elements
     */
    pub fn front(&self) -> Option<&T> {
        self.ll.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.ll.back()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.ll.iter()
    }
    
}