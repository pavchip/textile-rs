use parser::inline::parse_inline_elements;
use parser::Block;
use regex::Regex;

pub fn parse_block_quotation(text: &str) -> Option<Block> {
    let pattern = Regex::new(r"bq\. (?P<text>(?:.|[\r\n])*)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();

        Some(Block::BlockQuotation(parse_inline_elements(content)))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline, BoldTagType};
    use super::*;

    #[test]
    fn parsers_block_quotation_correctly() {
        assert_eq!(
            parse_block_quotation("bq. *My quote*"),
            Some(Block::BlockQuotation(
                vec![
                    Inline::Bold(
                        vec![
                            Inline::Text("My quote".to_string())
                        ],
                        BoldTagType::Strong
                    )
                ]
            ))
        );
    }
}
