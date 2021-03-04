use std::collections::LinkedList;
use std::ops::Deref;
use std::ops::DerefMut;
use std::cmp::Ordering;
use std::collections::linked_list::Iter;
// nightly features
use std::collections::linked_list::Cursor;
use std::collections::linked_list::CursorMut;

// A sorted doubly linked-list
pub struct SortedLinkedList<T> {
    ll: LinkedList<T>,
}

impl <T: Ord> SortedLinkedList<T> {
    pub const fn new() -> SortedLinkedList<T> {
        SortedLinkedList { ll: LinkedList::new() }
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
    // Iterate from the first element to look for an appropate place
    // O(n) worst case
    pub fn push_front(&mut self, elt: T) {
        let mut c = self.cursor_front_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less | Ordering::Equal => {
                            c.insert_before(elt);
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

    // Iterate from the last elemnet to look for an appropate place
    // O(n) worst case
    pub fn push_back(&mut self, elt: T) {
        let mut c = self.cursor_back_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Less => {
                            c.move_prev();
                        },
                        Ordering::Greater | Ordering::Equal => {
                            c.insert_after(elt);
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
                                Ordering::Less | Ordering::Equal => {  
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
        self.ll = temp;
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

impl<T> Deref for SortedLinkedList<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.ll
    }
}

impl<T> DerefMut for SortedLinkedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ll
    }
}

impl<T: Clone> Clone for SortedLinkedList<T> {
    fn clone(&self) -> Self {
        SortedLinkedList { ll: self.ll.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ll.clone_from(&source.ll);
    }
}

#[cfg(test)]
mod tests {
    use crate::container_library::sorted_linked_list::SortedLinkedList;
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
}