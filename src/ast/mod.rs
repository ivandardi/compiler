mod binary_operator;
mod expression;
mod function;
mod id;
mod statement;
mod types;
mod variable;

pub use self::binary_operator::BinOp;
pub use self::expression::Expr;
pub use self::function::Function;
pub use self::id::Id;
pub use self::statement::Statement;
pub use self::types::Type;
pub use self::variable::Variable;

#[derive(Debug)]
pub struct Ast(pub Vec<Function>);

impl Ast {
    pub fn new(vec: Vec<Function>) -> Self {
        Ast(vec)
    }
}

impl ::std::ops::Deref for Ast {
    type Target = [Function];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl ::std::ops::DerefMut for Ast {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl IntoIterator for Ast {
    type Item = Function;
    type IntoIter = ::std::vec::IntoIter<Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Ast {
    type Item = &'a Function;
    type IntoIter = ::std::slice::Iter<'a, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Ast {
    type Item = &'a mut Function;
    type IntoIter = ::std::slice::IterMut<'a, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
