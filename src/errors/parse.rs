use crate::util::span::Span;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {

    #[error("unexpected token at {0:?}")]
    UnexpectedToken(Span),

    #[error("Incomplete input at {0:?}")]
    Incomplete(Span),
    
    #[error("generic parse error: {0}")]
    Generic(String),
}