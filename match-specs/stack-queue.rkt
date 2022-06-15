#lang rosette
(define (lifo push pop)
  (lambda (l)
    (forall (list elem) (equal? (cdr (pop (push l elem))) elem))))

(define (spec-push xs x) (append xs (list x)))
(define (pre-push xs) #t)
(define (post-push xs x ys) (equal? ys (spec-push xs x)))
(define (spec-pop xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons (take xs (- (length xs) 1)) (last xs))]))
(define (pre-pop xs) #t)
(define (post-pop xs r) (equal? r (spec-pop xs)))


(define (pre-push-unique-sat xs) (equal? xs (remove-duplicates xs)))
(define (spec-push-unique-sat xs x)
  (append (filter (lambda (e) (not (equal? e x))) xs) (list x)))

(define (pre-pop-unique-sat xs) (equal? xs (remove-duplicates xs)))
(define (spec-pop-unique-sat xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons (take xs (- (length xs) 1)) (last xs))]))

(define (pre-push-unique-alt xs) (equal? xs (remove-duplicates xs)))
(define (spec-push-unique-alt xs x)
  (remove-duplicates (append xs (list x))))
;(define (spec-push-unique-alt xs x)
;  (cond
;    [(list? (member x xs)) xs]
;    [else (append xs (list x))]))

(define (pre-pop-unique-alt xs) (equal? xs (remove-duplicates xs)))
(define (spec-pop-unique-alt xs)
  (cond
    [(null? xs) (cons xs null)]
    [else (cons (take xs (- (length xs) 1)) (last xs))]))


(define (generate-list n)
    (define-symbolic* y integer? #:length n)
    y)

; The list
(define-symbolic elem integer?)
(define-symbolic len (bitvector 32))
(define ls (take-bv (generate-list 5) len))

;(verify (assert ((lifo spec-push spec-pop) ls elem)))
(define (check-unique-stack-sat xs)
  (assume (and (pre-push-unique-sat xs) (pre-pop-unique-sat xs)))
  (assert ((lifo spec-push-unique-sat spec-pop-unique-sat) xs)))

(define (check-unique-stack-alt xs)
  (assume (and (pre-push-unique-alt xs) (pre-pop-unique-alt xs)))
  (assert ((lifo spec-push-unique-alt spec-pop-unique-alt) xs)))
