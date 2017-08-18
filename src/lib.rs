#![feature(conservative_impl_trait)]
#![allow(unknown_lints)]
#![deny(missing_debug_implementations)]

extern crate lalrpop_util;

#[allow(clippy)]
mod lang;
mod ast;
mod semantic;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn get_code(filename: &str) -> io::Result<String> {
    let mut code = String::new();
    File::open(filename)
        .and_then(|mut f| f.read_to_string(&mut code))?;
    Ok(code)
}

pub fn compile(code: &str) -> Result<Vec<ast::Function>, String> {
    let results = lang::parse_file(code).map_err(|e| format!("{:?}", e))?;
    semantic::semantic_check(&results)?;
    Ok(results)
}

// TODO symbol table
// Name(ID)   Type(IC)   Type(Data)    Scope     Time of Discovery
// foo        fn         I64           global
// bar        var        Bool          foo
