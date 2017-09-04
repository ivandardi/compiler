use super::id::Id;
use super::expression::Expr;


#[derive(Debug, Clone, Hash)]
pub enum Variable {
    Id(Id),
    Vec { id: Id, index: Box<Expr> },
}
