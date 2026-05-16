mod scheme;
mod scheme_tests;

use std::env;
use scheme::run_scheme;
use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg_list = args.iter();
    _ = arg_list.next();
    if arg_list.len() != 0 {
        for file in arg_list{
            match run_scheme_on_file(file) {
                Ok(x) => println!("{}", x),
                Err(e) => println!("Error: {}", e)
            }
        }
    }
    else {
        let mut input = String::from("");
        let _ = io::stdin().read_to_string(&mut input);
        println!("{}", run_scheme(input.clone()));
    }
}

pub fn run_scheme_on_file(s: &str)-> Result<String, std::io::Error> {
    let mut text = String::new();

    File::open(s)?.read_to_string(&mut text)?;
    Ok(run_scheme(text))
}
