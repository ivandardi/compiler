extern crate compiler;

mod common;

#[test]
fn test_simple_function() {
    common::test_code("tests/simple_function.crust");
}
