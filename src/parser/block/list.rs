use parser::{Block, ListElement};
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::{ORDERED_LIST_PATTERN, UNORDERED_LIST_PATTERN};
use regex::Captures;

pub fn parse_list(lines: &[&str]) -> Option<(Block, usize)> {
    let empty_line_pos = lines.iter().position(|el| el.is_empty());
    let lines = match empty_line_pos {
        Some(value) => &lines[..value],
        None => &lines,
    };

    if let Some((list, _)) = parse_multilevel_list(lines, 0) {
        Some((list, empty_line_pos.unwrap_or(lines.len()) + 1))
    } else {
        None
    }
}

pub fn parse_multilevel_list(lines: &[&str], list_level: usize) -> Option<(Block, usize)> {
    lazy_static! {
        static ref FNS: Vec<fn(&[&str], usize) -> Option<(Block, usize)>> = vec![
            parse_ordered_list,
            parse_unordered_list,
        ];
    }

    for f in FNS.iter() {
        let res = f(lines, list_level);

        if res.is_some() {
            return res;
        }
    }
    None
}

fn parse_ordered_list(lines: &[&str], list_level: usize) -> Option<(Block, usize)> {
    if ORDERED_LIST_PATTERN.is_match(lines[0]) {
        let mut line_idx = 0;
        let mut elements = Vec::new();
        let mut start = None;

        while line_idx < lines.len() {
            let (caps, list_type) = get_list_data(lines[line_idx]);
            let level = caps.name("level").unwrap().len() - 1;

            if line_idx == 0 {
                start = match caps.name("start") {
                    Some(val) => Some(val.parse::<u8>().unwrap()),
                    None => None,
                };
            }

            if level == list_level && list_type == 'o' {
                let (attrs, _) = parse_inline_attributes(caps.name("attributes").unwrap());
                let mut tmp = vec![&lines[line_idx][caps.at(0).unwrap().len()..]];
                line_idx += 1;

                while line_idx < lines.len() {
                    if !is_list_item(lines[line_idx]) {
                        tmp.push(lines[line_idx]);
                    } else {
                        break;
                    }
                    line_idx += 1;
                }

                elements.push(ListElement::ListItem {
                    attributes: attrs,
                    elements: parse_inline_elements(&tmp),
                });
            } else if level > list_level {
                let (list, consumed_lines) = parse_multilevel_list(&lines[line_idx..], level).unwrap();
                elements.push(ListElement::List(list));
                line_idx += consumed_lines;
            } else {
                break;
            }
        }

        Some((
            Block::OrderedList {
                attributes: Vec::new(),
                elements: elements,
                level: list_level as u8,
                start: start,
            },
            line_idx
        ))
    } else {
        None
    }
}

fn parse_unordered_list(lines: &[&str], list_level: usize) -> Option<(Block, usize)> {
    if UNORDERED_LIST_PATTERN.is_match(lines[0]) {
        let mut line_idx = 0;
        let mut elements = Vec::new();

        while line_idx < lines.len() {
            let (caps, list_type) = get_list_data(lines[line_idx]);
            let level = caps.name("level").unwrap().len() - 1;

            if level == list_level && list_type == 'u' {
                let (attrs, _) = parse_inline_attributes(caps.name("attributes").unwrap());
                let mut tmp = vec![&lines[line_idx][caps.at(0).unwrap().len()..]];
                line_idx += 1;

                while line_idx < lines.len() {
                    if !is_list_item(lines[line_idx]) {
                        tmp.push(lines[line_idx]);
                    } else {
                        break;
                    }
                    line_idx += 1;
                }

                elements.push(ListElement::ListItem {
                    attributes: attrs,
                    elements: parse_inline_elements(&tmp),
                });
            } else if level > list_level {
                let (list, consumed_lines) = parse_multilevel_list(&lines[line_idx..], level).unwrap();
                elements.push(ListElement::List(list));
                line_idx += consumed_lines;
            } else {
                break;
            }
        }

        Some((
            Block::UnorderedList {
                attributes: Vec::new(),
                elements: elements,
                level: list_level as u8,
            },
            line_idx
        ))
    } else {
        None
    }
}

fn is_list_item(line: &str) -> bool {
    UNORDERED_LIST_PATTERN.is_match(line) || ORDERED_LIST_PATTERN.is_match(line)
}

fn get_list_data(line: &str) -> (Captures, char) {
    if UNORDERED_LIST_PATTERN.is_match(line) {
        (UNORDERED_LIST_PATTERN.captures(line).unwrap(), 'u')
    } else {
        (ORDERED_LIST_PATTERN.captures(line).unwrap(), 'o')
    }
}
