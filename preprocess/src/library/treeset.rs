/*LIBSPEC-NAME*
rust-btreeset-spec std::collections::BTreeSet
*ENDLIBSPEC-NAME*/

use std::collections::BTreeSet;
use std::iter::FromIterator;
use crate::traits::{Container, RandomAccess};
use crate::proptest::*;

use proptest::prelude::*;
use proptest::collection::btree_set;

use im::conslist::{ConsList};
use im::conslist;
use std::sync::Arc;

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord> Container<T> for BTreeSet<T> {

    /*LIBSPEC*
    /*OPNAME*
    len op-len pre-len post-len
    *ENDOPNAME*/
    (define (op-len xs) (cons xs (length xs)))
    (define (pre-len xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-len xs r) (equal? r (op-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        BTreeSet::len(self)
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
        BTreeSet::contains(self, x)
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
        BTreeSet::is_empty(self)
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
        BTreeSet::clear(self);
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
        BTreeSet::insert(self, elt);
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
        match BTreeSet::remove(self, &elt) {
            true => Some(elt),
            false => None
        }
    }
}

/*IMPL*
RandomAccess
*ENDIMPL*/
impl<T: Ord> RandomAccess<T> for BTreeSet<T> {
    /*LIBSPEC*
    /*OPNAME*
    first op-first pre-first post-first
    *ENDOPNAME*/
    (define (op-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    (define (pre-first xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-first xs r) (equal? r (op-first xs)))
    *ENDLIBSPEC*/
    fn first(&mut self) -> Option<&T> {
        BTreeSet::first(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    last op-last pre-last post-last
    *ENDOPNAME*/
    (define (op-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    (define (pre-last xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-last xs r) (equal? r (op-last xs)))
    *ENDLIBSPEC*/
    fn last(&mut self) -> Option<&T> {
        BTreeSet::last(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    nth op-nth pre-nth post-nth
    *ENDOPNAME*/
    (define (op-nth xs n)
      (cond
        [(>= n (length xs)) (cons xs null)]
        [(< n 0) (cons xs null)]
        [else (cons xs (list-ref xs n))]))
    (define (pre-nth xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-nth n xs r) (equal? r (op-nth xs n)))
    *ENDLIBSPEC*/
    fn nth(&mut self, n: usize) -> Option<&T> {
        BTreeSet::iter(self).nth(n)
    }                                      
}

fn abstraction<T>(t: BTreeSet<T>) -> ConsList<T> {
    let list: ConsList<T> = ConsList::from_iter(t);
    list
}

proptest!{
    #![proptest_config(ProptestConfig {
        cases: 100, .. ProptestConfig::default()
      })]
    #[test]
    fn test_btree_len(ref mut t in btree_set(".*", 0..100)) {
        let abs_list = abstraction(t.clone());
        // pre: our list model is a sorted and unique list
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        assert_eq!(Container::<String>::len(t), abs_list.len());
        assert_eq!(abstraction(t.clone()), abs_list);
    }

    #[test]
    fn test_btree_contains(ref mut t in btree_set(".*", 0..100), a in ".*") {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        assert_eq!(Container::<String>::contains(t, &a), contains(&abs_list, &a));
        assert_eq!(abstraction(t.clone()), abs_list);
    }

    #[test]
    fn test_btree_is_empty(ref mut t in btree_set(".*", 0..100)) {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        assert_eq!(Container::<String>::is_empty(t), abs_list.is_empty());
        assert_eq!(abstraction(t.clone()), abs_list);
    }

    #[test]
    fn test_btree_insert(ref mut t in btree_set(".*", 0..100), a in ".*") {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let after_list = unique(&abs_list.append(conslist![a.clone()]).sort());
        Container::<String>::insert(t, a.clone());
        assert_eq!(abstraction(t.clone()), after_list);
    }

    #[test]
    fn test_btree_clear(ref mut t in btree_set(".*", 0..100)) {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let after_list = clear(&abs_list);
        Container::<String>::clear(t);
        assert_eq!(abstraction(t.clone()), after_list);
    }

    #[test]
    fn test_btree_remove(ref mut t in btree_set(".*", 0..100), a in ".*") {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let (after_list, abs_elem) = remove(&abs_list, a.clone());
        let elem = Container::<String>::remove(t, a.clone());
        assert_eq!(abstraction(t.clone()), after_list);
        assert_eq!(elem, abs_elem);
    }

    #[test]
    fn test_btree_first(ref mut t in btree_set(".*", 0..100)) {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let elem = RandomAccess::<String>::first(t);
        let abs_first = first(&abs_list);
        assert_eq!(elem, abs_first);
        assert_eq!(abstraction(t.clone()), abs_list);
    }

    #[test]
    fn test_btree_last(ref mut t in btree_set(".*", 0..100)) {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let elem = RandomAccess::<String>::last(t);
        let abs_last = last(&abs_list);
        assert_eq!(elem, abs_last);
        assert_eq!(abstraction(t.clone()), abs_list);
    }

    #[test]
    fn test_btree_nth(ref mut t in btree_set(".*", 0..100), n in 0usize..100) {
        let abs_list = abstraction(t.clone());
        //pre
        assert_eq!(abs_list, unique(&abs_list.sort()));
        //post
        let elem = RandomAccess::<String>::nth(t, n.clone());
        let abs_nth = nth(&abs_list, n);
        assert_eq!(elem, abs_nth);
        assert_eq!(abstraction(t.clone()), abs_list);
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::traits::{Container, RandomAccess};
//     use std::collections::BTreeSet;

//     #[test]
//     fn test_treeset_container_trait() {
//         let set : &mut dyn Container<u32> = &mut BTreeSet::<u32>::new();
//         assert_eq!(set.len(), 0);
//         set.insert(1);
//         set.insert(4);
//         assert_eq!(set.len(), 2);
//         assert_eq!(set.remove(9), None);
//         assert_eq!(set.remove(1), Some(1));
//         assert_eq!(set.len(), 1);
//         assert!(set.contains(&4));
//         set.clear();
//         assert_eq!(set.len(), 0);
//     }
// }