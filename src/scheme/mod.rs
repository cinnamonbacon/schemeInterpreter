mod scheme_tests;

use std::ops;
use std::collections::HashMap;
use phf::{phf_set, Set};

use gcd::Gcd;

static SUPPORTED_OPPERATIONS: Set<&'static str> = phf_set! {
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
enum Val{
    Number(bool, u32, u32),
    Boolean(bool),
    //Unbound(Expr),
    Function(Vec<String>, Expr),
    SupportedFunction(String),
    //Name(String),
    SchemeError(),
}

use Val::Number;
use Val::Boolean;
//use Val::Unbound;
use Val::Function;
use Val::SupportedFunction;
//use Val::Name;
use Val::SchemeError;

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


#[derive(Debug)]
enum Expr{
    Text(String),
    Bound(Box<Val>),
    Tree(Box<ParseTree>),
}
use Expr::Text;
use Expr::Bound;
use Expr::Tree;

impl Clone for Expr{
    fn clone(&self) -> Self {
        match self{
            Text(s)=> Text(s.clone()),
            Bound(v) => Bound(Box::new(*v.clone())),
            Tree(pt) => {
                let mut ret = ParseTree{ list: Vec::new() };
                for lexp in &pt.list{
                    ret.list.push(lexp.clone());
                }
                Tree(Box::new(ret))
            }
        }
    }
}

impl Expr{
    fn bind_val(self, replace: &String, v: &Val) -> Expr{
        match self{
            Text(s) => if s == *replace { Bound(Box::new(v.clone())) } else{ Text(s) },
            Bound(b) => {
                if let Function(bindings, expr) = *b{
                    if let Some(_) = bindings.iter().position(|s| s == replace){
                        Bound(Box::new(Function(bindings, expr)))
                    }
                    else{
                        Bound(Box::new(Function(bindings, expr.bind_val(replace, v))))
                    }
                }
                else{
                    Bound(b)
                }
            }
            Tree(pt) => {
                let mut ret = ParseTree{ list: Vec::new() };
                if let Text(s) = &pt.list[0] {
                    if s == "lambda" {
                        if let Tree(bindings) = &pt.list[1] {
                            for b in bindings.list.clone(){
                                if let Text(binding_name) = b{
                                    if binding_name == *replace { 
                                        return Tree(pt) 
                                    };
                                }
                            }
                        }
                    }
                }
                for lexp in pt.list{
                    ret.list.push(lexp.bind_val(replace, v));
                }
                Tree(Box::new(ret))
            },
        }
    }
}

#[derive(Debug)]
struct ParseTree{
    list: Vec<Expr>,
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

impl ParseTree{
    fn add_expr(&mut self, ex: Expr){
        self.list.push(ex);
    }
}

fn tokenize_scheme(text: &str)-> Vec<&str> {
    let mut parsed = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| c == '(' || c == ')'){
        if last != index {
            parsed.push(&text[last..index]);
        }
        parsed.push(matched);
        last = index + matched.len();
    }
    parsed.into_iter().map(|x| x.split(' ').collect::<Vec::<&str>>()).flatten()
        .filter(|x| !x.trim().is_empty()).collect::<Vec::<&str>>()
}

fn build_tree<'a, I>(parsed: &mut I)-> ParseTree
where
    I: Iterator<Item = &'a str>,
{
    let mut pt = ParseTree{ list: Vec::<Expr>::new()};
    let mut m_ch = parsed.next();
    while let Some(ch) = m_ch {
        if ch == ")" { 
            break;
        }
        if ch == "(" {
            pt.add_expr(Tree(Box::new(build_tree(parsed))));
            m_ch = parsed.next();
            continue;
        }
        pt.add_expr(Expr::Text(String::from(ch)));
        m_ch = parsed.next();
    }
    pt
}

fn apply_func(mut vals: Vec<Val>, dict: &HashMap<String, Val>) -> Val{
    let func = vals.remove(0);
    match func {
        SupportedFunction(s) =>
            match s.as_str(){
                "+" =>  vals.into_iter().fold(Number(false, 0, 1), |x, y| x + y),
                "-" =>  {
                    let mut sum = vals[0].clone();
                    for val in vals.into_iter().skip(1){
                        sum = sum - val;
                    }
                    sum
                }
                "*" => vals.into_iter().fold(Number(false, 1, 1), |x, y| x * y),
                "/" => {
                    let mut quotient = vals[0].clone();
                    for val in vals.into_iter().skip(1){
                        if let Number(false, 0, 1) = val { return SchemeError(); }
                        quotient = quotient / val;
                    }
                    quotient
                }
                "number=?" => {
                    if vals.len() != 2 { return SchemeError(); }
                    if let Number(neg, n, d) = vals[0]{
                        if let Number(oneg, on, od) = vals[1]{
                            return Boolean(neg == oneg && n == on && d == od);
                        }
                    }
                    SchemeError()
                },
                _ => SchemeError(),
            }
        Function(bindings, exp) => {
            if bindings.len() != vals.len(){
                return SchemeError();
            }
            let mut new_dict = dict.clone();
            for it in bindings.iter().zip(vals.iter()){
                let (binding, val) = it;
                new_dict.insert(binding.to_string(), val.clone());
            }
            eval_scheme(&exp, &new_dict)
        }
        _ => SchemeError(),
    }
}

