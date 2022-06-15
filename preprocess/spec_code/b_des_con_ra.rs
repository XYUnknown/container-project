/*SPEC*
property descending<T> {
    \c -> ((for-all-consecutive-pairs c) geq?)
}
type DescendingCon<T> = {c impl (Container, RandomAccess) | (descending c)}
*ENDSPEC*/

fn main () {
    let mut c = DescendingCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 20);
}