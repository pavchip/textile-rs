use parser::Inline;
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::CITATION_PATTERN;

pub fn parse_citation(text: &str) -> Option<(Inline, usize)> {
    if CITATION_PATTERN.is_match(text) {
        let caps = CITATION_PATTERN.captures(text).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());

        Some((
            Inline::Citation {
                attributes: attrs,
                elements: parse_inline_elements(&[&*text]),
            },
            caps.at(0).unwrap().len()
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Inline;
    use super::*;

    #[test]
    fn parses_citation_correctly() {
        assert_eq!(
            parse_citation("??Textile citation element??"),
            Some((
                Inline::Citation {
                    attributes: vec![],
                    elements: vec![
                        Inline::Text("Textile citation element".to_string()),
                    ],
                },
                28
            ))
        );
    }
}
