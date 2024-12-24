mod scheme;

use std::env;
use scheme::run_scheme;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arg_list = args.iter();
    _ = arg_list.next();
    for file in arg_list{
        if let Err(error) = run_scheme(file){
            println!("Error ({error}) running file: {file}");
        }
    }
}
