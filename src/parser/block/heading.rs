use parser::inline::parse_inline_elements;
use parser::{Block, Properties};
use regex::Regex;

pub fn parse_heading(lines: &[&str]) -> Option<(Block, usize)> {
    let pattern = Regex::new(r"h(?P<level>[1-6])(?P<text_align>[>=]?)\. ").unwrap();
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
        let level = caps.name("level").unwrap().parse::<u8>().unwrap();
        let text_align = match caps.name("text_align").unwrap() {
            "=" => "center",
            ">" => "right",
            _ => "",
        }.to_string();
        let mut props = Properties::new();

        strings.push((&lines[0][caps.at(0).unwrap().len()..]).to_string());

        for line in &lines[1..] {
            cur_line += 1;
            if line.is_empty() {
                break;
            }
            strings.push(line.to_string());
        }

        if !text_align.is_empty() {
            props.insert("text-align".to_string(), text_align);
        }

        Some((
            Block::Heading {
                properties: props,
                level: level,
                elements: parse_inline_elements(&*strings.join("\n"))
            },
            cur_line
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Block, Inline, BoldTagType, ItalicTagType, Properties};
    use super::*;

    #[test]
    fn parses_heading_correctly() {
        assert_eq!(
            parse_heading(&vec!["h2. *Bold text* _Italic text_"]),
            Some((
                Block::Heading {
                    level: 2,
                    properties: Properties::default(),
                    elements: vec![
                        Inline::Bold(
                            vec![
                                Inline::Text("Bold text".to_string())
                            ],
                            BoldTagType::Strong
                        ),
                        Inline::Text(" ".to_string()),
                        Inline::Italic(
                            vec![
                                Inline::Text("Italic text".to_string())
                            ],
                            ItalicTagType::Emphasis
                        )
                    ]
                },
                1
            ))
        );
    }
}
