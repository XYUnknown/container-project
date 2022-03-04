pub mod container_constructor;

pub trait Container<T> {
    fn len(&mut self) -> usize;
    fn contains(&mut self, x: &T) -> bool;
    fn is_empty(&mut self) -> bool;
    fn insert(&mut self, elt: T);
    fn clear(&mut self);
    fn remove(&mut self, elt: T) -> Option<T>; // remove first occurance
}

pub trait Stack<T> {
    fn push(&mut self, elt: T);
    fn pop(&mut self) -> Option<T>;
}

// random access
pub trait WithPosition<T> {
    fn first(&mut self) -> Option<&T>;
    fn last(&mut self) -> Option<&T>;
    fn nth(&mut self, n: usize) -> Option<&T>;
}