#lang racket
(define first (lambda (x) (car x)))
(define second (lambda (x) (cadr x)))
(define rest (lambda (x) (cdr x)))
(define third (lambda (x) (caddr x)))
(define fourth (lambda (x) (cadddr x)))
(define fifth (lambda (x) (car (cddddr x))))

(define (truecount lst)
  (cond ((null? lst) 0)
        ((eq? (car lst) #t) (+ 1 (truecount (cdr lst))))
        (else (truecount (cdr lst)))))

(define (squarelist lst)
  (map (lambda (x) (* x x)) lst))

(define (hundreds? lst)
  (filter (lambda (x) (> x 100)) lst))

(define (collatz n)
  (display n) (newline)
  (cond ((eq? n 1) (void))
        ((even? n) (collatz (/ n 2)))
        (else (collatz (+ (* 3 n) 1)))))

(define family '(josh sara erin sandy jon))
(third family)
(fourth family)
(fifth family)
(truecount '(#t #f #t #t #f))
(squarelist '(1 2 3 4))
(hundreds? '(50 150 99 101 300))
(collatz 6)
