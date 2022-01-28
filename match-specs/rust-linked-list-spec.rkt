#lang rosette
; The list model

; (spec-length lst) -> pair?
(define (spec-length xs)
  (cons xs (length xs)))

(define (pre-length xs) #t)
(define (post-length xs r)
  (equal? r (spec-length xs)))

; (spec-contains elem lst) -> pair?
(define (spec-contains xs x)
  (cond
    [(list? (member x xs)) (cons xs #t)]
    [else (cons xs #f)]))

(define (pre-contains xs) #t)
(define (post-contains xs x r)
  (equal? r (spec-contains xs x)))

; (spec-is-empty lst) -> pair?
(define (spec-is-empty xs)
  (cons xs (null? xs)))

(define (pre-is-empty xs) #t)
(define (post-is-empty xs r)
  (equal? r (spec-is-empty xs)))

; (spec-push elem lst) -> list?
(define (spec-push xs x)
  (append xs (list x)))

(define (pre-push xs) #t)
(define (post-push xs x ys)
  (equal? ys (spec-push xs x)))

; (spec-pop lst) -> pair?
(define (spec-pop xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons (take xs (- (length xs) 1)) (last xs))]))

(define (pre-pop xs) #t)
(define (post-pop xs r)
  (equal? r (spec-pop xs)))

; (spec-first lst) -> pair?
(define (spec-first xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons xs (first xs))]))

(define (pre-first xs) #t)
(define (post-first xs r)
  (equal? r (spec-first xs)))

; (spec-last lst) -> pair?
(define (spec-last xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons xs (last xs))]))

(define (pre-last xs) #t)
(define (post-last xs r)
  (equal? r (spec-last xs)))

; (clear lst) -> list?
(define (spec-clear xs) null)
(define (pre-clear xs) #t)
(define (post-clear xs r)
  (equal? r (spec-clear xs)))