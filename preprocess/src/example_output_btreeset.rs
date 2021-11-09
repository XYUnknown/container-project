// generated code start 
type UniqueCon<T> = std::collections::BTreeSet<T>;
// generated code end

fn main () {
    let mut c = UniqueCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
    }
    assert_eq!(c.len(), 10);
}
