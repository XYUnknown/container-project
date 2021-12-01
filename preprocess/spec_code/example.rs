

/*SPEC*
property unique {
    \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
}
type UniqueCon<T> = {c : Con<T> | (unique c) }
*ENDSPEC*/

/*SPEC*
property ascending {
    \c -> ((for_all_unique_pairs c) \a -> \b -> ((neq a) b))
}
type AscendingCon<T> = {c : Con<T> | (ascending c) }
*ENDSPEC*/

fn main () {
    let mut c = UniqueCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 10);
}