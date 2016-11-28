use parser::Block;
use parser::attributes::parse_block_attributes;
use parser::block::parse_block;
use parser::patterns::CODE_BLOCK_PATTERN;

pub fn parse_code_block(lines: &[&str]) -> Option<(Block, usize)> {
    let mut cur_line = 1;

    if CODE_BLOCK_PATTERN.is_match(lines[0]) {
        let caps = CODE_BLOCK_PATTERN.captures(lines[0]).unwrap();
        let mut strings = Vec::new();
        strings.push(&lines[0][caps.at(0).unwrap().len()..]);

        if caps.name("mode").unwrap().len() == 1 {
            // Breaks parsing if line is empty.
            for line in &lines[1..] {
                cur_line += 1;
                if line.is_empty() {
                    break;
                }
                strings.push(line);
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
                strings.push(line);
            }
        }

        Some((
            Block::CodeBlock {
                attributes: parse_block_attributes(caps.name("attributes").unwrap()),
                code: strings.join("\n").trim_right().to_string(),
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
    fn parses_code_correctly() {
        assert_eq!(
            parse_code_block(&["bc. print('Hello World')", "print(10 * 4)"]),
            Some((
                Block::CodeBlock {
                    attributes: vec![],
                    code: "print('Hello World')\nprint(10 * 4)".to_string()
                },
                2
            ))
        );
    }

    #[test]
    fn parses_multiline_code_block_correctly() {
        assert_eq!(
            parse_code_block(&[
                "bc.. #include <iostream>",
                "using namespace std",
                "",
                "int main() {",
                "    cout << \"Hello, world!\" << endl;",
                "    return 0;",
                "}",
            ]),
            Some((
                Block::CodeBlock {
                    attributes: vec![],
                    code: "#include <iostream>\nusing namespace std\n\nint main() {\n    cout << \"Hello, world!\" << endl;\n    return 0;\n}".to_string(),
                },
                7
            ))
        );
        assert_eq!(
            parse_code_block(&[
                "bc.. #include <iostream>",
                "using namespace std",
                "",
                "int main() {",
                "    cout << \"Hello, world!\" << endl;",
                "    return 0;",
                "}",
                "",
                "p. Paragraph",
            ]),
            Some((
                Block::CodeBlock {
                    attributes: vec![],
                    code: "#include <iostream>\nusing namespace std\n\nint main() {\n    cout << \"Hello, world!\" << endl;\n    return 0;\n}".to_string(),
                },
                8
            ))
        );
    }
}
