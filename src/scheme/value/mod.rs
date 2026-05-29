use crate::scheme::ParseTree;

use std::ops;
use std::collections::HashMap;
use phf::{phf_set, Set};
use gcd::Gcd;
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::num::NonZeroUsize;

use lru::LruCache;

pub static SUPPORTED_OPPERATIONS: Set<&'static str> = phf_set! {
    "+",
    "-",
    "*",
    "/",
    "number=?",
    "if",
    "cond",
    "lambda",
    "cons",
    "car",
    "cdr",
    "empty?",
    "list",
    "begin",
};

lazy_static::lazy_static! {
    static ref CACHE: Mutex<HashMap<u64, LruCache<Vec::<Val>, Val>>> = Mutex::new(HashMap::new());
}

static CACHE_SIZE: usize = 50;

pub fn cache_init(n: u64) {
    CACHE.lock().unwrap().insert(n, LruCache::new(NonZeroUsize::new(CACHE_SIZE).unwrap()));
}
pub fn cache_get(n: u64, lst: &Vec::<Val>) -> Option<Val> {
    if let Some(cache) = CACHE.lock().unwrap().get_mut(&n) {
        cache.get(lst).cloned()
    }
    else {
        None
    }
}
pub fn cache_put(n: u64, lst: &Vec::<Val>, val: &Val) {
    if CACHE.lock().unwrap().get(&n).is_none() {
        cache_init(n);
    }
    CACHE.lock().unwrap().get_mut(&n).unwrap().put(lst.clone(), val.clone());
}


static FUNCTION_ID: AtomicUsize = AtomicUsize::new(0);

pub fn next_function_id() -> u64 {
    FUNCTION_ID.fetch_add(1, Ordering::SeqCst).try_into().unwrap()
}

#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq)]
pub enum Val{
    Number(bool, u64, u64),
    Boolean(bool),
    Function(Vec::<String>, Arc<ParseTree>, u64),
    SupportedFunction(Arc<String>),
    Pair(Arc<Val>, Arc<Val>),
    Empty(),
    SchemeError(String),
}

use Val::*;

impl Clone for Val{
    fn clone(&self) -> Self {
        match self{
            Number(b, n, d) => Number(*b, *n, *d),
            Boolean(b) => Boolean(*b),
            Function(bindings, tree, id) => Function(bindings.clone(), tree.clone(), *id),
            SupportedFunction(s) => SupportedFunction(s.clone()),
            Pair(x,y) => Pair(x.clone(), y.clone()),
            Empty() => Empty(),
            SchemeError(s) => SchemeError(s.clone()),
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
        SchemeError("Adding values that are not numbers".to_string())
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
        SchemeError("Subtracting values that are not numbers".to_string())
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
        SchemeError("Multiplying values that are not numbers".to_string())
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
        SchemeError("Dividing values that are not numbers".to_string())
    }
}

impl ToString for Val {
    fn to_string(&self) -> String {
        match self {
            Number(neg, n, d) => {
                format!("{}{n}{}", if *neg {"-"} else {""},
                    if *d != 1 {"/".to_string() + &d.to_string()} else {"".to_string()})
            },
            Boolean(b) => {
                format!("{}", b)
            },
            Function(_bindings, _expr, _id) => "Function".to_string(),
            SupportedFunction(_s) => "Function".to_string(),
            Pair(x, y) => {
                format!("Pair({},{})", x.to_string(), y.to_string())
            },
            Empty() => "empty".to_string(),
            SchemeError(s) => {
                format!("Error: {}", s)
            }
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Number(neg, n, d) => {
                if let Number(oneg, on, od) = other {
                    neg == oneg && n == on && d == od
                }
                else {
                    false
                }
            },
            Boolean(b) => if let Boolean(ob) = other { b == ob } else { false }
            Function(_,_,n) => {
                if let Function(_,_,on) = other {
                    n == on
                }
                else {
                    false
                }
            },
            SupportedFunction(s) => if let SupportedFunction(os) = other { *s == *os } else { false }
            Pair(a, b) => {
                if let Pair(oa, ob) = other{
                    a == oa && b == ob
                }
                else {
                    false
                }
            }
            Empty() => if let Empty() = other { true } else { false }
            SchemeError(_s) => { false }
        }
    }
}
