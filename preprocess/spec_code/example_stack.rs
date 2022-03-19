/*SPEC*
property lifo<T> {
    \c <: (Stack) -> (forall \n -> ((equal? (pop ((push c) n))) n))
}

type StrackCon<S> = {c impl (Container, Stack) | (lifo c)}
*ENDSPEC*/

fn main () {
    let mut c = StackCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
        //c.first();
    }
    assert_eq!(c.len(), 10);
}