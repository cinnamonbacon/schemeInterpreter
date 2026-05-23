use crate::scheme::Expr;

use std::ops;
use phf::{phf_set, Set};
use gcd::Gcd;

pub static SUPPORTED_OPPERATIONS: Set<&'static str> = phf_set! {
    "+",
    "-",
    "*",
    "/",
    "number=?",
    "if",
    "cond",
    "lambda",
};

#[derive(Debug)]
pub enum Val{
    Number(bool, u32, u32),
    Boolean(bool),
    Function(Vec<String>, Expr),
    SupportedFunction(String),
    SchemeError(),
}

use Val::*;

impl Clone for Val{
    fn clone(&self) -> Self {
        match self{
            Number(b, n, d) => Number(*b, *n, *d),
            Boolean(b) => Boolean(*b),
            Function(bindings, exp) => Function(bindings.clone(), exp.clone()),
            SupportedFunction(s) => SupportedFunction(s.clone()),
            SchemeError() => SchemeError(),
        }
    }
}


impl ops::Add<Val> for Val {
    type Output = Val;

    fn add(self, rhs: Val) -> Val {
        if let Number(neg, n ,d) = self{
            if let Number(oneg, on, od) = rhs{
                let mut new_denom = d * od / d.gcd(od);
                let mut n = n * new_denom / od;
                let on = on * new_denom / d;
                let mut neg = neg;

                if n > on {
                    n = if neg == oneg {n + on} else {n - on};
                }
                else {
                    n = if neg == oneg {on + n} else {on - n};
                    neg = oneg;
                }

                if n == 0 {
                    new_denom = 1;
                    neg = false;
                }

                return Number(neg, n, new_denom);
            }
        }
        SchemeError()
    }
}

impl ops::Sub<Val> for Val {
    type Output = Val;

    fn sub(self, rhs: Val) -> Val {
        if let Number(neg, n ,d) = self{
            if let Number(oneg, on, od) = rhs{
                let mut new_denom = d * od / d.gcd(od);
                let mut n = n * new_denom / od;
                let on = on * new_denom / d;
                let mut neg = neg;
                let oneg = !oneg;

                if n > on {
                    n = if neg == oneg {n + on} else {n - on};
                }
                else {
                    n = if neg == oneg {on + n} else {on - n};
                    neg = oneg;
                }

                if n == 0 {
                    new_denom = 1;
                    neg = false;
                }

                return Number(neg, n, new_denom);
            }
        }
        SchemeError()
    }
}

impl ops::Mul<Val> for Val {
    type Output = Val;

    fn mul(self, rhs: Val) -> Val {
        if let Number(neg, n ,d) = self{
            if let Number(oneg, on, od) = rhs{
                let n = n * on;
                let d = d * od;
                let common = n.gcd(d);
                let n = n / common;
                let d = d / common;

                let neg = neg != oneg;
                return Number(neg, n, d);
            }
        }
        SchemeError()
    }
}

impl ops::Div<Val> for Val {
    type Output = Val;

    fn div(self, rhs: Val) -> Val {
        if let Number(neg, n ,d) = self{
            if let Number(oneg, od, on) = rhs{
                let n = n * on;
                let d = d * od;
                let common = n.gcd(d);
                let n = n / common;
                let d = d / common;

                let neg = neg != oneg;
                return Number(neg, n, d);
            }
        }
        SchemeError()
    }
}
