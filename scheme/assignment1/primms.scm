; read-lines [port-or-filename] -- defaults to current input
(define (read-lines . args)
  (let ((p (cond ((null? args) (current-input-port))
                 ((port? (car args)) (car args))
                 ((string? (car args)) (open-input-file (car args)))
                 (else (error 'read-lines "bad argument")))))
    (let loop ((line (read-line p)) (lines (list)))
      (if (eof-object? line)
          (begin (if (and (pair? args) (string? (car args)))
                   (close-input-port p))
                 (reverse lines))
          (loop (read-line p) (cons line lines))))))

(read-lines "edges2.txt")

(define (load-graph filename)
  (define lines (read-lines filename))
  (define num-vertices (string->number (car-str (car lines))))
  (define num-edges (string->number (car-str (cdr-str (car lines)))))
  (list num-vertices num-edges))
(car-str "a2 s")


(load-graph "edges.txt")

(define (car-str str)
  (string-head str (string-find-next-char-set str (char-set #\ #\newline))))

(define (cdr-str str)
  (string-tail str (string-find-next-char str #\ )))
