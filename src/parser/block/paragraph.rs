use parser::Block;
use parser::attributes::parse_block_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::PARAGRAPH_PATTERN;

pub fn parse_paragraph(lines: &[&str]) -> Option<(Block, usize)> {
    let pos = lines.iter().position(|el| !el.is_empty());
    let mut cur_line = match pos {
        Some(value) => {
            match value {
                0 => 1,
                _ => value + 1,
            }
        }
        None => 1,
    };
    let lines = match pos {
        Some(value) => &lines[value..],
        None => lines,
    };
    let mut attributes = "";
    let mut strings = Vec::new();

    if PARAGRAPH_PATTERN.is_match(lines[0]) {
        let caps = PARAGRAPH_PATTERN.captures(lines[0]).unwrap();
        attributes = caps.name("attributes").unwrap_or("");
        strings.push(&lines[0][caps.at(0).unwrap().len()..]);

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line);
        }
    }
    Some((
        Block::Paragraph {
            attributes: parse_block_attributes(attributes),
            elements: parse_inline_elements(&strings),
            starts_with_p: PARAGRAPH_PATTERN.find(lines[0]).unwrap().1 != 0,
        },
        cur_line
    ))
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline};
    use super::*;

    #[test]
    fn parses_paragraph_correctly() {
        assert_eq!(
            parse_paragraph(&["p. Paragraph", "with text"]),
            Some((
                Block::Paragraph {
                    attributes: vec![],
                    elements: vec![
                        Inline::Text("Paragraph".to_string()),
                        Inline::Break,
                        Inline::Text("with text".to_string()),
                    ],
                    starts_with_p: true,
                },
                2
            ))
        );
    }
}
