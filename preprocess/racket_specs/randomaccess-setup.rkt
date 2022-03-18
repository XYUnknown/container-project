#lang rosette
; (generate-list n) -> list?
; Generate a list of symbolic integer values of length n
(define (generate-list n)
    (define-symbolic* y integer? #:length n)
    y)

(define (check-not-contradict prop pre xs)
  (assert (and (prop xs) (pre xs) (> (length xs) 1))))

(define (check-spec-first prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))

(define (check-spec-last prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))


(define (check-spec-nth prop pre spec xs n)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs n)))))


(define (check prop pres specs xs n)
  (cond 
    [(or (unsat? (solve (check-not-contradict prop (first pres) xs)))
         (unsat? (solve (check-not-contradict prop (second pres) xs)))
         (unsat? (solve (check-not-contradict prop (third pres) xs)))
         ) #f]
    [else (and (unsat? (verify (check-spec-first prop (first pres) (first specs) xs)))
               (unsat? (verify (check-spec-last prop (second pres) (second specs) xs)))
               (unsat? (verify (check-spec-nth prop (third pres) (third specs) xs n)))
               )]))


; The list
(define-symbolic n integer?)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 10) len))

(provide check n ls)