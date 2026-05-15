mod scheme;
mod scheme_tests;

use std::env;
use scheme::run_scheme_on_file;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg_list = args.iter();
    _ = arg_list.next();
    for file in arg_list{
        match run_scheme_on_file(file) {
            Ok(x) => println!("{}", x),
            Err(e) => println!("Error: {}", e)
        }
    }
}
