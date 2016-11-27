use parser::{Attribute, Inline};
use parser::attributes::parse_inline_attributes;
use parser::patterns::{IMAGE_PATTERN, IMAGE_SRC_ALT_PATTERN};

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
        let href = caps.name("href").unwrap_or("").to_string();
        let (mut attrs, text) = parse_inline_attributes(caps.name("string").unwrap());
        let image_src_alt_caps = IMAGE_SRC_ALT_PATTERN.captures(&*text).unwrap();
        let alt = image_src_alt_caps.name("alt").unwrap_or("").to_string();
        let src = image_src_alt_caps.name("src").unwrap().to_string();

        if !align.is_empty() {
            attrs.push(Attribute::Align(align));
        }

        Some((
            Inline::Image {
                attributes: attrs,
                alt: alt,
                href: href,
                src: src,
            },
            group_0_len
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::{Attribute, Inline};
    use super::*;

    #[test]
    fn parses_image_with_alt_text_correctly() {
        assert_eq!(
            parse_image("!http://example.com(Example image)!"),
            Some((
                Inline::Image {
                    attributes: vec![],
                    alt: "Example image".to_string(),
                    href: "".to_string(),
                    src: "http://example.com".to_string(),
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
                    attributes: vec![],
                    alt: "".to_string(),
                    href: "".to_string(),
                    src: "http://example.com".to_string(),
                },
                20
            ))
        );
    }

    #[test]
    fn parses_image_with_align_correctly() {
        assert_eq!(
            parse_image("!>http://example.com(Example image)!"),
            Some((
                Inline::Image {
                    attributes: vec![
                        Attribute::Align("right".to_string()),
                    ],
                    alt: "Example image".to_string(),
                    href: "".to_string(),
                    src: "http://example.com".to_string(),
                },
                36
            ))
        );
    }

    #[test]
    fn parses_image_with_link_correctly() {
        assert_eq!(
            parse_image("!http://example.com/image.jpg(Example image)!:http://example.com"),
            Some((
                Inline::Image {
                    attributes: vec![],
                    alt: "Example image".to_string(),
                    href: "http://example.com".to_string(),
                    src: "http://example.com/image.jpg".to_string(),
                },
                64
            ))
        );
    }
}
