This is an interpreter for a subset of the scheme language.

It is written in the rust language and uses the cargo package manager.
To utalize it you may want to install the rust language and cargo.

If you wish to utalize it you will have to clone this git repository.

```bash
git clone https://github.com/cinnamonbacon/schemeInterpreter
```

If you then have cargo you can run the following command to build the executable.
```bash
cargo build
```

The following commands are implemented as of the last edit of this file:
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
