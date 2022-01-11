#lang rosette
; (generate-list n) -> list?
; Generate a list of symbolic integer values of length n
(define (generate-list n)
    (define-symbolic* y integer? #:length n)
    y)

(provide generate-list)