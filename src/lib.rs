#![deny(missing_debug_implementations)]

extern crate lalrpop_util;

pub mod ast;
mod lang;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn get_code(filename: &str) -> io::Result<String> {
    let mut code = String::new();
    File::open(filename).and_then(|mut f| f.read_to_string(&mut code))?;
    Ok(code)
}

pub fn compile(code: &str) -> Vec<ast::Function> {
    lang::parse_file(code).expect("Error!")
}