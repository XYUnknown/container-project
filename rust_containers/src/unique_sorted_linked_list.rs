use std::collections::LinkedList;
use std::ops::Deref;
use std::ops::DerefMut;
use std::cmp::Ordering;
use std::collections::linked_list::Iter;
// nightly features
use std::collections::linked_list::Cursor;
use std::collections::linked_list::CursorMut;

use crate::sorted_linked_list::SortedLinkedList;
use crate::unique_linked_list::UniqueLinkedList;

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
        *self.ll = temp; // deref mut
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

impl<T> Deref for UniqueSortedLinkedList<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll.deref()
    }
}

impl<T: Clone> Clone for UniqueSortedLinkedList<T> {
    fn clone(&self) -> Self {
        UniqueSortedLinkedList { ll: self.ll.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ll.clone_from(&source.ll);
    }
}

// A unique and sorted doubly linked-list
pub struct UniqueSortedLinkedListAlt<T> {
    ll: UniqueLinkedList<T>,
}

impl<T: Ord + PartialEq> UniqueSortedLinkedListAlt<T> {
    pub const fn new() -> UniqueSortedLinkedListAlt<T> {
        UniqueSortedLinkedListAlt { ll: UniqueLinkedList::new() }
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
        let mut c = self.cursor_front_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less => {
                            c.insert_before(elt);
                            break;
                        },
                        Ordering::Equal => { // do not insert the duplication
                            break;
                        },
                        Ordering::Greater => c.move_next()
                    }
                },
                None => { // empty list
                    c.insert_before(elt);
                    break;
                }
            }
        }
    }

    pub fn push_back(&mut self, elt: T) {
        let mut c = self.cursor_back_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less => {
                            c.move_prev();
                        },
                        Ordering::Greater => {
                            c.insert_after(elt);
                            break;
                        },
                        Ordering::Equal => { // do not push dupilcation
                            break;
                        }
                    }
                },
                None => { // empty list
                    c.insert_after(elt);
                    break;
                }
            }
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
                                    match self.ll.pop_front() {
                                        Some(elem) => temp.push_back(elem),
                                        None => continue // not possible
                                    }
                                },
                                Ordering::Greater => {
                                    match other.ll.pop_front() {
                                        Some(elem) => temp.push_back(elem),
                                        None => continue // not possible
                                    }
                                },
                                Ordering::Equal => {
                                    match self.ll.pop_front() {
                                        Some(elem) => temp.push_back(elem),
                                        None => continue // not possible
                                    }
                                    other.ll.pop_front(); // discard dupilcation
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
        *self.ll = temp; // deref mut
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

    pub fn cursor_front(&self) -> Cursor<'_, T> {
        self.ll.cursor_front()
    }

    pub fn cursor_front_mut(&mut self) -> CursorMut<'_, T> {
        self.ll.cursor_front_mut()
    }

    pub fn cursor_back(&self) -> Cursor<'_, T> {
        self.ll.cursor_back()
    }

    pub fn cursor_back_mut(&mut self) -> CursorMut<'_, T> {
        self.ll.cursor_back_mut()
    }
    
}

impl<T> Deref for UniqueSortedLinkedListAlt<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll.deref()
    }
}

impl<T> DerefMut for UniqueSortedLinkedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ll
    }
}

impl<T: Clone> Clone for UniqueSortedLinkedListAlt<T> {
    fn clone(&self) -> Self {
        UniqueSortedLinkedListAlt { ll: self.ll.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ll.clone_from(&source.ll);
    }
}