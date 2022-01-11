#lang rosette
; The list model

; (spec-length lst) -> pair?
; The return type is a pair with the first element being the length of the list
; and the second element being the list itself.
(define (spec-length xs)
  (cons (length xs) xs))

(define (pre-length xs) #t)

; (post-length lst result) -> boolean?
; The result of the function to be verfied should produce the same result of the
; specification, i.e., the same integer representing the length and the unchanged
; list.
(define (post-length xs r)
  (equal? r (spec-length xs)))


; (spec-contains elem lst) -> pair?
; The return type is a pair with the first element being a boolean value, which
; indicates the elem is in the lst when it's true and the elem is not in the lst
; when it's false; and the second element being the list itself.
(define (spec-contains x xs)
  (cond
    [(list? (member x xs)) (cons #t xs)]
    [else (cons #f xs)]))

(define (pre-contains xs) #t)

; (post-contains elem lst result) -> boolean?
; The result of the function to be verfied should produce the same result of the
; specification, i.e., the same boolean value representing if the elem is in
; the lst or not and the unchanged list.
(define (post-contains x xs r)
  (equal? (r (spec-contains x xs))))


; (spec-insert elem lst) -> list?
(define (spec-insert x xs)
  (append (list x) xs))

(define (pre-insert xs) #t)
(define (post-insert x xs ys)
  (equal? ys (spec-insert x xs)))


; (spec-remove elem lst) -> list?
(define (spec-remove x xs)
  (remove x xs))

(define (pre-remove xs) #t)
(define (post-remove x xs ys)
  (equal? ys (spec-remove x xs)))

(provide spec-length spec-contains spec-insert spec-remove)