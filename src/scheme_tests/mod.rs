#[cfg(test)]
mod tests{
    use crate::scheme::run_scheme;

    #[test]
    fn basic_expr() {
        let s = String::from("(+ 4 5 (* 7 8))");
        assert_eq!(run_scheme(s), "65\n");

        let s = String::from("(* 2 5)");
        assert_eq!(run_scheme(s), "10\n");

        let s = String::from("(number=? (+ 1 2 3 4) 10)");
        assert_eq!(run_scheme(s), "true\n");

        let s = String::from("(if (number=? 2 1) 2 3)");
        assert_eq!(run_scheme(s), "3\n");

        let s = String::from("(cond (number=? 1 1) 2 (number=? 1 2) 3)");
        assert_eq!(run_scheme(s), "2\n");

        let s = String::from("(+ 3 -1)");
        assert_eq!(run_scheme(s), "2\n");

        let s = String::from("(- 4 2)");
        assert_eq!(run_scheme(s), "2\n");
    }

    #[test]
    fn constants() {
        let s = String::from("(define t 2)\n(define x (+ 1 1))\n(number=? t x)\n(number=? t 3)");
        assert_eq!(run_scheme(s), "true\nfalse\n");
    }
    
    #[test]
    fn function_def() {
        let s = String::from("(define (plus a b) (+ a b))\n(plus 3 4)\n
            (define (always-first x) (if (number=? 0 0) x (always-first x)))\n(always-first 4)");
        assert_eq!(run_scheme(s), "7\n4\n")
    }

    #[test]
    fn factorial() {
        let s = String::from("(define (fact n) (if (number=? n 0) 1 (* n (fact (- n 1)))))
            \n(fact 4)\n(fact 5)\n(fact 6)\n(fact 7)");
        assert_eq!(run_scheme(s), "24\n120\n720\n5040\n");
    }

    #[test]
    fn lambdas() {
        let s = String::from("((lambda (x y) (+ x y)) 2 5)\n((lambda (x) (+ x 1)) 1)");
        assert_eq!(run_scheme(s), "7\n2\n");
    }

    #[test]
    fn pairs() {
        let s = String::from("(define (pair x y) (lambda (b) (if b x y)))\n(define (first p) (p true))\n
            (define (second p) (p false))\n(first (pair 4 (pair 3 2)))\n(first (second (pair 4 (pair 3 2))))\n
            (second (second (pair 4 (pair 3 2))))");
        assert_eq!(run_scheme(s), "4\n3\n2\n");
    }

    #[test]
    fn lambda_def() {
        let s = String::from("(define add_one (lambda (x) (+ x 1)))\n(add_one 1)");
        assert_eq!(run_scheme(s), "2\n");
    }

    #[test]
    fn scoped_def() {
        let s = String::from("(define x 1)\n(define (f x) x)\n(f 2)");
        assert_eq!(run_scheme(s), "2\n");

        let s = String::from("(define (f x) x)\n(define x 1)\n(f 2)");
        assert_eq!(run_scheme(s), "2\n");
    }

    #[test]
    fn higher_order() {
        let s = String::from("(define (foo x) (x 3 4))\n(foo +)");
        assert_eq!(run_scheme(s), "7\n");
    }
}
