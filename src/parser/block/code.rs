use parser::Block;

pub fn parse_code(lines: &[&str]) -> Option<(Block, usize)> {
    let mut strings = Vec::new();
    let mut cur_line = 1;

    if lines[0].starts_with("bc. ") {
        strings.push((&lines[0][4..]).to_string());

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        Some((Block::Code(strings.join("\n")), cur_line))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Block;
    use super::*;

    #[test]
    fn parsers_code_correctly() {
        assert_eq!(
            parse_code(&vec!["bc. print('Hello World')", "print(10 * 4)"]),
            Some((
                Block::Code("print('Hello World')\nprint(10 * 4)".to_string()),
                2
            ))
        );
    }
}
