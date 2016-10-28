use parser::inline::parse_inline_elements;
use parser::Block;
use regex::Regex;

pub fn parse_header(text: &str) -> Option<Block> {
    let pattern = Regex::new(r"h(?P<level>[1-6])\. (?P<text>(?:.|[\r\n])*)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();
        let level = caps.name("level").unwrap().parse::<u8>().unwrap();

        Some(Block::Header {
            elements: parse_inline_elements(content),
            level: level
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline, BoldTagType, ItalicTagType};
    use super::*;

    #[test]
    fn parsers_header_correctly() {
        assert_eq!(
            parse_header("h2. *Bold text* _Italic text_"),
            Some(Block::Header {
                elements: vec![
                    Inline::Bold(
                        vec![
                            Inline::Text("Bold text".to_string())
                        ],
                        BoldTagType::Strong
                    ),
                    Inline::Text(" ".to_string()),
                    Inline::Italic(
                        vec![
                            Inline::Text("Italic text".to_string())
                        ],
                        ItalicTagType::Emphasis
                    )
                ],
                level: 2
            })
        );
    }
}
