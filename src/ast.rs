use std::fmt::{Debug, Formatter, Error};

pub enum Statement {
    Block,

}

pub enum Expr {
    Ident(String),
    Number(i64),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{}", n),
            Op(ref l, op, ref r) => write!(fmt, "{:?} {:?} {:?})", l, op, r),
            Ident(ref id) =>  write!(fmt, "{}", id),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FnDecl {
    identifier: String,
    statements: Vec<VarDecl>,
}

impl FnDecl {
    pub fn new(identifier: Box<Expr>, statements: Vec<VarDecl>) ->Self {
        use self::Expr::*;
        let identifier = match *identifier {
            Ident(id) => id.to_string(),
            _ => panic!("Invalid ID"),
        };
        FnDecl {
            identifier: identifier,
            statements,
        }
    }
}