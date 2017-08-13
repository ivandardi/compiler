use std::fmt::{Debug, Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub struct VarDecl {
    identifier: String,
    expression: Box<Expr>,
}

impl VarDecl {
    pub fn new(identifier: Box<Expr>, expression: Box<Expr>) ->Self {
        use self::Expr::*;
        let identifier = match *identifier {
            Ident(id) => id.to_string(),
            _ => panic!("Invalid ID"),
        };
        VarDecl {
            identifier: identifier,
            expression,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FnDecl {
    identifier: String,
    statements: Vec<VarDecl>,
}

impl FnDecl {
    pub fn new(identifier: Box<Expr>, statements: Vec<VarDecl>) -> Self {
        use self::Expr::*;
        let identifier = match *identifier {
            Ident(id) => id.to_string(),
            _ => panic!("Invalid ID"),
        };
        FnDecl {
            identifier,
            statements,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CmpOperator {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Debug for CmpOperator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CmpOperator::*;
        write!(fmt, match *self {
            Eq => "==",
            Ne => "!=",
            Lt => "<",
            Gt => ">",
            Le => "<=",
            Ge => ">=",
        });
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        write!(fmt, match *self {
            Mul => "*",
            Div => "/",
            Add => "+",
            Sub => "-",
        });
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    I64,
    Array(Box(Type)),
}

impl Debug for Type {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Type::*;
        match *self {
            I64 => write!(fmt, "i64"),
            Array(ty) => write!(fmt, "[]{:?}", ty),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Number(i64),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{}", n),
            Ident(ref id) =>  write!(fmt, "{}", id),
        }
    }
}

#[derive(Debug, Copy, Clone)]