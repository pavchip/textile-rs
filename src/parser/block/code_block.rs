use parser::Block;

pub fn parse_code_block(lines: &[&str]) -> Option<(Block, usize)> {
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

    if lines[0].starts_with("bc. ") {
        strings.push((&lines[0][4..]).to_string());

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        Some((Block::CodeBlock(strings.join("\n")), cur_line))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Block;
    use super::*;

    #[test]
    fn parses_code_correctly() {
        assert_eq!(
            parse_code_block(&vec!["bc. print('Hello World')", "print(10 * 4)"]),
            Some((
                Block::CodeBlock("print('Hello World')\nprint(10 * 4)".to_string()),
                2
            ))
        );
    }
}
