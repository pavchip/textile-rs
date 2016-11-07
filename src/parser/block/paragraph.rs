use parser::inline::parse_inline_elements;
use parser::Block;
use regex::Regex;

pub fn parse_paragraph(text: &str) -> Block {
    let pattern = Regex::new("(?:p\\. )?(?P<text>(?:.|[\r\n])*)").unwrap();
    let caps = pattern.captures(text).unwrap();
    let content = caps.name("text").unwrap();

    Block::Paragraph {
        elements: parse_inline_elements(content)
    }
}
