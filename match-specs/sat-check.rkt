#lang rosette
(require "properties.rkt")
(require "list.rkt")
(require "unique-list.rkt")
(require "ascending-list.rkt")
(require "strict-ascending-list.rkt")
(require "utils.rkt")

(define (check-spec-length prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (cdr (spec xs)))))

(define (check-spec-contains prop pre spec x xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (cdr (spec x xs)))))

(define (check-spec-insert prop pre spec x xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (spec x xs))))

(define (check-not-contradict prop pre xs)
  (assert (and (prop xs) (pre xs) (> (length xs) 1))))

(define (check-spec-remove prop pre spec x xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (spec x xs))))

(define (check prop pres specs x xs)
  (cond 
    [(or (unsat? (solve (check-not-contradict prop (first pres) xs)))
          (unsat? (solve (check-not-contradict prop (second pres) xs)))
          (unsat? (solve (check-not-contradict prop (third pres) xs)))
          (unsat? (solve (check-not-contradict prop (fourth pres) xs)))) #f]
    [else (and (unsat? (verify (check-spec-length prop (first pres) (first specs) xs)))
               (unsat? (verify (check-spec-contains prop (second pres) (second specs) x xs)))
               (unsat? (verify (check-spec-insert prop (third pres) (third specs) x xs)))
               (unsat? (verify (check-spec-remove prop (fourth pres) (fourth specs) x xs))))]))


; The list
(define-symbolic elem integer?)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 10) len))

; Uncomment to run
; (check unique (list pre-length pre-contains pre-insert pre-remove) (list spec-length spec-contains spec-insert spec-remove) elem ls)
; #f

; (check ascending (list pre-length pre-contains pre-insert pre-remove) (list spec-length spec-contains spec-insert spec-remove) elem ls)
; #f

; (check unique (list pre-length-unique pre-contains-unique pre-insert-unique pre-remove-unique) (list spec-length-unique spec-contains-unique spec-insert-unique spec-remove-unique) elem ls)
; #t

; (check ascending (list pre-length-unique pre-contains-unique pre-insert-unique pre-remove-unique) (list spec-length-unique spec-contains-unique spec-insert-unique spec-remove-unique) elem ls)
; #f

; (check (lambda (x) (and (unique x) (ascending x))) (list pre-length-unique pre-contains-unique pre-insert-unique pre-remove-unique) (list spec-length-unique spec-contains-unique spec-insert-unique spec-remove-unique) elem ls)
; #f

; (check ascending (list pre-length-ascending pre-contains-ascending pre-insert-ascending pre-remove-ascending) (list spec-length-ascending spec-contains-ascending spec-insert-ascending spec-remove-ascending) elem ls)
; #t

; (check unique (list pre-length-ascending pre-contains-ascending pre-insert-ascending pre-remove-ascending) (list spec-length-ascending spec-contains-ascending spec-insert-ascending spec-remove-ascending) elem ls)
; #f

; (check (lambda (x) (and (unique x) (ascending x))) (list pre-length-ascending pre-contains-ascending pre-insert-ascending pre-remove-ascending) (list spec-length-ascending spec-contains-ascending spec-insert-ascending spec-remove-ascending) elem ls)
; #f

; (check (lambda (x) (and (unique x) (ascending x))) (list pre-length-strict-ascending pre-contains-strict-ascending pre-insert-strict-ascending pre-remove-strict-ascending) (list spec-length-strict-ascending spec-contains-strict-ascending spec-insert-strict-ascending spec-remove-strict-ascending) elem ls)
; #t

; (check unique (list pre-length-strict-ascending pre-contains-strict-ascending pre-insert-strict-ascending pre-remove-strict-ascending) (list spec-length-strict-ascending spec-contains-strict-ascending spec-insert-strict-ascending spec-remove-strict-ascending) elem ls)
; #t

; (check ascending (list pre-length-strict-ascending pre-contains-strict-ascending pre-insert-strict-ascending pre-remove-strict-ascending) (list spec-length-strict-ascending spec-contains-strict-ascending spec-insert-strict-ascending spec-remove-strict-ascending) elem ls)
; #t

; (check (lambda (x) (not (unique x))) (list pre-length-strict-ascending pre-contains-strict-ascending pre-insert-strict-ascending pre-remove-strict-ascending) (list spec-length-strict-ascending spec-contains-strict-ascending spec-insert-strict-ascending spec-remove-strict-ascending) elem ls)
; #f