use crate::parse::Rule;
use thiserror::Error;

pub type DiroResult<T> = Result<T, DiroError>;

#[derive(Debug, Error)]
pub enum DiroError {
    #[error("Pest Error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Unknown Verb: {0}")]
    UnknownVerb(String),
    #[error("IntParseError: {0}")]
    IntParseError(#[from] std::num::ParseIntError),
}
