use parser::Inline;
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::LINK_PATTERN;

pub fn parse_link(text: &str) -> Option<(Inline, usize)> {
    if LINK_PATTERN.is_match(text) {
        let caps = LINK_PATTERN.captures(text).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let href = caps.name("href").unwrap().to_string();

        Some((
            Inline::Link {
                attributes: attrs,
                description: parse_inline_elements(&[&*text]),
                href: href,
            },
            caps.at(0).unwrap().len()
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Inline, ItalicTagType};
    use super::*;

    #[test]
    fn parses_link_correctly() {
        assert_eq!(
            parse_link("\"_Text_\":http://example.com"),
            Some((
                Inline::Link {
                    attributes: vec![],
                    description: vec![
                        Inline::Italic {
                            attributes: vec![],
                            elements: vec![
                                Inline::Text("Text".to_string()),
                            ],
                            tag_type: ItalicTagType::Emphasis,
                        }
                    ],
                    href: "http://example.com".to_string(),
                },
                27
            ))
        );
    }
}
