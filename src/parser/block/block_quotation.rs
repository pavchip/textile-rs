use parser::inline::parse_inline_elements;
use parser::Block;

pub fn parse_block_quotation(lines: &[&str]) -> Option<(Block, usize)> {
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

    if lines[0].starts_with("bq. ") {
        strings.push((&lines[0][4..]).to_string());

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        Some((Block::BlockQuotation(parse_inline_elements(&*strings.join("\n"))), cur_line))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline, BoldTagType};
    use super::*;

    #[test]
    fn parses_block_quotation_correctly() {
        assert_eq!(
            parse_block_quotation(&vec!["bq. *My quote*"]),
            Some((
                Block::BlockQuotation(
                    vec![
                        Inline::Bold(
                            vec![
                                Inline::Text("My quote".to_string())
                            ],
                            BoldTagType::Strong
                        )
                    ]
                ),
                1
            ))
        );
    }
}
