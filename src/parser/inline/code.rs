use parser::Inline;
use regex::Regex;

pub fn parse_code(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new("^@(?P<text>.*?)@").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();

        Some((Inline::Code(content.to_string()), content.len() + 2))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Inline;
    use super::*;

    #[test]
    fn parses_code_correctly() {
        assert_eq!(
            parse_code("@print('Hello World')@"),
            Some((Inline::Code("print('Hello World')".to_string()), 22))
        );
    }
}
