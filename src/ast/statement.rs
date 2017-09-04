use super::declaration::Declaration;
use super::variable::Variable;
use super::expression::Expr;

#[derive(Debug, Clone, Hash)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expr),
    Assignment(Variable, Expr),
    If(Expr, Vec<Statement>, Option<Vec<Statement>>),
    Loop(Expr, Vec<Statement>),
    Return(Option<Expr>),
    Block(Vec<Statement>),
}
