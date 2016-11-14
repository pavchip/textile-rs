use parser::Block;
use parser::inline::parse_inline_elements;
use parser::utils::parse_block_attributes;
use regex::Regex;

pub fn parse_heading(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new("h(?P<level>[1-6])(?P<attributes>.*)\\. ").unwrap();
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
        let level: u8 = caps.name("level").unwrap().parse().unwrap();

        strings.push((&lines[0][caps.at(0).unwrap().len()..]).to_string());

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        Some((
            Block::Heading {
                attributes: parse_block_attributes(caps.name("attributes").unwrap()),
                level: level,
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
    use parser::{Attributes, Block, Inline, BoldTagType, ItalicTagType};
    use super::*;

    #[test]
    fn parses_heading_correctly() {
        assert_eq!(
            parse_heading(&vec!["h2. *Bold text* _Italic text_"]),
            Some((
                Block::Heading {
                    attributes: Attributes::new(),
                    level: 2,
                    elements: vec![
                        Inline::Bold {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Bold text".to_string())
                            ],
                            tag_type: BoldTagType::Strong,
                        },
                        Inline::Text(" ".to_string()),
                        Inline::Italic {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Italic text".to_string())
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
