extern crate compiler;

use compiler::{compile, get_code};

use std::env;

fn main() {
    let filename = env::args().nth(1).expect("Expected file to compile");

    println!("Compiling {:?}", filename);

    let code = get_code(&filename).expect("Failed to get code");
    let results = compile(&code);

    if let Err(results) = results {
        panic!("Error: {}", results);
    }
}
