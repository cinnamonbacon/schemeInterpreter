use std::fs::File;
use std::io::{self, Read}; use std::ops;
use std::collections::HashMap;

use gcd::Gcd;

#[derive(Debug)]
enum Val{
    Number(bool, u32, u32),
    Boolean(bool),
    //Function(ParseTree),
    Name(String),
    SchemeError(),
}
use Val::Number;
use Val::Boolean;
//use Val::Function;
use Val::Name;
use Val::SchemeError;

#[derive(Debug)]
enum Expr{
    Text(String),
    Tree(Box<ParseTree>),
}
use Expr::Text;
use Expr::Tree;

impl Clone for Expr{
    fn clone(&self) -> Self {
        match self{
            Text(s)=> Text(s.clone()),
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
    fn bind_val(self, replace: &String, exp: &Expr) -> Expr{
        match self{
            Text(s) => if s == *replace { exp.clone() } else{ Text(s) },
            Tree(pt) => {
                let mut ret = ParseTree{ list: Vec::new() };
                for lexp in pt.list{
                    ret.list.push(lexp.bind_val(replace, exp));
                }
                Tree(Box::new(ret))
            }
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
                let new_denom = d * od / d.gcd(od);
                let mut n = n * new_denom / od;
                let on = on * new_denom / d;
                let mut neg = neg;

                if n > on {
                    n = if neg{n - on} else {on + n};
                }
                else {
                    neg = oneg;
                    n = if neg{on - n} else {on + n};
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

impl ParseTree{
    fn add_expr(&mut self, ex: Expr){
        self.list.push(ex);
    }
}

fn tokenize_scheme(text: &str)-> Vec<&str> {
    let mut parsed = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| (c == '(' || c == ')')){
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

fn apply_func(func: Val, mut val: Vec<Val>) -> Val{
    match func{
        Name(s) => {
            match s.as_str(){
                "+" => {
                    val.into_iter().fold(Number(false, 0, 1), |x, y| x + y)
                },
                "*" => val.into_iter().fold(Number(false, 1, 1), |x, y| x * y),
                "number=?" => {
                    if val.len() != 2 {return SchemeError();}
                    if let Number(neg, n, d) = val[0]{
                        if let Number(oneg, on, od) = val[1]{
                            return Boolean(neg == oneg && n == on && d == od);
                        }
                    }
                    SchemeError()
                },
                "cond" => {
                    if val.len() % 2 != 0 { return SchemeError(); }
                    let mut index = 0;
                    while index < val.len() {
                        if let Boolean(b) = val[index] { if b { return val.remove(index + 1); }}
                        index += 2;
                    }
                    SchemeError()
                },
                "if" => {
                    if val.len() != 3 { return SchemeError(); }
                    if let Boolean(b) = val[0] { if b { return val.remove(1); } else { return val.remove(2); }}
                    SchemeError()
                },
                _ => SchemeError(),
            }
        }
        _ => SchemeError(),
    }
}

fn eval_scheme(ex: &Expr) -> Val{
    match ex{
        Text(txt) => if let Ok(n) = txt.parse(){ Number(false, n , 1) } else{Name(String::from(txt))},
        Tree(expr) => {
            let mut vals: Vec::<Val> = Vec::new();
            let func = eval_scheme(&expr.list[0]);
            let mut first = false;
            for exp in &expr.list {
                if !first {
                    first = true;
                    continue;
                }
                vals.push(eval_scheme(&exp))
            }
            apply_func(func, vals)
        }
    }
}


pub fn run_scheme(s: &str)-> Result<String, io::Error> {
    let mut text = String::new();

    File::open(s)?.read_to_string(&mut text)?;
    let parsed = tokenize_scheme(&text);

    let tree = build_tree(&mut (parsed.into_iter()));
    //println!("{tree:?}");

    let mut definitions = HashMap::new();

    for mut expr in tree.list{
        if let Tree(tr) = expr.clone(){
            if let Text(s) = &tr.list[0]{
                if s == "define" {
                    match tr.list[1].clone() {
                        Text(var) => {definitions.insert(var, tr.list[2].clone()); ()}
                        _ => () // TODO: functions
                    }
                    continue;
                }
            }
        }

        for (replaced, replacement) in &definitions{
            expr = expr.bind_val(&replaced, &replacement);
        }
        let result = eval_scheme(&expr);
        match result{
            Number(neg, n, d) => {
                if neg {
                    print!("-")
                }
                print!("{n}");
                if d != 1 {
                    print!("/{d}");
                }
                println!("");
                ()
            },
            Boolean(b) => {
                println!("{b}");
                ()
            }
            SchemeError() => {
                println!("Error");
                ()
            }
            _ => ()
        }
    }


    Ok(text)
}

