#lang rosette
(define (lifo push pop) (lambda (l x) (equal? (cdr (pop (push l x))) x)))


(define (spec-push xs x) (append xs (list x)))
(define (pre-push xs) #t)
(define (post-push xs x ys) (equal? ys (spec-push xs x)))
(define (spec-pop xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons (take xs (- (length xs) 1)) (last xs))]))
(define (pre-pop xs) #t)
(define (post-pop xs r) (equal? r (spec-pop xs)))

(define (generate-list n)
    (define-symbolic* y integer? #:length n)
    y)

(define (pre-push-unique-sat xs) (equal? xs (remove-duplicates xs)))
(define (spec-push-unique-sat xs x)
  (append (filter (lambda (e) (not (equal? e x))) xs) (list x)))

(define (spec-pop-unique-sat xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons (take xs (- (length xs) 1)) (last xs))]))


; The list
(define-symbolic elem integer?)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 10) len))

;(verify (assert ((lifo spec-push spec-pop) ls elem)))
(define (check-unique-stack-sat xs x)
  (assume (pre-push-unique-sat xs))
  (assert ((lifo spec-push-unique-sat spec-pop-unique-sat) xs x)))