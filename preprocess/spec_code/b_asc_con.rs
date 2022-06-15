/*SPEC*
property ascending<T> {
    \c -> ((for-all-consecutive-pairs c) leq?)
}
type AscendingCon<T> = {c impl (Container) | (ascending c)}
*ENDSPEC*/

fn main () {
    let mut c = AscendingCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 20);
}