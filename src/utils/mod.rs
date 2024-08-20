use std::collections::HashSet;
use regex::Regex;

pub mod image;

pub fn escape_single_quotes(input: &str) -> String {
    let re = Regex::new(r"'").unwrap();
    re.replace_all(input, "\\'").to_string()
}

// 字符串数组去重
pub fn deduplicate(vec: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();

    for item in vec {
        if seen.insert(item.clone()) {
            deduped.push(item);
        }
    }
    deduped
}

#[cfg(test)]
mod test {
    #[test]
    fn test_escape_single_quotes() {
        assert_eq!(super::escape_single_quotes("I'm"), "I\\'m");
    }
}
