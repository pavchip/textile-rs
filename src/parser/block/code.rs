use parser::Block;
use regex::Regex;

pub fn parse_code(text: &str) -> Option<Block> {
    let pattern = Regex::new(r"bc\. (?P<text>(?:.|[\r\n])*)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();

        Some(Block::Code(content.to_string()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Block;
    use super::*;

    #[test]
    fn parsers_code_correctly() {
        assert_eq!(
            parse_code("bc. print('Hello World')\nprint(10 * 4)"),
            Some(Block::Code("print('Hello World')\nprint(10 * 4)".to_string()))
        );
    }
}
