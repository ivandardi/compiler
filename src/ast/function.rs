use super::id::Id;
use super::types::Type;
use super::statement::Statement;

#[derive(Debug, Clone, Hash)]
pub struct Function {
    pub id: Id,
    pub params: Vec<(Id, Type)>,
    pub ret: Type,
    pub stmts: Vec<Statement>,
}
