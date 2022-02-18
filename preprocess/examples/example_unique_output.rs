/*CODEGEN*/
use preprocess::traits::container_constructor::ContainerConstructor;

struct UniqueCon<T> {
    elem_t: core::marker::PhantomData<T>,
}

trait UniqueConTrait<T>: preprocess::traits::container::Container<T>{}
impl<T: 'static + Ord> UniqueConTrait<T> for <UniqueCon<T> as ContainerConstructor>::Impl {}

impl<T: 'static + Ord> ContainerConstructor for UniqueCon<T> {
    type Impl = std::collections::BTreeSet::<T>; /// consider ...
    type Interface = dyn UniqueConTrait<T>;
    fn new() -> Box<Self::Interface> {
        Box::new(Self::Impl::new())
    }
}
/*ENDCODEGEN*/
fn main () {
    let mut c = UniqueCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
        //c.first();
    }
    assert_eq!(c.len(), 10);
}