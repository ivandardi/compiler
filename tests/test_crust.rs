extern crate compiler;

use compiler::{compile, get_code};
use std::env;

fn test_code(filename: &str) {
    let code = get_code(filename).expect("Failed to get code");
    let results = compile(&code);

    if let Err(results) = results {
        panic!("Error: {}", results);
    }

    if env::var("PRINT_AST").is_ok() {
        println!("{:#?}", results);
    }
}

#[test]
fn test_void_function() {
    test_code("tests/crust_void_function.crust");
}

#[test]
fn test_void_variable() {
    test_code("tests/crust_void_variable.crust");
}

#[test]
#[should_panic]
fn test_simple_function() {
    test_code("tests/crust_simple_function.crust");
}

#[test]
#[should_panic]
fn test_mainless() {
    test_code("tests/crust_mainless.crust");
}

#[test]
#[ignore]
fn test_quicksort() {
    test_code("tests/crust_quicksort.crust");
}
