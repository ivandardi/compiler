extern crate compiler;

use compiler::{compile, get_code};
use std::env;

fn test_code(filename: &str) {
    let code = get_code(filename).expect("Failed to get code");
    let results = compile(&code);
    if let Err(results) = results {
        if let Ok(_) = env::var("PRINT_ERR") {
            eprintln!("Error: {}", results);
        }
        panic!("Error on {}!", filename);
    }
}

#[test]
fn test_void_function() {
    test_code("tests/crust_void_function.crust");
}

#[test]
#[should_panic]
fn test_void_variable() {
    test_code("tests/crust_void_variable.crust");
}

#[test]
fn test_simple_function() {
    test_code("tests/crust_simple_function.crust");
}

#[test]
#[should_panic]
fn test_mainless() {
    test_code("tests/crust_mainless.crust");
}

#[test]
fn test_matrix() {
    test_code("tests/crust_matrix.crust");
}

#[test]
#[ignore]
fn test_quicksort() {
    test_code("tests/crust_quicksort.crust");
}
