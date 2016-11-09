use parser::Block;
use parser::inline::parse_inline_elements;
use regex::Regex;

pub fn parse_paragraph(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new("(?:p\\. )?").unwrap();
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

        Some(
            (Block::Paragraph {
                elements: parse_inline_elements(&*strings.join("\n"))
            },
            cur_line
        ))
    } else {
        None
    }
}
