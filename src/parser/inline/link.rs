use parser::inline::parse_inline_elements;
use parser::Inline;
use regex::Regex;

pub fn parse_link(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new("^\"(?P<description>.*?)\":(?P<url>[^ \\(\\)]+)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let description = caps.name("description").unwrap();
        let url = caps.name("url").unwrap();

        Some((Inline::Link {
            description: parse_inline_elements(description),
            url: url.to_string()
        }, description.len() + url.len() + 3))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Inline, ItalicTagType};
    use super::*;

    #[test]
    fn parses_italic_text_correctly() {
        assert_eq!(
            parse_link("\"_Text_\":http://example.com"),
            Some((Inline::Link {
                description: vec![
                    Inline::Italic(
                        vec![
                            Inline::Text("Text".to_string())
                        ],
                        ItalicTagType::Emphasis
                    )
                ],
                url: "http://example.com".to_string()
            }, 27))
        );
    }
}
