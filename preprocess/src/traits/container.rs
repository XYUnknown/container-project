pub trait Container<T> {
    fn len(&self) -> usize;
    fn contains(&self, x: &T) -> bool;
    fn is_empty(&self) -> bool;
    fn insert(&mut self, elt: T);
    fn clear(&mut self);
    fn remove(&mut self, elt: T) -> Option<T>; // remove first occurance
}