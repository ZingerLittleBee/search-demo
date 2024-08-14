use regex::Regex;
use tracing::info;

pub mod image;

pub fn replace_single_quotes(input: &str) -> String {
    let re = Regex::new(r"'").unwrap();
    let res = re.replace_all(input, "\'").to_string();

    info!("origin: {input}, replaced: {res}");

    res
}

#[cfg(test)]
mod test {
    #[test]
    fn test_replace_single_quotes() {
        assert_eq!(super::replace_single_quotes("I'm"), "I\'m");
    }
}
