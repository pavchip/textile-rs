use parser::inline::parse_inline_elements;
use parser::Block;
use regex::Regex;
use std::collections::HashMap;

pub fn parse_header(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new(r"h(?P<level>[1-6])(?P<text_align>[>=]?)\. (?P<text>.*)").unwrap();
    let mut strings = Vec::new();
    let mut cur_line = 1;

    if pattern.is_match(lines[0]) {
        let caps = pattern.captures(lines[0]).unwrap();
        let text = caps.name("text").unwrap().to_string();
        let level = caps.name("level").unwrap().parse::<u8>().unwrap();
        let text_align = match caps.name("text_align").unwrap() {
            "=" => "center",
            ">" => "right",
            _ => "",
        }.to_string();
        let mut attrs = HashMap::<String, String>::new();

        strings.push(text);

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        if !text_align.is_empty() {
            attrs.insert("text-align".to_string(), text_align);
        }

        Some((
            Block::Header {
                attributes: attrs,
                level: level,
                elements: parse_inline_elements(&*strings.join("\n"))
            },
            cur_line
        ))
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
            parse_header(&vec!["h2. *Bold text* _Italic text_"]),
            Some((
                Block::Header {
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
                },
                1
            ))
        );
    }
}
