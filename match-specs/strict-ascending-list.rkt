#lang rosette
; (spec-length-strict-ascending lst) -> pair?
(define (spec-length-strict-ascending xs)
  (cons (length xs) xs))

(define (pre-length-strict-ascending xs) (equal? xs (remove-duplicates (sort xs <))))
(define (post-length-strict-ascending xs r)
  (equal? r (spec-length-strict-ascending xs)))

; (spec-contains-strict-ascending elem lst) -> pair?
(define (spec-contains-strict-ascending x xs)
  (cond
    [(list? (member x xs)) (cons #t xs)]
    [else (cons #f xs)]))

(define (pre-contains-strict-ascending xs) (equal? xs (remove-duplicates (sort xs <))))
(define (post-contains-strict-ascending x xs r)
  (equal? r (spec-contains-strict-ascending x xs)))

; (spec-insert-strict-ascending elem lst) -> list?
(define (spec-insert-strict-ascending x xs)
  (remove-duplicates (sort (append (list x) xs) <)))

(define (pre-insert-strict-ascending xs) (equal? xs (remove-duplicates (sort xs <))))
(define (post-insert-strict-ascending x xs r)
  (equal? r (spec-insert-strict-ascending x xs)))

; (spec-remove-strict-ascending elem lst) -> list?
(define (spec-remove-strict-ascending x xs)
  (remove x xs))

(define (pre-remove-strict-ascending xs) (equal? xs (remove-duplicates (sort xs <))))
(define (post-remove-strict-ascending x xs r)
  (equal? r (spec-remove-strict-ascending x xs)))


(provide pre-length-strict-ascending pre-contains-strict-ascending pre-insert-strict-ascending pre-remove-strict-ascending spec-length-strict-ascending spec-contains-strict-ascending spec-insert-strict-ascending spec-remove-strict-ascending)