/*LIBSPEC-NAME*
rust-btreeset-spec std::collections::BTreeSet
*ENDLIBSPEC-NAME*/

use std::collections::BTreeSet;
use crate::traits::{Container, WithPosition};

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord> Container<T> for BTreeSet<T> {

    /*LIBSPEC*
    /*OPNAME*
    len spec-len pre-len post-len
    *ENDOPNAME*/
    (define (spec-len xs) (cons xs (length xs)))
    (define (pre-len xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-len xs r) (equal? r (spec-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        BTreeSet::len(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    contains spec-contains pre-contains post-contains
    *ENDOPNAME*/
    (define (spec-contains xs x)
      (cond
        [(list? (member x xs)) (cons xs #t)]
        [else (cons xs #f)]))
    (define (pre-contains xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-contains xs x r) (equal? r (spec-contains xs x)))
    *ENDLIBSPEC*/
    fn contains(&mut self, x: &T) -> bool {
        BTreeSet::contains(self, x)
    }

    /*LIBSPEC*
    /*OPNAME*
    is-empty spec-is-empty pre-is-empty post-is-empty
    *ENDOPNAME*/
    (define (spec-is-empty xs) (cons xs (null? xs)))
    (define (pre-is-empty xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-is-empty xs r) (equal? r (spec-is-empty xs)))
    *ENDLIBSPEC*/
    fn is_empty(&mut self) -> bool {
        BTreeSet::is_empty(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    clear spec-clear pre-clear post-clear 
    *ENDOPNAME*/
    (define (spec-clear xs) null)
    (define (pre-clear xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-clear xs r) (equal? r (spec-clear xs)))
    *ENDLIBSPEC*/
    fn clear(&mut self) {
        BTreeSet::clear(self);
    }

    /*LIBSPEC*
    /*OPNAME*
    insert spec-insert pre-insert post-insert
    *ENDOPNAME*/
    (define (spec-insert xs x) (remove-duplicates (sort (append xs (list x)) <)))
    (define (pre-insert xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-insert xs x ys) (equal? ys (spec-insert xs x)))
    *ENDLIBSPEC*/
    fn insert(&mut self, elt: T) {
        BTreeSet::insert(self, elt);
    }

    /*LIBSPEC*
    /*OPNAME*
    remove spec-remove pre-remove post-remove
    *ENDOPNAME*/
    (define (spec-remove xs x)
      (cond
        [(list? (member x xs)) (cons (remove x xs) x)]
        [else (cons xs null)]))
    (define (pre-remove xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-remove xs r) (equal? r (spec-remove xs)))
    *ENDLIBSPEC*/
    fn remove(&mut self, elt: T) -> Option<T> {
        match BTreeSet::remove(self, &elt) {
            true => Some(elt),
            false => None
        }
    }
}

/*IMPL*
WithPosition
*ENDIMPL*/
impl<T: Ord> WithPosition<T> for BTreeSet<T> {
    /*LIBSPEC*
    /*OPNAME*
    first spec-first pre-first post-first
    *ENDOPNAME*/
    (define (spec-first xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (first xs))]))
    (define (pre-first xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-first xs r) (equal? r (spec-first xs)))
    *ENDLIBSPEC*/
    fn first(&mut self) -> Option<&T> {
        BTreeSet::first(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    last spec-last pre-last post-last
    *ENDOPNAME*/
    (define (spec-last xs)
      (cond
        [(null? xs) (cons xs null)]
        [else (cons xs (last xs))]))
    (define (pre-last xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-last xs r) (equal? r (spec-last xs)))
    *ENDLIBSPEC*/
    fn last(&mut self) -> Option<&T> {
        BTreeSet::last(self)
    }

    /*LIBSPEC*
    /*OPNAME*
    nth spec-nth pre-nth post-nth
    *ENDOPNAME*/
    (define (spec-nth xs n)
      (cond
        [(>= n (length xs)) (cons xs null)]
        [(< n 0) (cons xs null)]
        [else (cons xs (list-ref xs n))]))
    (define (pre-nth xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-nth n xs r) (equal? r (spec-nth xs n)))
    *ENDLIBSPEC*/
    fn nth(&mut self, n: usize) -> Option<&T> {
        BTreeSet::iter(self).nth(n)
    }                                      
}

#[cfg(test)]
mod tests {
    use crate::traits::{Container, WithPosition};
    use std::collections::BTreeSet;

    #[test]
    fn test_treeset_container_trait() {
        let set : &mut dyn Container<u32> = &mut BTreeSet::<u32>::new();
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