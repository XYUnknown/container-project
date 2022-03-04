/*LIBSPEC-NAME*
rust-hashset-spec std::collections::HashSet
*ENDLIBSPEC-NAME*/

use std::collections::HashSet;
use std::hash::Hash;
use crate::traits::Container;

/*IMPL*
Container
*ENDIMPL*/
impl<T: Ord + Hash> Container<T> for HashSet<T> {

    /*LIBSPEC*
    /*OPNAME*
    len spec-len pre-len post-len
    *ENDOPNAME*/
    (define (spec-len xs) (cons xs (length xs)))
    (define (pre-len xs) (equal? xs (remove-duplicates (sort xs <))))
    (define (post-len xs r) (equal? r (spec-len xs)))
    *ENDLIBSPEC*/
    fn len(&mut self) -> usize {
        HashSet::len(self)
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
        HashSet::contains(self, x)
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
        HashSet::is_empty(self)
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
        HashSet::clear(self);
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
        HashSet::insert(self, elt);
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
        match HashSet::remove(self, &elt) {
            true => Some(elt),
            false => None
        }
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