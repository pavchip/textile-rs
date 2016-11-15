use parser::Block;
use parser::block::parse_block;
use parser::inline::parse_inline_elements;
use parser::utils::parse_block_attributes;
use regex::Regex;

pub fn parse_block_quotation(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new("bq(?P<attributes>.*?)(?P<mode>\\.{1,2}) ").unwrap();
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
    let mut blocks = Vec::new();
    let mut strings = Vec::new();

    if pattern.is_match(lines[0]) {
        let caps = pattern.captures(lines[0]).unwrap();
        let attrs = parse_block_attributes(caps.name("attributes").unwrap());
        strings.push((&lines[0][caps.at(0).unwrap().len()..]).to_string());

        if caps.name("mode").unwrap().len() == 1 {
            // Breaks parsing if line is empty.
            for line in &lines[1..] {
                cur_line += 1;
                if line.is_empty() {
                    break;
                }
                strings.push(line.to_string());
            }
            blocks.push(Block::Paragraph {
                attributes: attrs.clone(),
                elements: parse_inline_elements(&*strings.join("\n")),
                starts_with_p: false,
            });
        } else {
            // Breaks parsing if line is block element.
            let mut paragraph_idx = 0;
            for line in &lines[1..] {
                // Make paragraph if line is empty.
                if line.is_empty() {
                    blocks.push(Block::Paragraph {
                        attributes: attrs.clone(),
                        elements: parse_inline_elements(&*(&strings[paragraph_idx..cur_line]).join("\n")),
                        starts_with_p: false,
                    });
                    paragraph_idx = cur_line + 1;
                }
                cur_line += 1;
                match parse_block(&[line]) {
                    Some((Block::Paragraph {starts_with_p, ..}, _)) => {
                        if starts_with_p {
                            cur_line -= 1;
                            break;
                        }
                    },
                    Some(_) => {
                        cur_line -= 1;
                        break;
                    },
                    _ => {},
                }
                strings.push(line.to_string());
            }
        }

        Some((
            Block::BlockQuotation {
                attributes: attrs.clone(),
                elements: blocks,
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
                        Block::Paragraph {
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
                            starts_with_p: false,
                        }
                    ],
                },
                1
            ))
        );
    }
}
