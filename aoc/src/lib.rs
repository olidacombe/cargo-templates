pub fn {{main-function}}(mut it: impl Iterator<Item = String>) -> {{main-function-return-type}} {
    {{main-function-return-type}}::default()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
            !!!CHANGE_ME!!!
        "};
        assert_eq!(
            {{main-function}}(example.lines().map(String::from)),
            {{expected}}
        )
    }
}
