use parser::Block;
use parser::block::parse_block;
use parser::patterns::COMMENT_PATTERN;

pub fn parse_comment(lines: &[&str]) -> Option<(Block, usize)> {
    let mut cur_line = 1;

    if COMMENT_PATTERN.is_match(lines[0]) {
        let caps = COMMENT_PATTERN.captures(lines[0]).unwrap();
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
                strings.push(line.to_string());
            }
        }

        Some((Block::Comment(strings), cur_line))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Block;
    use super::*;

    #[test]
    fn parses_comment_block_correctly() {
        assert_eq!(
            parse_comment(&vec!["###. Comment block"]),
            Some((
                Block::Comment(vec!["Comment block".to_string()]),
                1
            ))
        );
    }

    #[test]
    fn parses_multiline_comment_block_correctly() {
        assert_eq!(
            parse_comment(&vec!["###.. Comment block", "", "in multiline mode", "", "p. Paragraph"]),
            Some((
                Block::Comment(vec![
                    "Comment block".to_string(),
                    "".to_string(),
                    "in multiline mode".to_string(),
                    "".to_string(),
                ]),
                4
            ))
        );
    }
}
