extern crate lalrpop_util;

pub mod ast;
pub mod lang;

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let code = get_code("main.crust").expect("Failed to get code");
    let results : () = lang::parse_file(&code);
//    print_results(&results);
    println!("Results: {:?}", results);
}

fn get_code(filename: &str) -> io::Result<String> {
    let mut code = String::new();
    File::open(filename).and_then(|mut f| f.read_to_string(&mut code))?;
    Ok(code)
}

//fn print_results(results: &[lang::])