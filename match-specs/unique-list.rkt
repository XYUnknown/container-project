#lang rosette
(require "properties.rkt")
; The unique list model

; (spec-length-unique lst) -> pair?
(define (spec-length-unique xs)
  (cons (length xs) xs))

(define (pre-length-unique xs) (equal? xs (remove-duplicates xs)))
(define (post-length-unique xs r)
  (equal? r (spec-length-unique xs)))

; (spec-contains-unique elem lst) -> pair?
(define (spec-contains-unique x xs)
  (cond
    [(list? (member x xs)) (cons #t xs)]
    [else (cons #f xs)]))

(define (pre-contains-unique xs) (equal? xs (remove-duplicates xs)))
(define (post-contains-unique x xs r)
  (equal? r (spec-contains-unique x xs)))

; (spec-insert-unique elem lst) -> lst
(define (spec-insert-unique x xs)
  (remove-duplicates (append (list x) xs)))

(define (pre-insert-unique xs) (equal? xs (remove-duplicates xs)))
(define (post-insert-unique x xs r)
  (equal? r (spec-insert-unique x xs)))

; (spec-remove-unique elem lst) -> lst
(define (spec-remove-unique x xs)
  (remove x xs))

(define (pre-remove-unique xs) (equal? xs (remove-duplicates xs)))
(define (post-remove-unique x xs r)
  (equal? r (spec-remove-unique x xs)))