use std::collections::BTreeSet;
use crate::traits::container::Container;

impl<T: Ord> Container<T> for BTreeSet<T> {
    fn len(&self) -> usize {
        BTreeSet::len(self)
    }

    fn contains(&self, x: &T) -> bool {
        BTreeSet::contains(self, x) // use fully qualified syntax to avoid function name collision
    }

    fn is_empty(&self) -> bool {
        BTreeSet::is_empty(self)
    }

    fn clear(&mut self) {
        BTreeSet::clear(self);
    }

    fn insert(&mut self, elt: T) {
        BTreeSet::insert(self, elt);
    }

    fn remove(&mut self, elt: T) -> Option<T> {
        match BTreeSet::remove(self, &elt) {
            true => Some(elt),
            false => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::container::Container;
    use std::collections::BTreeSet;

    #[test]
    fn test_treeset_container_trait() {
        let set : &mut dyn Container<u32> = &mut BTreeSet::<u32>::new();
        assert_eq!(set.len(), 0);
        set.insert(1);
        set.insert(4);
        assert_eq!(set.len(), 2);
        assert_eq!(set.remove(9), None);
        assert_eq!(set.remove(1), Some(1));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&4));
        set.clear();
        assert_eq!(set.len(), 0);
    }
}