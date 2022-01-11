#lang rosette
(require "properties.rkt")
; The list model

; (length-spec lst) -> pair?
; The return type is a pair with the first element being the length of the list
; and the second element being the list itself.
(define (length-spec xs)
  (cons (length xs) xs))

(define (pre-len xs) #t)

; (post-len lst result) -> boolean?
; The result of the function to be verfied should produce the same result of the
; specification, i.e., the same integer representing the length and the unchanged
; list.
(define (post-len xs r)
  (equal? r (length-spec xs)))


; (contains-spec elem lst) -> pair?
; The return type is a pair with the first element being a boolean value, which
; indicates the elem is in the lst when it's true and the elem is not in the lst
; when it's false; and the second element being the list itself.
(define (contains-spec x xs)
  (cond
    [(list? (member x xs)) (cons #t xs)]
    [else (cons #f xs)]))

(define (pre-contains xs) #t)

; (post-contains elem lst result) -> boolean?
; The result of the function to be verfied should produce the same result of the
; specification, i.e., the same boolean value representing if the elem is in
; the lst or not and the unchanged list.
(define (post-contains x xs r)
  (equal? (r (contains-spec x xs))))


; (insert-spec elem lst) -> list?
(define (insert-spec x xs)
  (append (list x) xs))

(define (pre-ins xs) #t)
(define (post-ins x xs ys)
  (equal? ys (insert-spec x xs)))


; (remove-spec elem lst) -> list?
(define (remove-spec x xs)
  (remove x xs))

(define (pre-remove xs) #t)
(define (post-remove x xs ys)
  (equal? ys (remove-spec x xs)))