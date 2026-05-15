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
to run the code. Give the names of the files you wish to run through the interpreter as arguments. In the folder examples there are some files that you can try running the interpreter on.


The following commands are implemented as of now:
adding, multiplication, and subtraction
```scheme
(+ 3 4 (* 2 (- 3 4)))
```
For example read as 3 + 4 + (2 * (3 - 4)) reduces to 5.

The boolean statement number=? and if and cond are implemented. Consider the examples
```scheme
(if (number=? 1 2) 1 2)
```
As 1 does not equal 2 the second expression is taken and the result is 2.

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
(+ x 2)
```

You may also want to play around with lambda functions which are implemented. Consider the implementation of pairs.
```scheme
(define (pair x y) (lambda (b) (if b x y)))
(define (first p) (p true))
(define (second p) (p false))
```
Lists can then be implemented from here by nesting pairs.
