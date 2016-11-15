use parser::Inline;
use parser::inline::parse_inline_elements;
use parser::patterns::LINK_PATTERN;
use parser::utils::parse_inline_attributes;

pub fn parse_link(text: &str) -> Option<(Inline, usize)> {
    if LINK_PATTERN.is_match(text) {
        let caps = LINK_PATTERN.captures(text).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let url = caps.name("url").unwrap().to_string();

        Some((
            Inline::Link {
                attributes: attrs,
                description: parse_inline_elements(&[&*text]),
                url: url,
            },
            caps.at(0).unwrap().len()
        ))
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
            parse_link("\"_Text_\":http://example.com"),
            Some((
                Inline::Link {
                    attributes: Attributes::new(),
                    description: vec![
                        Inline::Italic {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Text".to_string())
                            ],
                            tag_type: ItalicTagType::Emphasis,
                        }
                    ],
                    url: "http://example.com".to_string(),
                },
                27
            ))
        );
    }
}
