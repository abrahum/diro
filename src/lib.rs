mod dice;
mod error;
mod parse;

pub use dice::{Dice, RollResult};
pub use error::{DiroError, DiroResult};
pub use parse::{parse, DiroAst};
