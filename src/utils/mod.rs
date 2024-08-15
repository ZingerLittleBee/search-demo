use regex::Regex;

pub mod image;

pub fn escape_single_quotes(input: &str) -> String {
    let re = Regex::new(r"'").unwrap();
    re.replace_all(input, "\\'").to_string()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_escape_single_quotes() {
        assert_eq!(super::escape_single_quotes("I'm"), "I\\'m");
    }
}
