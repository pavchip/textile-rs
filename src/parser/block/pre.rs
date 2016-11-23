use parser::Block;
use parser::attributes::parse_block_attributes;
use parser::block::parse_block;
use parser::patterns::PRE_PATTERN;

pub fn parse_pre_block(lines: &[&str]) -> Option<(Block, usize)> {
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

    if PRE_PATTERN.is_match(lines[0]) {
        let caps = PRE_PATTERN.captures(lines[0]).unwrap();
        let mut strings = Vec::new();
        strings.push((&lines[0]).replace(caps.at(0).unwrap(), ""));

        if caps.name("mode").unwrap().len() == 1 {
            // Breaks parsing if line is empty.
            for line in &lines[1..] {
                cur_line += 1;
                if line.is_empty() {
                    break;
                }
                strings.push(line.to_string());
            }
        } else {
            // Breaks parsing if line is block element.
            for line in &lines[1..] {
                cur_line += 1;
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
                strings.push(line.to_string());
            }
        }

        Some((
            Block::Pre {
                attributes: parse_block_attributes(caps.name("attributes").unwrap()),
                lines: strings,
            },
            cur_line
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Block;
    use super::*;

    #[test]
    fn parses_pre_block_correctly() {
        assert_eq!(
            parse_pre_block(&["pre. Pre-formatted text."]),
            Some((
                Block::Pre {
                    attributes: vec![],
                    lines: vec!["Pre-formatted text.".to_string()],
                },
                1
            ))
        );
    }
}
