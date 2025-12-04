use std::string::String;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("stack is empty and no more elements can be retrieved")]
    EmptyStack,
    #[error("cannot divide by zero")]
    DivisionByZero,
    #[error("io error")]
    IOError,
    #[error("expected a word in definition")]
    ExpectedWord,
    #[error("word has already been defined")]
    WordAlreadyDefined,
    #[error("unknown token: {0}")]
    UnknownToken(String),
    #[error("can't retrieve index because we are not in a loop")]
    NotInLoop,
}
