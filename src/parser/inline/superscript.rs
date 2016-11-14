use parser::Inline;
use parser::inline::parse_inline_elements;
use parser::utils::parse_inline_attributes;
use regex::Regex;

pub fn parse_superscript_text(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^(?P<count1>\^+)(?P<string>.+?)(?P<count2>\^+)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let group_0 = caps.at(0).unwrap();
        let (attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 {
            Some((
                Inline::Superscript {
                    attributes: attrs,
                    elements: parse_inline_elements(&*text),
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
    fn parses_superscript_text_correctly() {
        assert_eq!(
            parse_superscript_text("^Superscript text^"),
            Some((
                Inline::Superscript {
                    attributes: Attributes::new(),
                    elements: vec![
                        Inline::Text("Superscript text".to_string())
                    ],
                },
                18
            ))
        );
    }
}
