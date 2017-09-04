//#![feature(conservative_impl_trait)]
#![allow(unknown_lints)]
#![deny(missing_debug_implementations)]

extern crate lalrpop_util;
#[macro_use]
extern crate derive_error;

#[allow(clippy)]
mod lang;
mod errors;
mod ast;
mod semantic;
mod symbol_table;

use symbol_table::SymbolTable;
use std::fs::File;
use std::env;
use std::io;
use std::io::prelude::*;

pub fn get_code(filename: &str) -> io::Result<String> {
    let mut code = String::new();
    File::open(filename).and_then(|mut f| f.read_to_string(&mut code))?;
    Ok(code)
}

pub fn compile(code: &str) -> Result<ast::Ast, String> {
    let ast = lang::parse_file(code).map_err(|e| format!("{:?}", e))?;
//    let (ast, symbol_table) = SymbolTable::new(ast);
//    let results = semantic::check(&ast, &symbol_table)?;

    if let Err(_) = env::var("NO_AST") {
        println!("{:#?}", ast);
    }

    if let Err(_) = env::var("NO_SEM") {
        println!("{}", symbol_table);
    }

    Ok(ast)
}
