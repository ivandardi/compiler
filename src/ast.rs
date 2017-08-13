#[derive(Debug, Clone)]
pub struct Function {
    pub identifier: Box<Expr>,
    pub params: Vec<(Box<Expr>, Type)>,
    pub ret: Type,
    pub declarations: Vec<VarDecl>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub identifier: Box<Expr>,
    pub ty: Type,
    pub expression: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Box<Expr>),
    Assignment(Box<Expr>, Box<Expr>),
    If(Comparison, Vec<Statement>, Option<Vec<Statement>>),
    While(Comparison, Vec<Statement>),
    Return(Option<Box<Expr>>),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub struct Comparison {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub operator: CmpOperator,
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

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Type {
    I64,
    Bool,
    Array(Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Operation(Box<Expr>, Opcode, Box<Expr>),
    Variable(Variable),
    FunctionCall {
        identifier: Box<Expr>,
        arg_list: Vec<Box<Expr>>,
    },
}

#[derive(Debug, Clone)]
pub enum Variable {
    Id(String),
    Vec {
        identifier: Box<Expr>,
        index: Box<Expr>,
    },
}
