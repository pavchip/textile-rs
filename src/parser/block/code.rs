use parser::Block;
use regex::Regex;

pub fn parse_code(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new(r"bc\. (?P<text>.*)").unwrap();
    let mut strings = Vec::new();
    let mut cur_line = 1;

    if pattern.is_match(lines[0]) {
        let caps = pattern.captures(lines[0]).unwrap();
        let text = caps.name("text").unwrap().to_string();

        strings.push(text);

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
