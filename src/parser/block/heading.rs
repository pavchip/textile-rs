use parser::Block;
use parser::attributes::parse_block_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::HEADING_PATTERN;

pub fn parse_heading(lines: &[&str]) -> Option<(Block, usize)> {
    let mut cur_line = 1;

    if HEADING_PATTERN.is_match(lines[0]) {
        let caps = HEADING_PATTERN.captures(lines[0]).unwrap();
        let level: u8 = caps.name("level").unwrap().parse().unwrap();
        let mut strings = Vec::new();
        strings.push(&lines[0][caps.at(0).unwrap().len()..]);

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line);
        }

        Some((
            Block::Heading {
                attributes: parse_block_attributes(caps.name("attributes").unwrap()),
                level: level,
                elements: parse_inline_elements(&strings),
            },
            cur_line
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline, BoldTagType, ItalicTagType};
    use super::*;

    #[test]
    fn parses_heading_correctly() {
        assert_eq!(
            parse_heading(&vec!["h2. *Bold text* _Italic text_"]),
            Some((
                Block::Heading {
                    attributes: vec![],
                    level: 2,
                    elements: vec![
                        Inline::Bold {
                            attributes: vec![],
                            elements: vec![
                                Inline::Text("Bold text".to_string()),
                            ],
                            tag_type: BoldTagType::Strong,
                        },
                        Inline::Text(" ".to_string()),
                        Inline::Italic {
                            attributes: vec![],
                            elements: vec![
                                Inline::Text("Italic text".to_string()),
                            ],
                            tag_type: ItalicTagType::Emphasis,
                        },
                    ]
                },
                1
            ))
        );
    }
}
