mod scheme_tests;
mod value;
mod expression;

use std::collections::HashMap;

use value::Val;
use value::Val::*;
use value::SUPPORTED_OPPERATIONS;

use expression::ParseTree;
use expression::Expr;
use expression::Expr::*;

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
        pt.add_expr(Expr::Text(String::from(ch).into()));
        m_ch = parsed.next();
    }
    pt
}

fn apply_func(mut vals: Vec<Val>, dict: &HashMap<String, Val>) -> Val{
    let func = vals.remove(0);
    match func {
        SupportedFunction(s) =>
            match &**s{
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
                "cons" => {
                    if vals.len() != 2 { return SchemeError(); }
                    Pair(vals[0].clone().into(), vals[1].clone().into())
                }
                "car" => {
                    if vals.len() != 1 { return SchemeError(); }
                    if let Pair(x,_y) = &vals[0] {
                        (**x).clone()
                    }
                    else { SchemeError() }
                }
                "cdr" => {
                    if vals.len() != 1 { return SchemeError(); }
                    if let Pair(_x,y) = &vals[0] {
                        (**y).clone()
                    }
                    else { SchemeError() }
                }
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
            if **txt == "true" { Boolean(true) }
            else if **txt == "false" { Boolean(false) }
            else if let Ok(n) = (**txt).parse::<i32>(){ Number(n < 0, n.abs().try_into().unwrap() , 1) }
            else if SUPPORTED_OPPERATIONS.contains(&*txt as &str) {
                SupportedFunction(txt.clone())
            }
            else if dict.contains_key(&**txt) {
                dict.get(&**txt).unwrap().clone()
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
                            if **s != "lambda"{
                                return SchemeError();
                            }
                        } else {
                            return SchemeError();
                        }
                    }
                }
                if let SupportedFunction(s) = &next_res{
                    match (**s).as_str(){
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
            if **s == "define" {
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

        result_string += &result.to_string();
        result_string += "\n";
    }
    return result_string;
}

