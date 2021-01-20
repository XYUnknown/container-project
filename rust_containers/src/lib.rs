//#![feature(const_fn)] // enabling impl<T: PartialEq> 

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
    fn unique_creation_with_capacity_works() {
        let vec = UniqueVec::<u32>::with_capacity(10);
        assert_eq!(vec.capacity(), 10);
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

    #[test]
    fn unique_pop_none_works() {
        let mut vec = UniqueVec::<u32>::new();
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn unique_pop_some_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
    }

    #[test]
    fn unique_insert_works() {
        let mut vec = UniqueVec::new();
        vec.insert(0, 10);
        vec.insert(1, 10);
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn unique_remove_works() {
        let mut vec = UniqueVec::new();
        vec.push(1);
        vec.push(2);
        assert_eq!(vec.remove(0), 1);
        assert_eq!(vec.len(), 1)
    }
}
