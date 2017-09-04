#[derive(Debug, Error)]
pub enum CompilerError {
    /// Holds lexical errors
    #[error(non_std, no_from)]
    Lexical,
    /// Holds syntax errors
    #[error(non_std, no_from)]
    Syntax,
    /// Holds semantic errors
    #[error(non_std)]
    Semantic(SemanticError),
}

#[derive(Debug, Error)]
pub enum SemanticError {
    #[error(msg_embedded, non_std, no_from)]
    NoMainFunction(String),
    #[error(msg_embedded, non_std, no_from)]
    MainTakingParameters(String),
    #[error(msg_embedded, non_std, no_from)]
    MainNotReturningVoid(String),
    #[error(msg_embedded, non_std, no_from)]
    VoidVariable(String),
    #[error(msg_embedded, non_std, no_from)]
    DuplicateFunction(String),
}


