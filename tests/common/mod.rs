use compiler::{compile, get_code};

use std::env;

pub fn test_code(filename: &str) {
    let code = get_code(filename).expect("Failed to get code");
    let results = compile(&code);
    if env::var("PRINT-AST").is_ok() {
        println!("{:#?}", results);
    }
}
