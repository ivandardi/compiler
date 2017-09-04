use super::id::Id;
use super::types::Type;
use super::statement::Statement;
use super::expression::Expr;

#[derive(Debug, Clone, Hash)]
pub struct Declaration {
    /// Identifier of the new variable/function
    pub id: Id,
    /// Type of the new variable/function
    pub ty: Type,
    /// Associated code if function, optional initialization if variable
    pub data: DeclarationData,
}

#[derive(Debug, Clone, Hash)]
pub enum DeclarationData {
    Function(Vec<Statement>),
    Variable(Option<Expr>),
}
