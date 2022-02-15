pub trait Stack<T> {
    fn push(&mut self, elt: T);
    fn pop(&mut self) -> Option<T>; // remove first occurance
}