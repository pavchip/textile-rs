use parser::inline::parse_inline_elements;
use parser::Block;
use regex::Regex;
use std::collections::HashMap;

pub fn parse_header(text: &str) -> Option<Block> {
    let pattern = Regex::new(r"h(?P<level>[1-6])(?P<text_align>[>=]?)\. (?P<text>(?:.|[\r\n])*)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();
        let level = caps.name("level").unwrap().parse::<u8>().unwrap();
        let text_align = match caps.name("text_align").unwrap() {
            "=" => "center",
            ">" => "right",
            _ => "",
        }.to_string();
        let mut attrs = HashMap::<String, String>::new();

        if !text_align.is_empty() {
            attrs.insert("text-align".to_string(), text_align);
        }

        Some(Block::Header {
            attributes: attrs,
            level: level,
            elements: parse_inline_elements(content)
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline, BoldTagType, ItalicTagType, Attributes};
    use super::*;

    #[test]
    fn parsers_header_correctly() {
        assert_eq!(
            parse_header("h2. *Bold text* _Italic text_"),
            Some(Block::Header {
                attributes: Attributes::default(),
                level: 2,
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
                ]
            })
        );
    }
}
