mod binary_operator;
mod declaration;
mod expression;
mod id;
mod statement;
mod types;
mod variable;

pub use self::binary_operator::BinOp;
pub use self::declaration::{Declaration, DeclarationData};
pub use self::expression::Expr;
pub use self::id::Id;
pub use self::statement::Statement;
pub use self::types::{Type, TypePrimitive};
pub use self::variable::Variable;

#[derive(Debug)]
pub struct Ast(pub Vec<Declaration>);

impl Ast {
    pub fn new(vec: Vec<Declaration>) -> Self {
        Ast(vec)
    }
}

impl ::std::ops::Deref for Ast {
    type Target = [Declaration];

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
    type Item = Declaration;
    type IntoIter = ::std::vec::IntoIter<Declaration>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Ast {
    type Item = &'a Declaration;
    type IntoIter = ::std::slice::Iter<'a, Declaration>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Ast {
    type Item = &'a mut Declaration;
    type IntoIter = ::std::slice::IterMut<'a, Declaration>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
