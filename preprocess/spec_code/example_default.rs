/*SPEC*
property default {
    \c -> true
}

type DefaultCon<T> = {c impl (Container) | (default c)}
*ENDSPEC*/

fn main () {
    let mut c = DefaultCon::<u32>::new();
    for x in 0..10 {
        c.insert(x);
        c.insert(x);
        //c.first();
    }
    is_empty()
}