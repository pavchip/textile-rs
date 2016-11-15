use parser::{Attribute, Inline};
use parser::utils::parse_inline_attributes;
use parser::patterns::{IMAGE_PATTERN, IMAGE_URL_ALT_PATTERN};

pub fn parse_image(text: &str) -> Option<(Inline, usize)> {
    if IMAGE_PATTERN.is_match(text) {
        let caps = IMAGE_PATTERN.captures(text).unwrap();
        let group_0_len = caps.at(0).unwrap().len();
        let align = match caps.name("align").unwrap() {
            "<" => "left",
            "=" => "center",
            ">" => "right",
            _ => "",
        }.to_string();
        let (mut attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let image_url_alt_caps = IMAGE_URL_ALT_PATTERN.captures(&*text).unwrap();
        let url = image_url_alt_caps.name("url").unwrap().to_string();
        let alt = match image_url_alt_caps.name("alt") {
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
