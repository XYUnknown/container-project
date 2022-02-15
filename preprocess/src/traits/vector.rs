use std::vec::Vec;
use crate::traits::container::Container;

impl<T: PartialEq> Container<T> for Vec<T> {
    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn contains(&self, x: &T) -> bool {
        <[T]>::contains(self, x) // use fully qualified syntax to avoid function name collision
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn clear(&mut self) {
        Vec::clear(self);
    }

    fn insert(&mut self, elt: T) {
        Vec::push(self, elt);
    }

    fn remove(&mut self, elt: T) -> Option<T> {
        match self.iter().position(|x| *x == elt) {
            Some(index) => {
                Some(self.remove(index))
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::container::Container;
    use std::vec::Vec;

    #[test]
    fn test_vector_container_trait() {
        let vec : &mut dyn Container<u32> = &mut Vec::<u32>::new();
        assert_eq!(vec.len(), 0);
        vec.insert(1);
        vec.insert(4);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.remove(9), None);
        assert_eq!(vec.remove(1), Some(1));
        assert_eq!(vec.len(), 1);
        assert!(vec.contains(&4));
        vec.clear();
        assert_eq!(vec.len(), 0);
        //assert_eq!(vec.pop(), None); // error
    }
}