use super::id::Id;
use super::types::Type;
use super::variable::Variable;
use super::expression::Expr;

#[derive(Debug, Clone, Hash)]
pub enum Statement {
    VarDecl {
        id: Id,
        ty: Type,
        expr: Option<Box<Expr>>,
    },
    Expression(Box<Expr>),
    Assignment(Variable, Box<Expr>),
    If(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>),
    Loop(Box<Expr>, Vec<Statement>),
    Return(Option<Box<Expr>>),
    Block(Vec<Statement>),
}
