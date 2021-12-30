mod parse;
mod error;
mod dice;

pub use parse::parse;
pub use dice::Dice;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
