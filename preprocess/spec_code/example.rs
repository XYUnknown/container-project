

/*SPEC*
property unique {
    \c -> ((for-all-unique-pairs c) \a -> \b -> ((neq a) b))
}
type UniqueCon<T> = {c : Con<T> | (unique c) }
*ENDSPEC*/

/*SPEC*
property ascending {
    \c -> ((for-all-unique-pairs c) \a -> \b -> ((leq a) b))
}
type AscendingCon<T> = {c : Con<T> | (ascending c)}

type UniqueAscendingCon<T> = {c : Con<T> | ((unique c) and (ascending c)) }
*ENDSPEC*/

fn main () {
    let mut c = UniqueCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 10);
}