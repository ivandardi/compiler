use std::fmt;
use super::id::Id;

/// In Crust, there's only 3 kinds of types:
/// * Functions: fn() -> type
/// * Arrays: [][]type
/// * Primitive types: void, i64, bool
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Function {
        params: Vec<(Id, Type)>,
        ret: Box<Type>,
    },
    Array(Box<Type>),
    Primitive(TypePrimitive),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TypePrimitive {
    Int,
    Bool,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Type::*;
        use self::TypePrimitive::*;
        write!(f, "{}", match *self {
            Function {ref params,ref ret} => {
//                let params = params.join(", ");
//                format!("fn({}) -> {}", params, ret)
                ":)".to_string()
            },
            Array(ref types) => format!("[]{}", types),
            Primitive(ty) => match ty {
                Int => "Int".to_string(),
                Bool => "Bool".to_string(),
                Void => "Void".to_string(),
            },
        })
    }
}
