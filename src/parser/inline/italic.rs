use parser::{Inline, ItalicTagType};
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::ITALIC_TEXT_PATTERN;

pub fn parse_italic_text(text: &str) -> Option<(Inline, usize)> {
    if ITALIC_TEXT_PATTERN.is_match(text) {
        let caps = ITALIC_TEXT_PATTERN.captures(text).unwrap();
        let group_0 = caps.at(0).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 || count1 == 2 {
            let tag_type = if count1 == 1 {
                ItalicTagType::Emphasis
            } else {
                ItalicTagType::Italic
            };
            Some((
                Inline::Italic {
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
    use parser::{Attributes, Inline, ItalicTagType};
    use super::*;

    #[test]
    fn parses_italic_text_correctly() {
        assert_eq!(
            parse_italic_text("_Emphasis text_"),
            Some((
                Inline::Italic {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Emphasis text".to_string())
                    ],
                    tag_type: ItalicTagType::Emphasis,
                },
                15
            ))
        );
        assert_eq!(
            parse_italic_text("__Italic text__"),
            Some((
                Inline::Italic {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Italic text".to_string())
                    ],
                    tag_type: ItalicTagType::Italic,
                },
                15
            ))
        );
    }
}
