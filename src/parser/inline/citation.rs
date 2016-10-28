use parser::inline::parse_inline_elements;
use parser::Inline;
use regex::Regex;

pub fn parse_citation(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^\?\?(?P<text>.+?)\?\?").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();

        Some((Inline::Citation(parse_inline_elements(content)), content.len() + 4))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Inline;
    use super::*;

    #[test]
    fn parses_citation_correctly() {
        assert_eq!(
            parse_citation("??Textile citation element??"),
            Some((Inline::Citation(
                vec![
                    Inline::Text("Textile citation element".to_string())
                ]
            ), 28))
        );
    }
}
