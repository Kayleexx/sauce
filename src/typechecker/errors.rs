use crate::util::span::Span;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TypeError {
    #[error("unknown identifier `{1}` at {0:?}")]
    UnknownIdent(Span, String),

    #[error("type mismatch at {0:?}: expected {1}, found {2}")]
    Mismatch(Span, String, String),

     #[error("invalid pipeline at {0:?}: right side must be callable")]
    InvalidPipeline(Span),

    #[error("generic type error: {0}")]
    Generic(String),
}