fn eval_scheme(ex: &Expr, dict: &HashMap<String,Val>) -> Val{
    match ex{
        Text(txt) => {
            if txt == "true" { Boolean(true) }
            else if txt == "false" { Boolean(false) }
            else if let Ok(n) = txt.parse::<i32>(){ Number(n < 0, n.abs().try_into().unwrap() , 1) }
            else if SUPPORTED_OPPERATIONS.contains(txt as &str) { 
                SupportedFunction(txt.to_string())
            }
            else if dict.contains_key(txt) {
                dict.get(txt).unwrap().clone()
            }
            else{ SchemeError() }
        },
        Tree(expr) => {
            let mut vals: Vec::<Val> = Vec::new();
            for exp in &expr.list {
                let next_res = eval_scheme(&exp, dict);
                if let SchemeError() = next_res {
                    if vals.len() > 0 {
                        if let SupportedFunction(s) = &vals[0]{
                            if s != "lambda"{
                                return SchemeError();
                            }
                        } else {
                            return SchemeError();
                        } 
                    }
                }
                if let SupportedFunction(s) = &next_res{
                    match s.as_str(){
                        // Special treatement of cond and if and lambda
                        "cond" => {
                            if expr.list.len() % 2 != 1 { return SchemeError(); }
                            let mut index = 1;
                            while index < expr.list.len() {
                                match eval_scheme(&expr.list[index], dict){
                                    Boolean(true) => { return eval_scheme(&expr.list[index + 1], dict); },
                                    Boolean(false) => (),
                                    _ => { return SchemeError() },
                                }
                                index += 2;
                            }
                            return SchemeError();
                        },
                        "if" => {
                            if expr.list.len() != 4 { return SchemeError(); }
                            match eval_scheme(&expr.list[1], dict){
                                Boolean(true) => { return eval_scheme(&expr.list[2], dict); },
                                Boolean(false) => { return eval_scheme(&expr.list[3], dict); },
                                _ => { return SchemeError() }
                            }
                        },
                        "lambda" => {
                            let mut bindings = Vec::new();
                            if let Tree(binding_list) = &expr.list[1] {
                                for binding in &*binding_list.list {
                                    if let Text(s) = binding {
                                        bindings.push(s.to_string());
                                    } else {
                                        return SchemeError();
                                    }
                                }
                                let mut expression = expr.list[2].clone();
                                for (key,val) in dict {
                                    if bindings.contains(key) { continue; }
                                    expression = expression.bind_val(key,val);
                                }
                                return Function(bindings, expression);
                            } else {
                                return SchemeError();
                            }
                        }
                        _ => (),
                    }
                }
                vals.push(next_res);
            }
            apply_func(vals, dict)
        }
        Bound(v) => *v.clone()
    }
}


fn add_definition(expr: &Expr, definitions: &mut HashMap<String, Val>) -> bool{
    if let Tree(tr) = expr {
        if let Text(s) = &tr.list[0]{
            if s == "define" {
                match &tr.list[1] {
                    Text(var) => {definitions.insert(var.to_string(), eval_scheme(&tr.list[2], &definitions));}
                    Tree(bindings) => {
                        if let Text(var) = &bindings.list[0]{
                            let mut bounded = Vec::new();
                            for bind in &bindings.list[1..] {
                                if let Text(s) = bind {
                                    bounded.push(s.to_string());
                                }
                                else {
                                    continue;
                                }
                            }
                            definitions.insert(var.to_string(), Function(bounded, tr.list[2].clone()));
                        }
                    }
                    Bound(_v) => () // Should not get here
                }
                return true;
            }
        }
    }
    return false;
}

pub fn run_scheme(text: String) -> String {
    let parsed = tokenize_scheme(&text);

    let tree = build_tree(&mut (parsed.into_iter()));

    let mut definitions = HashMap::new();
    let mut result_string = String::new();

    for expr in tree.list{
        if add_definition(&expr, &mut definitions) {continue}

        let result = eval_scheme(&expr, &definitions);

        match result {
            Number(neg, n, d) => {
                result_string += format!("{}{n}{}\n", if neg {"-"} else {""},
                    if d != 1 {"/".to_string() + &d.to_string()} else {"".to_string()}).as_str()
            },
            Boolean(b) => {
                result_string += format!("{}\n", b).as_str()
            },
            SchemeError() => result_string += "Error\n",
            _ => result_string += ""
        }
    }
    return result_string;
}

