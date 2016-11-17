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

    if PARAGRAPH_PATTERN.is_match(lines[0]) {
        let caps = PARAGRAPH_PATTERN.captures(lines[0]).unwrap();
        let attributes = match caps.name("attributes") {
            Some(string) => string,
            None => "",
        };

        strings.push(&lines[0][caps.at(0).unwrap().len()..]);

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line);
        }

        Some((
            Block::Paragraph {
                attributes: parse_block_attributes(attributes),
                elements: parse_inline_elements(&strings),
                starts_with_p: PARAGRAPH_PATTERN.find(&lines[0]).unwrap().1 != 0
            },
            cur_line
        ))
    } else {
        None
    }
}
