use crate::parse::Rule;
use thiserror::Error;

pub type DiroResult<T> = Result<T, DiroError>;

#[derive(Debug, Error)]
pub enum DiroError {
    #[error("Pest Error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("IntParseError: {0}")]
    IntParseError(#[from] std::num::ParseIntError),
    #[error("division by zero")]
    ZeroDivision,
    #[error("KQ number can't be bigger than the amount of dices")]
    KQTooBig,
    #[error("At least one dice must be present")]
    NoDice,
    #[error("Dice should roll before calulate")]
    DiceNotRolled,
}
