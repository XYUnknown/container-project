#lang rosette
(require "properties.rkt")
(require "list.rkt")
(require "unique-list.rkt")
(require "utils.rkt")

(define (check-spec-length prop spec xs)
  (prop (cdr (spec xs))))

(define (check-spec-contains prop spec x xs)
  (prop (cdr (spec x xs))))

(define (check-spec-insert prop spec x xs)
  (prop (spec x xs)))

(define (check-spec-remove prop spec x xs)
  (prop (spec x xs)))

(define (check prop specs x xs)
  (assume (prop xs))
  (assert (and (check-spec-length prop (first specs) xs)
               (check-spec-contains prop (second specs) x xs)
               (check-spec-insert prop (third specs) x xs)
               (check-spec-remove prop (fourth specs) x xs))))

; The list
(define-symbolic elem integer?)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 10) len))

; Uncomment to run
; (verify (check unique (list spec-length spec-contains spec-insert spec-remove) elem ls))
; model found

; (verify (check ascending (list spec-length spec-contains spec-insert spec-remove) elem ls))
; model found

; (verify (check unique (list spec-length-unique spec-contains-unique spec-insert-unique spec-remove-unique) elem ls))
; (unsat)

; (verify (check ascending (list spec-length-unique spec-contains-unique spec-insert-unique spec-remove-unique) elem ls))
; model found

; (verify (check (lambda (x) (and (unique x) (ascending x))) (list spec-length-unique spec-contains-unique spec-insert-unique spec-remove-unique) elem ls))
; model found