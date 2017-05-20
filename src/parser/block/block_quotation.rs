use parser::Block;
use parser::attributes::parse_block_attributes;
use parser::block::parse_block;
use parser::block::paragraph::parse_paragraph;
use parser::inline::parse_inline_elements;
use parser::patterns::BLOCK_QUOTATION_PATTERN;

pub fn parse_block_quotation(lines: &[&str]) -> Option<(Block, usize)> {
    let mut cur_line = 1;

    if BLOCK_QUOTATION_PATTERN.is_match(lines[0]) {
        let caps = BLOCK_QUOTATION_PATTERN.captures(lines[0]).unwrap();
        let mut bq_attrs = parse_block_attributes(caps.name("attributes").unwrap());
        let p_attrs = bq_attrs.clone();
        let mut blocks = Vec::new();
        let mut strings = Vec::new();
        strings.push(&lines[0][caps.at(0).unwrap().len()..]);

        if let Some(cite) = caps.name("cite") {
            bq_attrs.insert("cite".to_string(), cite.to_string());
        }

        if caps.name("mode").unwrap().len() == 1 {
            // Breaks parsing if line is empty.
            for line in &lines[1..] {
                cur_line += 1;
                if line.is_empty() {
                    break;
                }
                strings.push(line);
            }
            blocks.push(Block::Paragraph {
                attributes: p_attrs.clone(),
                elements: parse_inline_elements(&strings),
                starts_with_p: false,
            });
        } else {
            // Breaks parsing if line is block element.
            for (idx, line) in (&lines[1..]).iter().enumerate() {
                cur_line += 1;

                if lines[idx].is_empty() {
                    match parse_block(&[line]) {
                        Some((Block::Paragraph { starts_with_p, .. }, _)) => {
                            if starts_with_p {
                                cur_line -= 1;
                                break;
                            }
                        }
                        Some(_) => {
                            cur_line -= 1;
                            break;
                        }
                        _ => {}
                    }
                }
                strings.push(line);
            }
            let mut line_pos = 0;

            while line_pos < strings.len() {
                if let Some((mut paragraph, lines_count)) =
                       parse_paragraph(&strings[line_pos..strings.len()]) {
                    if let Block::Paragraph { ref mut attributes, .. } = paragraph {
                        *attributes = p_attrs.clone();
                    }
                    line_pos += lines_count;
                    blocks.push(paragraph);
                }
            }
        }

        Some((
            Block::BlockQuotation {
                attributes: bq_attrs,
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
    use parser::{Attributes, Block, Inline};
    use super::*;

    #[test]
    fn parses_block_quotation_correctly() {
        assert_eq!(
            parse_block_quotation(&["bq. *My quote*"]),
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
                                        Inline::Text("My quote".to_string()),
                                    ],
                                    tag_type: "strong".to_string(),
                                },
                            ],
                            starts_with_p: false,
                        }
                    ],
                },
                1
            ))
        );
    }

    #[test]
    fn parses_multiline_block_quotation_correctly() {
        assert_eq!(
            parse_block_quotation(&["bq.. Block quotation", "", "in multiline mode"]),
            Some((
                Block::BlockQuotation {
                    attributes: Attributes::new(),
                    elements: vec![
                        Block::Paragraph {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Block quotation".to_string()),
                            ],
                            starts_with_p: false,
                        },
                        Block::Paragraph {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("in multiline mode".to_string()),
                            ],
                            starts_with_p: false,
                        },
                    ],
                },
                3
            ))
        );
        assert_eq!(
            parse_block_quotation(&["bq.. Block quotation", "", "in multiline mode", "", "h1. Heading"]),
            Some((
                Block::BlockQuotation {
                    attributes: Attributes::new(),
                    elements: vec![
                        Block::Paragraph {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Block quotation".to_string()),
                            ],
                            starts_with_p: false,
                        },
                        Block::Paragraph {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("in multiline mode".to_string()),
                            ],
                            starts_with_p: false,
                        },
                    ],
                },
                4
            ))
        );
    }

    #[test]
    fn parses_block_quotation_with_cite_correctly() {
        assert_eq!(
            parse_block_quotation(&["bq.:http://example.com Block quotation"]),
            Some((
                Block::BlockQuotation {
                    attributes: hashmap!{
                        "cite".to_string() => "http://example.com".to_string(),
                    },
                    elements: vec![
                        Block::Paragraph {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Block quotation".to_string()),
                            ],
                            starts_with_p: false,
                        },
                    ],
                },
                1
            ))
        );
    }
}
