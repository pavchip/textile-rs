use parser::inline::parse_inline_elements;
use parser::{Inline, ItalicTagType};
use regex::Regex;

pub fn parse_italic_text(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new("^(?P<count1>_+)(?P<text>.+?)(?P<count2>_+)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 || count1 == 2 {
            let tag_type = if count1 == 1 {
                ItalicTagType::Emphasis
            } else {
                ItalicTagType::Italic
            };
            Some((Inline::Italic(parse_inline_elements(content), tag_type),
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
    use parser::{Inline, ItalicTagType};
    use super::*;

    #[test]
    fn parses_italic_text_correctly() {
        assert_eq!(
            parse_italic_text("_Emphasis text_"),
            Some((Inline::Italic(
                vec![
                    Inline::Text("Emphasis text".to_string())
                ],
                ItalicTagType::Emphasis
            ), 15))
        );
        assert_eq!(
            parse_italic_text("__Italic text__"),
            Some((Inline::Italic(
                vec![
                    Inline::Text("Italic text".to_string())
                ],
                ItalicTagType::Italic
            ), 15))
        );
    }
}
