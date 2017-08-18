#[derive(Debug, Clone)]
pub struct Function {
    pub id: Id,
    pub params: Vec<(Id, Type)>,
    pub ret: Type,
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    I64,
    Bool,
    Void,
    Array(Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Operation(Box<Expr>, BinOp, Box<Expr>),
    Variable(Variable),
    FunctionCall { id: Id, args: Vec<Box<Expr>> },
}

#[derive(Debug, Clone)]
pub enum Variable {
    Id(Id),
    Vec { id: Id, index: Box<Expr> },
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Id(pub String);

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}
