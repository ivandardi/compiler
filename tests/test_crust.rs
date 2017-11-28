extern crate compiler;

use compiler::{compile, get_code};
use std::env;

fn test_code(filename: &str) {
    let code = get_code(filename).expect("Failed to get code");
    let results = compile(&code);
    if let Err(results) = results {
        eprintln!("Error: {}", results);
        panic!("Error on {}!", filename);
    }
}

/*
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
*/

#[test]
fn test_0() {
    test_code("tests/0.crust");
}

#[test]
#[should_panic]
fn test_1() {
    test_code("tests/1.crust");
}

#[test]
#[should_panic]
fn test_2() {
    test_code("tests/2.crust");
}

#[test]
#[should_panic]
fn test_3() {
    test_code("tests/3.crust");
}

#[test]
#[should_panic]
fn test_4() {
    test_code("tests/4.crust");
}
