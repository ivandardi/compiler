use ast;

pub fn semantic_check(ast: &[ast::Function]) -> Result<(), String> {
    check_main_function(ast)?;
    check_void_variables(ast)?;
    Ok(())
}

fn check_main_function(ast: &[ast::Function]) -> Result<(), String> {
    let main_fn = ast
        .iter()
        .find(|ref fun| fun.id.0 == "main")
        .ok_or("Main function not found!")?;

    if main_fn.params != vec![] {
        return Err(String::from("Main function can't take parameters!"));
    }
    if main_fn.ret != ast::Type::Void {
        return Err(String::from("Main function is not returning a void type!"));
    }
    Ok(())
}

fn check_void_variables(ast: &[ast::Function]) -> Result<(), String> {
    for fun in ast {
        for stmt in &fun.stmts {
            if let &ast::Statement::VarDecl {ref ty, ..} = stmt {
                if *ty == ast::Type::Void {
                    return Err(String::from("Can't declare a variable with a void type!"));
                }
            }
        }
    }
    Ok(())
}
