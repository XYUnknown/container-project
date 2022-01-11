#lang rosette
; The ascending list model

; (spec-length-ascending lst) -> pair?
(define (spec-length-ascending xs)
  (cons (length xs) xs))

(define (pre-length-ascending xs) (equal? xs (sort xs <)))
(define (post-length-ascending xs r)
  (equal? r (spec-length-ascending xs)))

; (spec-contains-ascending elem lst) -> pair?
(define (spec-contains-ascending x xs)
  (cond
    [(list? (member x xs)) (cons #t xs)]
    [else (cons #f xs)]))

(define (pre-contains-ascending xs) (equal? xs (sort xs <)))
(define (post-contains-ascending x xs r)
  (equal? r (spec-contains-ascending x xs)))

; (spec-insert-ascending elem lst) -> lst
(define (spec-insert-ascending x xs)
  (sort (append (list x) xs) <))

(define (pre-insert-ascending xs) (equal? xs (sort xs <)))
(define (post-insert-ascending x xs r)
  (equal? r (spec-insert-ascending x xs)))

; (spec-remove-ascending elem lst) -> lst
(define (spec-remove-ascending x xs)
  (remove x xs))

(define (pre-remove-ascending xs) (equal? xs (sort xs <)))
(define (post-remove-ascending x xs r)
  (equal? r (spec-remove-ascending x xs)))


(provide spec-length-ascending spec-contains-ascending spec-insert-ascending spec-remove-ascending)