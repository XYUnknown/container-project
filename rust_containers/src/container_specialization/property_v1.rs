// The version not working
use std::vec::Vec;
use std::marker::PhantomData;

pub trait PushProperty<T, R> { // the problem is the R here, depending on the specific property
    fn pre(&self, value: &T) -> R;
    fn exec(&mut self, cond: R, value: T); 
    fn assert(&self) -> bool;
  
    fn p_push(&mut self, value: T) {
        let cond = self.pre(&value);
        self.exec(cond, value);
        assert!(self.assert());
    }
}

pub trait Unique {}
pub trait Sorted {}

pub trait Container<T, P: ?Sized> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn contains(&self, x: &T) -> bool;
    fn is_empty(&self) -> bool;
    fn first(&mut self) -> Option<&T>;
    fn last(&mut self) -> Option<&T>;
}

pub struct VecWrapper<T, P: ?Sized> {
    v: Vec<T>,
    property: PhantomData<P> // just a marker
}

impl<T: PartialEq, P: ?Sized> VecWrapper<T, P> {
    pub const fn new() -> VecWrapper<T, P> {
        VecWrapper { v: Vec::new(), property: PhantomData, }
    }
}

impl<T: PartialEq, P: ?Sized> PushProperty<T, ()> for VecWrapper<T, P> {
    default fn pre(&self, value: &T) -> () {
        ()
    }
    default fn exec(&mut self, cond: (), value: T) {
        self.v.push(value);
    }
    default fn assert(&self) -> bool {
        true
    }
}

impl<T: PartialEq> PushProperty<T, bool> for VecWrapper<T, dyn Unique> {
    fn pre(&self, value: &T) -> bool {
        !self.v.contains(&value)
    }
    
    fn exec(&mut self, cond: bool, value: T) {
        if cond {
            self.v.push(value);
        }
    }
    
    fn assert(&self) -> bool {
        true // TODO implement assertion
    }
}

impl<T: PartialEq+Ord> PushProperty<T, usize> for VecWrapper<T, dyn Sorted> {
    fn pre(&self, value: &T) -> usize {
        self.v.binary_search(&value).unwrap_or_else(|i| i)
    }
    
    fn exec(&mut self, cond: usize, value: T) {
        self.v.insert(cond, value); // cond is the location
    }
    
    fn assert(&self) -> bool {
        self.v.iter().is_sorted()
    }
}


impl<T: PartialEq, P: ?Sized> Container<T, P> for VecWrapper<T, P> {
    fn push(&mut self, value: T) {
        self.p_push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    fn clear(&mut self) {
        self.v.clear();
    }

    fn len(&self) -> usize {
        self.v.len()
    }

    fn contains(&self, x: &T) -> bool {
        self.v.contains(x)
    }

    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    fn first(&mut self) -> Option<&T> {
        self.v.first()
    }

    fn last(&mut self) -> Option<&T> {
        self.v.last()
    }
}

fn get_vec<T: 'static + PartialEq + Ord + Sized, P: 'static + ?Sized> () -> Box<dyn Container<T, P>> {
    let vec = Box::new(VecWrapper::<T, P>::new());
    vec
}

#[cfg(test)]
mod tests {
    use std::vec::Vec;
    use crate::container_specialization::property_v1::*;

    #[test]
    fn test_prop_push_works() {
        let mut vec = VecWrapper::<_, ()>::new();
        for x in 0..100 {
            vec.p_push(x);
            vec.p_push(x);
        }
        assert_eq!(vec.v.len(), 200);
    }

    /*#[test]
    fn test_prop_unique_push_works() {
        let mut vec = VecWrapper::<u32, dyn Unique>::new();
        for x in 0..100 {
            vec.p_push(x);
            vec.p_push(x);
        }
        assert_eq!(vec.v.len(), 100);
    }*/

    #[test]
    fn test_prop_con_works() {
        let mut c = get_vec::<u32, ()>();
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 200);
    }

    /*
    #[test]
    fn test_prop_unique_con_works() {
        let mut c = get_vec::<u32, dyn Unique>();
        for x in 0..100 {
            c.push(x);
            c.push(x);
        }
        assert_eq!(c.len(), 100);
    }*/
}