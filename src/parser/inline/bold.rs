use parser::inline::parse_inline_elements;
use parser::{Inline, BoldTagType};
use regex::Regex;

pub fn parse_bold_text(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^(?P<count1>\*+)(?P<text>.+?)(?P<count2>\*+)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 || count1 == 2 {
            let tag_type = if count1 == 1 {
                BoldTagType::Strong
            } else {
                BoldTagType::Bold
            };
            Some((Inline::Bold(parse_inline_elements(content), tag_type),
                  content.len() + count1 * 2))
        } else {
            let text = caps.at(0).unwrap();
            Some((Inline::Text(text.to_string()), text.len()))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Inline, BoldTagType};
    use super::*;

    #[test]
    fn parses_bold_text_correctly() {
        assert_eq!(
            parse_bold_text("*Strong text*"),
            Some((Inline::Bold(
                vec![
                    Inline::Text("Strong text".to_string())
                ],
                BoldTagType::Strong
            ), 13))
        );
        assert_eq!(
            parse_bold_text("**Bold text**"),
            Some((Inline::Bold(
                vec![
                    Inline::Text("Bold text".to_string())
                ],
                BoldTagType::Bold
            ), 13))
        );
    }
}
