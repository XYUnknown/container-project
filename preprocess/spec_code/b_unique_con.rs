/*SPEC*
property unique<T> {
    \c <: (Container) -> ((for-all-elems c) \a -> ((unique-count? a) c))
}

type UniqueCon<S> = {c impl (Container) | (unique c)}
*ENDSPEC*/

fn main () {
    let mut c = UniqueCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 10);
}