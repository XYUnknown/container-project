#lang rosette
; (generate-list n) -> list?
; Generate a list of symbolic integer values of length n
(define (generate-list n)
    (define-symbolic* y integer? #:length n)
    y)

(define (check-spec-len prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))

(define (check-spec-is-empty prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))

(define (check-spec-first prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))

(define (check-spec-last prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))

(define (check-spec-contains prop pre spec xs x)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs x)))))

(define (check-spec-insert prop pre spec xs x)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (spec xs x))))

(define (check-spec-pop prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (car (spec xs)))))

(define (check-spec-clear prop pre spec xs)
  (assume (and (prop xs) (pre xs)))
  (assert (prop (spec xs))))

(define (check-not-contradict prop pre xs)
  (assert (and (prop xs) (pre xs) (> (length xs) 1))))

(define (check prop pres specs xs x)
  (cond 
    [(or (unsat? (solve (check-not-contradict prop (first pres) xs)))
         (unsat? (solve (check-not-contradict prop (second pres) xs)))
         (unsat? (solve (check-not-contradict prop (third pres) xs)))
         (unsat? (solve (check-not-contradict prop (fourth pres) xs)))
         (unsat? (solve (check-not-contradict prop (fifth pres) xs)))
         (unsat? (solve (check-not-contradict prop (sixth pres) xs)))
         (unsat? (solve (check-not-contradict prop (seventh pres) xs)))
         (unsat? (solve (check-not-contradict prop (eighth pres) xs)))) #f]
    [else (and (unsat? (verify (check-spec-clear prop (first pres) (first specs) xs)))
               (unsat? (verify (check-spec-contains prop (second pres) (second specs) xs x)))
               (unsat? (verify (check-spec-first prop (third pres) (third specs) xs)))
               (unsat? (verify (check-spec-insert prop (fourth pres) (fourth specs) xs x)))
               (unsat? (verify (check-spec-is-empty prop (fifth pres) (fifth specs) xs)))
               (unsat? (verify (check-spec-last prop (sixth pres) (sixth specs) xs)))
               (unsat? (verify (check-spec-len prop (seventh pres) (seventh specs) xs)))
               (unsat? (verify (check-spec-pop prop (eighth pres) (eighth specs) xs))))]))


; The list
(define-symbolic elem integer?)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 10) len))

(provide check elem ls)