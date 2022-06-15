#lang rosette
; Combinators
; list -> boolean
; we choose the list as our model for specifications
; The binary combinators
(define (for-all-unique-pairs l fn)
  (foldl elem-and #t
         (flatten
          (map (lambda (a)
                 (map (lambda (b) (fn a b)) (remove a l))) l))))

(define (for-all-consecutive-pairs l fn)
  (foldl elem-and #t
         (map (lambda (p) (fn (first p) (second p))) (consecutive-pairs l))))

; The unary combinator
(define (for-all-elems l fn)
  (foldl elem-and #t
         (map (lambda (a) (fn a)) l)))

; Helpers
; (elem-and a b) -> boolean
; Since the and operator in Racket is a syntax instead of a procedure,
; we need to create an and procedure which can be used as a parameter
; of a procedure
(define (elem-and a b) (and a b))

; (not-equal? a b) -> boolean?
(define (not-equal? a b) (not (equal? a b)))

; (leq? a b) -> boolean?
(define (leq? . args)
  (cond [(andmap string? args) (apply string<=? args)]
        [(andmap char? args) (apply char<=? args)]
        [else (apply <= args)]))

; (geq? a b) -> boolean?
(define (geq? . args)
  (cond [(andmap string? args) (apply string>=? args)]
        [(andmap char? args) (apply char>=? args)]
        [else (apply >= args)]))

; (contains? elem lst) -> boolean?
(define (contains x l)
  (cond
    [(list? (member x l)) #t]
    [else #f]))

; (unique-count? elem lst) -> boolean
; Checking if the occurance of the elem in the lst is exactly once
(define (unique-count? x l)
  (= 1 (count (lambda (y) (= x y)) l)))

; (once? elem lst) -> boolean?
; The equalvent version of unique-count?,
; created to compare the performance of the solver
(define (once? x l)
  (cond
    [(empty? l) #f]
    [else (or (and (= x (first l)) (not (contains x (rest l))))
              (and (not (= x (first l))) (once? x (rest l))))]))

; (consecutive-pairs lst) -> list?
; Obtaining all consecutive pairs of elements of a given list
; Examples:
; > (consecutive-pairs '(1 2 3))
; '((1 2) (2 3))
; > (consecutive-pairs '(1))
; '()
; > (consecutive-pairs null)
; '()
(define (consecutive-pairs l)
  (cond
    [(< (length l) 2) null]
    [else (append (list (take l 2)) (consecutive-pairs (drop l 1)))]))

; Export procedures
(provide for-all-unique-pairs for-all-consecutive-pairs for-all-elems elem-and not-equal? leq? geq? unique-count?)