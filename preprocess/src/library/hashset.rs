/*LIBSPEC-NAME*
rust-hashset-spec std::collections::HashSet
*ENDLIBSPEC-NAME*/

use std::collections::HashSet;
use std::hash::Hash;
use crate::traits::Container;

use crate::proptest::*;
use proptest::prelude::*;
use proptest::collection::hash_set;

use im::conslist::{ConsList};
use im::conslist;
use std::sync::Arc;
use std::iter::FromIterator;

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord + Hash> Container<T> for HashSet<T> {

    /*LIBSPEC*
    /*OPNAME*
    len op-len pre-len post-len
    *ENDOPNAME*/
    (define (op-len xs) (cons xs (length xs)))
    (define (pre-len xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-len xs r) (equal? r (op-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        HashSet::len(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    contains op-contains pre-contains post-contains
    *ENDOPNAME*/
    (define (op-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    (define (pre-contains xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-contains xs x r) (equal? r (op-contains xs x)))
    *ENDLIBSPEC*/
    fn contains(&mut self, x: &T) -> bool {
        HashSet::contains(self, x)
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty op-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (op-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-is-empty xs r) (equal? r (op-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&mut self) -> bool {
        HashSet::is_empty(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    clear op-clear pre-clear post-clear 
    *ENDOPNAME*/
    (define (op-clear xs) null)
    (define (pre-clear xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-clear xs r) (equal? r (op-clear xs)))
    *ENDLIBSPEC*/
    fn clear(&mut self) {
        HashSet::clear(self);
    }

    /*LIBSPEC*
    /*OPNAME*
    insert op-insert pre-insert post-insert
    *ENDOPNAME*/
    (define (op-insert xs x) (remove-duplicates (sort (append xs (list x)) <)))
    (define (pre-insert xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-insert xs x ys) (equal? ys (op-insert xs x)))
    *ENDLIBSPEC*/
    fn insert(&mut self, elt: T) {
        HashSet::insert(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    remove op-remove pre-remove post-remove
    *ENDOPNAME*/
    (define (op-remove xs x)
      (cond
        [(list? (member x xs)) (cons (remove x xs) x)]
        [else (cons xs null)]))
    (define (pre-remove xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-remove xs r) (equal? r (op-remove xs)))
    *ENDLIBSPEC*/
    fn remove(&mut self, elt: T) -> Option<T> {
        match HashSet::remove(self, &elt) {
            true => Some(elt),
            false => None
        }
    }
}

fn abstraction<T: Ord>(h: HashSet<T>) -> ConsList<T> {
    let list: ConsList<T> = ConsList::from_iter(h);
    list.sort()
}

proptest!{
    #[test]
    fn test_hashset_len(ref mut h in hash_set(".*", 0..100)) {
        let abs_list = abstraction(h.clone());
        // pre: our list model is a sorted and unique list
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        assert_eq!(Container::<String>::len(h), abs_list.len());
        assert_eq!(abstraction(h.clone()), abs_list);
    }

    #[test]
    fn test_hashset_contains(ref mut h in hash_set(".*", 0..100), a in ".*") {
        let abs_list = abstraction(h.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        assert_eq!(Container::<String>::contains(h, &a), contains(&abs_list, &a));
        assert_eq!(abstraction(h.clone()), abs_list);
    }

    #[test]
    fn test_hashset_is_empty(ref mut h in hash_set(".*", 0..100)) {
        let abs_list = abstraction(h.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        assert_eq!(Container::<String>::is_empty(h), abs_list.is_empty());
        assert_eq!(abstraction(h.clone()), abs_list);
    }

    #[test]
    fn test_hashset_insert(ref mut h in hash_set(".*", 0..100), a in ".*") {
        let abs_list = abstraction(h.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let after_list = unique(&abs_list.append(conslist![a.clone()]).sort());
        Container::<String>::insert(h, a.clone());
        assert_eq!(abstraction(h.clone()), after_list);
    }

    #[test]
    fn test_hash_clear(ref mut h in hash_set(".*", 0..100)) {
        let abs_list = abstraction(h.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let after_list = clear(&abs_list);
        Container::<String>::clear(h);
        assert_eq!(abstraction(h.clone()), after_list);
    }

    #[test]
    fn test_hashset_remove(ref mut h in hash_set(".*", 0..100), a in ".*") {
        let abs_list = abstraction(h.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let (after_list, abs_elem) = remove(&abs_list, a.clone());
        let elem = Container::<String>::remove(h, a.clone());
        assert_eq!(abstraction(h.clone()), after_list);
        assert_eq!(elem, abs_elem);
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::Container;
    use std::collections::HashSet;

    #[test]
    fn test_hashset_container_trait() {
        let set : &mut dyn Container<u32> = &mut HashSet::<u32>::new();
        assert_eq!(set.len(), 0);
        set.insert(1);
        set.insert(4);
        assert_eq!(set.len(), 2);
        assert_eq!(set.remove(9), None);
        assert_eq!(set.remove(1), Some(1));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&4));
        set.clear();
        assert_eq!(set.len(), 0);
    }
}