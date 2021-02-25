use std::any::{Any, TypeId};

pub trait Container<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self);
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn contains(&self, x: &T) -> bool;
    fn is_empty(&self) -> bool;
}

pub trait Vector<T> : Container<T> {

}

pub enum Property {
    Unique,
    Sorted,
}

pub fn type_of<T: ?Sized + Any>(_s: &T) -> TypeId {
    TypeId::of::<String>()
}