use parser::Block;
use parser::block::parse_block;
use parser::patterns::NO_TEXTILE_BLOCK_PATTERN;

pub fn parse_no_textile(lines: &[&str]) -> Option<(Block, usize)> {
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

    if NO_TEXTILE_BLOCK_PATTERN.is_match(lines[0]) {
        let caps = NO_TEXTILE_BLOCK_PATTERN.captures(lines[0]).unwrap();
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

        Some((Block::NoTextileBlock(strings), cur_line))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Block;
    use super::*;

    #[test]
    fn parses_no_textile_block_correclty() {
        assert_eq!(
            parse_no_textile(&["notextile. No *Textile formatting* in this block."]),
            Some((
                Block::NoTextileBlock(vec!["No *Textile formatting* in this block.".to_string()]),
                1
            ))
        );
    }
}