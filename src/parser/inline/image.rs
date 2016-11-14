use parser::{Attribute, Inline};
use parser::utils::parse_inline_attributes;
use regex::Regex;

pub fn parse_image(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new("^!(?P<align>[<|>|=]?)(?P<string>.+?)!").unwrap();
    let url_alt_pattern = Regex::new("(?P<url>[^\\(\\) ]+)(?:\\((?P<alt>.+)\\))?").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let group_0_len = caps.at(0).unwrap().len();
        let align = match caps.name("align").unwrap() {
            "<" => "left",
            "=" => "center",
            ">" => "right",
            _ => "",
        }.to_string();
        let (mut attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let url_alt_pattern_caps = url_alt_pattern.captures(&*text).unwrap();
        let url = url_alt_pattern_caps.name("url").unwrap().to_string();
        let alt = match url_alt_pattern_caps.name("alt") {
            Some(alt_text) => alt_text,
            None => "",
        }.to_string();

        if !align.is_empty() {
            attrs.push(Attribute::Align(align));
        }

        Some((
            Inline::Image {
                attributes: attrs,
                alt: alt,
                url: url,
            },
            group_0_len
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Attributes, Inline};
    use super::*;

    #[test]
    fn parses_image_with_alt_text_correctly() {
        assert_eq!(
            parse_image("!http://example.com(Example image)!"),
            Some((
                Inline::Image {
                    attributes: Attributes::new(),
                    alt: "Example image".to_string(),
                    url: "http://example.com".to_string(),
                },
                35
            ))
        );
    }

    #[test]
    fn parses_image_without_alt_text_correctly() {
        assert_eq!(
            parse_image("!http://example.com!"),
            Some((
                Inline::Image {
                    attributes: Attributes::new(),
                    alt: "".to_string(),
                    url: "http://example.com".to_string(),
                },
                20
            ))
        );
    }
}
