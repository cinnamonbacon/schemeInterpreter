This is an interpreter for a subset of the scheme language.

It is written in the rust language and uses the cargo package manager.
To utilize it you may want to install the rust language and cargo.

If you wish to utilize it you will have to clone this git repository.

```bash
git clone https://github.com/cinnamonbacon/schemeInterpreter
```

If you then have cargo you can run the following command to build the executable.
```bash
cargo build
```

The executable will be built and can be found at target/debug/scheme but you can also use the command
```
cargo run
```
to run the code. Give the names of the files you wish to run through the interpreter as arguments. If no argument is given it will read from stdin until Ctrl-D is pressed (EOF) and then output the result. 

In the folder examples there are some files that you can try running the interpreter on.


The following commands are implemented as of now:
adding, multiplication, subtraction, and division
```scheme
(/ (+ 3 4 (* 2 (- 3 4))) 5)
```
For example read as (3 + 4 + (2 * (3 - 4))) / 5 reduces to 1.

The boolean statement number=? and if and cond are implemented. Consider the examples
```scheme
(if (number=? 1 2) 1 2)
```
As 1 does not equal 2 the second expression is taken and the result is 2.

Cond takes an alternating sequence of boolean statements and values and for the first boolean that evaluates to true it returns the next value. Note that if and cond only evaluate the returned value allowing for reccursion.

Constants are also implemented. For example
```scheme
(define x 4)
(+ x 2)
```
The x is replaced with 4 and the expression reduces to 6.

Function definitions are implemented. For example consider the following
```scheme
(define (add x y) (+ x y))
(add 4 2)
```
Expands the expression to (+ 4 2) and then evaluates to 6.

Putting this all together we can write a basic implementation of factorial
```scheme
(define (fact n) (if (number=? n 0) 1 (* n (fact (- n 1)))))
(fact 4)
```

Pairs are also implemented with cons creating a pair such and car and cdr reading the first and second element respectively. For example
```
(car (cons 1 2))
(cdr (cons 1 2))
```
Evaluate to 1 and 2. Without pairs we could implement the fibonacci sequence such as
```
(define (fib n) (cond (number=? n 0) 1 (number=? n 1) 1 true (+ (fib (- n 1)) (fib (- n 2)))))
```
But the time of this grows exponentially as we are redoing a bunch of work when calculating the (n-1)th and (n-2)th term. Instead we can define (fib-helper n) to return a pair of the nth and (n-1)th term. Then we can add them
```
(define (fib-next p) (cons (+ (car p) (cdr p)) (car p)))
(define (fib-helper n) (cond (number=? n 0) (cons 1 0) (number=? n 1) (cons 1 1) true 
    (fib-next (fib-helper (- n 1)))))
(define (fib n) (car (fib-helper n)))
```

Lists are a nesting of pairs where the first element in each pair is the first of the list and the rest of the elements are stored in the second part of the list. There is also a special list that every list contains which is the empty list. "empty" evaluates to this list and (empty? x) evaluates if something is the empty list. It is false for everything except the empty list
```
(empty? empty)
(empty? 3)
```
The first one gives true where the second gives false. You could create lists with cons such as
```
(cons 1 (cons 2 (cons 3 empty)))
```
but there is a shorthand. That is the list function which takes an arbitrary number of values and creates a list out of them. The value above can similarly be created as
```
(list 1 2 3)
```

You may also want to play around with lambda functions which are implemented. Consider an alternative implementation of pairs.
```scheme
(define (pair x y) (lambda (b) (if b x y)))
(define (first p) (p true))
(define (second p) (p false))
```
