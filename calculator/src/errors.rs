use std::{error, fmt::Display};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct ParserError {
    pub position: usize,
    pub message: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for ParserError {}

#[derive(Debug)]
pub struct LexerError {
    pub position: usize,
    pub message: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for LexerError {}

#[derive(Debug)]
pub(crate) struct SemanticError {
    pub message: String,
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for SemanticError {}

#[derive(Debug)]
pub(crate) struct EvaluationError {
    pub message: String,
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for EvaluationError {}
