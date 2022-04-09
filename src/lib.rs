mod dice;
mod error;
mod parse;
#[cfg(test)]
mod tests;

pub use dice::{Dice, RollResult};
pub use error::{DiroError, DiroResult};
pub use parse::{parse, DiroAst};
