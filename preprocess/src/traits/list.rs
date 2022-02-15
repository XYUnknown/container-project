use std::collections::LinkedList;
use std::cmp::Ordering;
// nightly features
use std::collections::linked_list::CursorMut;
use crate::traits::container::Container;
use crate::traits::stack::Stack;

impl<T: Ord> Container<T> for LinkedList<T> {
    fn len(&self) -> usize {
        LinkedList::len(self)
    }

    fn contains(&self, x: &T) -> bool {
        LinkedList::contains(self, x)
    }

    fn is_empty(&self) -> bool {
        LinkedList::is_empty(self)
    }

    fn clear(&mut self) {
        LinkedList::clear(self);
    }

    fn insert(&mut self, elt: T) {
        LinkedList::push_back(self, elt);
    }

    fn remove(&mut self, elt: T) -> Option<T> {
        let mut c = self.cursor_front_mut();
        loop {
            match c.current() {
                Some(x) => {
                    match &elt.cmp(x) {
                        Ordering::Equal => {
                            return c.remove_current()
                        },
                        _ => c.move_next()
                    }
                },
                None => { // empty list
                    return None;
                }
            }
        }
    }
}

impl<T> Stack<T> for LinkedList<T> {
    fn push(&mut self, elt: T) {
        LinkedList::push_back(self, elt);
    }

    fn pop(&mut self) -> Option<T> {
        LinkedList::pop_back(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::container::Container;
    use crate::traits::stack::Stack;
    use std::collections::LinkedList;

    #[test]
    fn test_list_container_trait() {
        let list : &mut dyn Container<u32> = &mut LinkedList::<u32>::new();
        assert_eq!(list.len(), 0);
        list.insert(1);
        list.insert(4);
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove(9), None);
        assert_eq!(list.remove(1), Some(1));
        assert_eq!(list.len(), 1);
        assert!(list.contains(&4));
        list.clear();
        assert_eq!(list.len(), 0);
        //assert_eq!(list.pop_back(), None); // error
    }

    #[test]
    fn test_list_combo_trait() {
        trait ContainerStack<T>: Container<T> + Stack<T> {}
        impl<T: Ord> ContainerStack<T> for LinkedList<T> {}
        let list : &mut dyn ContainerStack<u32> = &mut LinkedList::<u32>::new();
        assert_eq!(list.len(), 0);
        list.insert(1);
        list.insert(4);
        assert_eq!(list.len(), 2);
        assert_eq!(list.remove(9), None);
        assert_eq!(list.remove(1), Some(1));
        assert_eq!(list.len(), 1);
        assert!(list.contains(&4));
        list.clear();
        assert_eq!(list.len(), 0);
        //assert_eq!(list.pop_back(), None); // error
    }
}