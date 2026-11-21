#lang racket
(define chinese '(ling yi er san si wu liu qi ba jiu shi))
(define english '(zero one two three four five six seven eight nine ten))

; returns the number for word, or #f if it is not in either list
(define (lookup word ch en n)
  (cond ((null? ch) #f)
        ((or (eq? word (car ch)) (eq? word (car en))) n)
        (else (lookup word (cdr ch) (cdr en) (+ n 1)))))

(define (translate lst)
  (filter number? (map (lambda (w) (lookup w chinese english 0)) lst)))

; prints the items of lst with sep between them
(define (print-with lst sep)
  (cond ((null? lst) (void))
        ((null? (cdr lst)) (display (car lst)))
        (else (display (car lst)) (display sep) (print-with (cdr lst) sep))))

(define (go lst)
  (define nums (translate lst))
  (display "Translation: ") (print-with nums " ") (newline)
  (display "Addition: ") (print-with nums " + ")
  (display " = ") (display (apply + nums)) (newline)
  (display "Multiplication: ") (print-with nums " * ")
  (display " = ") (display (apply * nums)) (newline))

(go '(yi nine six ba))
(go '(yi josh three si))
