use common::parse::{self, Parse};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] parse::Error),
}

pub fn {{main-function}}(mut it: impl Iterator<Item = String>) -> Result<{{main-function-return-type}}> {
    Ok({{main-function-return-type}}::default())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() -> Result<()> {
        let example = indoc! {"
            !!!CHANGE_ME!!!
        "};
        assert_eq!(
            {{main-function}}(example.lines().map(String::from))?,
            {{expected-value}}
        );
        Ok(())
    }
}
