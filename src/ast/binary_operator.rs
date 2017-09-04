#[derive(Debug, Copy, Clone, Hash)]
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
