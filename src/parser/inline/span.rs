use parser::Inline;
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::SPAN_PATTERN;

pub fn parse_span(text: &str) -> Option<(Inline, usize)> {
    if SPAN_PATTERN.is_match(text) {
        let caps = SPAN_PATTERN.captures(text).unwrap();
        let group_0 = caps.at(0).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 {
            Some((
                Inline::Span {
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
    fn parses_span_correctly() {
        assert_eq!(
            parse_span("%Span element%"),
            Some((
                Inline::Span {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Span element".to_string()),
                    ],
                },
                14
            ))
        );
    }
}
