use ast;
use symbol_table::SymbolTable;
use errors::{CompilerError, SemanticError};

pub fn check(ast: &ast::Ast, symbols: &SymbolTable) -> Result<(), CompilerError> {
    // TODO multithread this
    check_main_function(ast)?;
    check_void_variables(ast)?;
    Ok(())
}

fn check_main_function(ast: &ast::Ast) -> Result<(), SemanticError> {
    let main_fn = ast
        .iter()
        .find(|fun| fun.id.0 == "main")
        .ok_or(SemanticError::NoMainFunction("Main function not found!".to_string()))?;
/*
    if main_fn.params != vec![] {
        return Err(SemanticError::MainTakingParameters("Main function can't take parameters!".to_string()));
    }
    if main_fn.ret != ast::Type::Void {
        return Err(SemanticError::MainNotReturningVoid("Main function is not returning a void type!".to_string()));
    }
*/
    Ok(())
}

fn check_void_variables(ast: &ast::Ast) -> Result<(), SemanticError> {
    for fun in ast {
        /*
        for stmt in &fun.stmts {
            if let ast::Statement::VarDecl {ref ty, ..} = *stmt {
                if *ty == ast::Type::Void {
                    return SemanticError::VoidVariable("Can't declare a variable with a void type!".to_string());
                }
            }
        }
        */
    }
    Ok(())
}

fn check_same_name(ast: &ast::Ast, symbols: &SymbolTable) -> Result<(), SemanticError> {
    Ok(())
}
