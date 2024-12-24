use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum Expr{
    Text(String),
    SubExpr(Box<ParseTree>),
}

#[derive(Debug)]
struct ParseTree{
    list: Vec<Expr>,
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
            pt.add_expr(Expr::SubExpr(Box::new(build_tree(parsed))));
            m_ch = parsed.next();
            continue;
        }
        pt.add_expr(Expr::Text(String::from(ch)));
        m_ch = parsed.next();
    }
    pt
}



pub fn run_scheme(s: &str)-> Result<String, io::Error> {
    let mut text = String::new();

    File::open(s)?.read_to_string(&mut text)?;
    let parsed = tokenize_scheme(&text);

    let tree = build_tree(&mut (parsed.into_iter()));
    println!("{tree:?}");


    Ok(text)
}

