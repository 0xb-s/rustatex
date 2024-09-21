use crate::parser::Rule;
use pest::error::Error as PestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustaTexError {
    #[error("Parsing error at {0}")]
    PestError(#[from] PestError<Rule>),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Rendering error: {0}")]
    RenderError(String),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
  
    #[allow(unused)]
    #[error("Other error: {0}")]
    Other(String),
}
