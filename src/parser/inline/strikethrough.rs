use parser::Inline;
use parser::inline::parse_inline_elements;
use parser::patterns::STRIKETHROUGH_TEXT_PATTERN;
use parser::utils::parse_inline_attributes;

pub fn parse_strikethrough_text(text: &str) -> Option<(Inline, usize)> {
    if STRIKETHROUGH_TEXT_PATTERN.is_match(text) {
        let caps = STRIKETHROUGH_TEXT_PATTERN.captures(text).unwrap();
        let group_0 = caps.at(0).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 {
            Some((
                Inline::Strikethrough {
                    attributes: attrs,
                    elements: parse_inline_elements(&[&*text]),
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
    use parser::{Attributes, Inline};
    use super::*;

    #[test]
    fn parses_strikethrough_text_correctly() {
        assert_eq!(
            parse_strikethrough_text("-Strikethrough text-"),
            Some((
                Inline::Strikethrough {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Strikethrough text".to_string())
                    ],
                },
                20
            ))
        );
    }
}
