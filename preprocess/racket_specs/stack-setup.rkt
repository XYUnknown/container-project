#lang rosette

(define (check-push-pop prop pres xs)
  (assume (and ((first pres) xs) ((second pres) xs)))
  (assert (prop xs)))

(define (check prop pres specs xs x)
  (unsat? (verify (check-push-pop prop pres xs))))

(provide check)