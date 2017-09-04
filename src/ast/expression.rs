use super::binary_operator::BinOp;
use super::variable::Variable;
use super::id::Id;

#[derive(Debug, Clone, Hash)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Operation(Box<Expr>, BinOp, Box<Expr>),
    Variable(Variable),
    FunctionCall { id: Id, args: Vec<Expr> },
}
