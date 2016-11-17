use parser::{Inline, BoldTagType};
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::BOLD_TEXT_PATTERN;

pub fn parse_bold_text(text: &str) -> Option<(Inline, usize)> {
    if BOLD_TEXT_PATTERN.is_match(text) {
        let caps = BOLD_TEXT_PATTERN.captures(text).unwrap();
        let group_0 = caps.at(0).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 || count1 == 2 {
            let tag_type = if count1 == 1 {
                BoldTagType::Strong
            } else {
                BoldTagType::Bold
            };
            Some((
                Inline::Bold {
                    attributes: attrs,
                    elements: parse_inline_elements(&[&*text]),
                    tag_type: tag_type,
                },
                group_0.len()
            ))
        } else {
            Some((Inline::Text(group_0.to_string()), group_0.len()))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Attributes, Inline, BoldTagType};
    use super::*;

    #[test]
    fn parses_bold_text_correctly() {
        assert_eq!(
            parse_bold_text("*Strong text*"),
            Some((
                Inline::Bold {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Strong text".to_string())
                    ],
                    tag_type: BoldTagType::Strong
                },
                13
            ))
        );
        assert_eq!(
            parse_bold_text("**Bold text**"),
            Some((
                Inline::Bold {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Bold text".to_string())
                    ],
                    tag_type: BoldTagType::Bold
                },
                13
            ))
        );
    }
}
