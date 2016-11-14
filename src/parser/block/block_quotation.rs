use parser::Block;
use parser::inline::parse_inline_elements;
use parser::utils::parse_block_attributes;
use regex::Regex;

pub fn parse_block_quotation(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new("bq(?P<attributes>.*)\\. ").unwrap();
    let pos = lines.iter().position(|el| !el.is_empty());
    let mut cur_line = match pos {
        Some(value) => {
            match value {
                0 => 1,
                _ => value + 1
            }
        }
        None => 1
    };
    let lines = match pos {
        Some(value) => &lines[value..],
        None => lines
    };
    let mut strings = Vec::new();

    if pattern.is_match(lines[0]) {
        let caps = pattern.captures(lines[0]).unwrap();
        strings.push((&lines[0][caps.at(0).unwrap().len()..]).to_string());

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        Some((
            Block::BlockQuotation {
                attributes: parse_block_attributes(caps.name("attributes").unwrap()),
                elements: parse_inline_elements(&*strings.join("\n")),
            },
            cur_line
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Attributes, Block, Inline, BoldTagType};
    use super::*;

    #[test]
    fn parses_block_quotation_correctly() {
        assert_eq!(
            parse_block_quotation(&vec!["bq. *My quote*"]),
            Some((
                Block::BlockQuotation {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Bold {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("My quote".to_string())
                            ],
                            tag_type: BoldTagType::Strong
                        }
                    ],
                },
                1
            ))
        );
    }
}
