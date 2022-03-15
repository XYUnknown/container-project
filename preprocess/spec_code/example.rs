/*SPEC*
property unique<T> {
    \c -> ((for-all-elems c) \a -> ((unique-count? a) c))
}

type UniqueCon<T> = {c impl (Container) | (unique c) }
*ENDSPEC*/

/*SPEC*
property ascending<T> {
    \c -> ((for-all-consecutive-pairs c) leq?)
}
type AscendingConWithPos<T> = {c impl (Container, WithPosition) | (ascending c)}

type UniqueAscendingCon<T> = {c impl (Container) | ((unique c) and (ascending c)) }
*ENDSPEC*/

// The Con<T> is the Container trait
fn main () {
    let mut c = UniqueCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 10);
}