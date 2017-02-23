use parser::Inline;
use parser::attributes::parse_inline_attributes;
use parser::patterns::{IMAGE_PATTERN, IMAGE_ALT_PATTERN};

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
        let (attrs, string) = parse_inline_attributes(caps.name("string").unwrap());
        let image_alt_caps = IMAGE_ALT_PATTERN.captures(&*string).unwrap();
        let alt = image_alt_caps.at(1).unwrap_or("").to_string();
        let src = IMAGE_ALT_PATTERN.replace(&*string, "");

        Some((
            Inline::Image {
                attributes: attrs,
                align: align,
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
    use parser::{Attributes, Inline};
    use super::*;

    #[test]
    fn parses_image_with_alt_text_correctly() {
        assert_eq!(
            parse_image("!http://example.com(Example image)!"),
            Some((
                Inline::Image {
                    attributes: Attributes::new(),
                    align: "".to_string(),
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
                    attributes: Attributes::new(),
                    align: "".to_string(),
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
                    attributes: Attributes::new(),
                    align: "right".to_string(),
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
                    attributes: Attributes::new(),
                    align: "".to_string(),
                    alt: "Example image".to_string(),
                    href: "http://example.com".to_string(),
                    src: "http://example.com/image.jpg".to_string(),
                },
                64
            ))
        );
    }
}
