use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("unknown variable `{0}`")]
    UnknownVariable(String),

    #[error("effect `{0}` was not handled")]
    UnhandledEffect(String),
}
