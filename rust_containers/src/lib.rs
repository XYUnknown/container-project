#![feature(const_fn)] // enabling impl<T: PartialEq> 

mod vectors;

#[cfg(test)]
mod tests {
    use crate::vectors::UniqueVec;
    #[test]
    fn unique_creation_works() {
        let vec = UniqueVec::<u32>::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn unique_push_works() {
        let mut vec = UniqueVec::new();
        for x in 0..10000 {
            vec.push(x);
            vec.push(x);
        }
        assert_eq!(vec.len(), 10000); // no duplication
    }
}
