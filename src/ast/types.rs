use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    I64,
    Bool,
    Void,
    Array(Box<Type>),
    Function, // TODO add proper function signature here
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Type::*;
        let x = match *self {
            I64 => "I64".to_string(),
            Bool => "Bool".to_string(),
            Void => "Void".to_string(),
            Array(ref types) => format!("[]{}", types),
            Function => "Function".to_string(),
        };
        write!(f, "{}", x)
    }
}
