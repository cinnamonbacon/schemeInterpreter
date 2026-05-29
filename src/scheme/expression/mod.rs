use crate::scheme::Val;
use crate::scheme::Val::Function;
use crate::scheme::next_function_id;
use std::sync::Arc;
use std::hash::Hash;

#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq,PartialEq)]
pub struct ParseTree{
    pub list: Vec<Expr>,
}

impl ParseTree{
    pub fn add_expr(&mut self, ex: Expr){
        self.list.push(ex);
    }
}

#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq,PartialEq)]
pub enum Expr{
    Text(Arc<String>),
    Bound(Box<Val>),
    Tree(Box<ParseTree>),
}

use Expr::*;

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

impl Expr {
    pub fn bind_val(self, replace: &String, v: &Val) -> Expr {
        match self {
            Text(s) => if **s == *replace { Bound(Box::new(v.clone())) } else{ Text(s) },
            Bound(b) => {
                if let Function(bindings, tree, n) = *b{
                    if let Some(_) = bindings.iter().position(|s| s == replace) {
                        Bound(Box::new(Function(bindings, tree.clone(), n)))
                    }
                    else{
                        let mut new_tree = ParseTree{ list: Vec::new() };
                        for expr in tree.list.clone() {
                            new_tree.add_expr(expr.bind_val(replace, v));
                        }
                        Bound(Box::new(Function(bindings, new_tree.into(), next_function_id())))
                    }
                }
                else{
                    Bound(b)
                }
            }
            Tree(pt) => {
                let mut ret = ParseTree{ list: Vec::new() };
                if let Text(s) = &pt.list[0] {
                    if **s == "lambda" {
                        if let Tree(bindings) = &pt.list[1] {
                            for b in bindings.list.clone(){
                                if let Text(binding_name) = b{
                                    if **binding_name == *replace {
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


