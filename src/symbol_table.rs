use std::collections::hash_map::{HashMap, Entry};
use std::fmt;
use ast::{Ast, Id, Statement, Type, Variable};
use errors::{CompilerError, SemanticError};

// Name(ID)   Type           Scope     Time of Discovery
// foo        fn() -> I64    global
// bar        Bool           foo

#[derive(Debug)]
struct Symbol<'a> {
    pub ty: &'a Type,
}

impl<'a> Symbol<'a> {
    pub fn new(ty: &'a Type) -> Self {
        Symbol { ty }
    }
}

/// Hashmap stores the name of the symbol.
/// The vector stores the scope and ocurrences of that symbol
#[derive(Debug)]
pub struct SymbolTable<'a>(HashMap<Vec<&'a str>, Symbol<'a>>);

impl<'a> SymbolTable<'a> {
    /// When building the symbol table, if we encounter a repeated variable,
    /// we'll add that to a vector of errors and don't insert the repeated
    /// variable into the table. That will make it generate more errors
    /// later on.
    ///
    /// When building the symbol table, we keep a stack of scopes. That's
    /// the key to search for in the symbol table. Expression blocks inside
    /// functions are anonymous scopes and therefore are referred by as
    /// numbers. So for example the code
    /// ```
    /// fn foo() {
    ///     let bar: i64 = 5;
    ///     {
    ///         let baz: i64 = 10;
    ///     }
    ///     {
    ///         let baz: i64 = 20;
    ///     }
    /// }
    /// ```
    /// will yield the following table:
    /// ```
    /// Scope & Name  | Type
    /// [foo]         | fn() -> void
    /// [foo, bar]    | i64
    /// [foo, 0, baz] | i64
    /// [foo, 1, baz] | i64
    /// ```
    ///
    pub fn new(ast: Ast) -> Self {
        let mut hashmap = HashMap::new();
        /*
        let mut symbols = vec![];
        let mut errors = vec![];
        for fun in ast {
            let fun = match hashmap.entry(&fun.id.0) {
                Entry::Occupied(o) => {
                    vec.add(SemanticError::DuplicateFunction(format!("Duplicate function {}!", o.key())));
                    continue;
                }
                Entry::Vacant(v) => {
                    v.insert(Symbol::new(&Type::Function))
                }
            };
            for &(Id(ref id), ref ty) in &fun.params {
                symbols.push(&id);
                hashmap.insert(symbols.clone(), Symbol::new(ty));
                symbols.pop();
            }
            for stmt in &fun.stmts {
                match *stmt {
                    Statement::VarDecl {id: Id(ref id), ref ty, ..} => {
                        symbols.push(&id);
                        hashmap.insert(symbols.clone(), Symbol::new(ty));
                        symbols.pop();
                    }
                    _ => {}
                }
            }
        }
        */
        (ast, SymbolTable(hashmap))
    }
}

impl<'a> fmt::Display for SymbolTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let padding = 20;
        writeln!(f, "{:<width$}|{:<width$}", "Scope & Name", "Type",
                 width = padding)?;
        writeln!(f, "{:-^width$}", "+", width = 2 * padding + 1)?;
        for (name, symbol) in self.0.iter() {
            let name = name.join(".");
            let ty = symbol.ty.to_string();
            writeln!(f, "{:<width$}|{:<width$}", name, ty, width = padding)?;
        }
        Ok(())
    }
}
