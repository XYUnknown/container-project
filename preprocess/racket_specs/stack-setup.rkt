#lang rosette

(define (check prop pres specs xs)
  (assume (and ((first pres) xs) ((second pres) xs)))
  (assert (prop xs)))

(provide check)