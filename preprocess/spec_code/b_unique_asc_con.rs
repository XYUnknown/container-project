/*SPEC*
property unique<T> {
    \c <: (Container) -> ((for-all-elems c) \a -> ((unique-count? a) c))
}
property ascending<T> {
    \c -> ((for-all-consecutive-pairs c) leq?)
}

type StrictlyAscendingCon<S> = {c impl (Container) | ((unique c) and (ascending c))}
*ENDSPEC*/

fn main () {
    let mut c = StrictlyAscendingCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 10);
}