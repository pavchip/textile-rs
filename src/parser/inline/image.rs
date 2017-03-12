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
        let (mut attrs, string) = parse_inline_attributes(caps.name("string").unwrap());
        let image_alt_caps = IMAGE_ALT_PATTERN.captures(&*string).unwrap();
        let src = IMAGE_ALT_PATTERN.replace(&*string, "");
        attrs.insert("src".to_string(), src);

        if !align.is_empty() {
            attrs.insert("align".to_string(), align);
        }

        if let Some(alt) = image_alt_caps.at(1) {
            attrs.insert("alt".to_string(), alt.to_string());
            attrs.insert("title".to_string(), alt.to_string());
        }

        Some((
            Inline::Image {
                attributes: attrs,
                href: href,
            },
            group_0_len
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
    fn parses_image_with_alt_text_correctly() {
        assert_eq!(
            parse_image("!http://example.com(Example image)!"),
            Some((
                Inline::Image {
                    attributes: hashmap!{
                        "alt".to_string() => "Example image".to_string(),
                        "src".to_string() => "http://example.com".to_string(),
                        "title".to_string() => "Example image".to_string(),
                    },
                    href: "".to_string(),
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
                    attributes: hashmap!{
                        "src".to_string() => "http://example.com".to_string(),
                    },
                    href: "".to_string(),
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
                    attributes: hashmap!{
                        "align".to_string() => "right".to_string(),
                        "alt".to_string() => "Example image".to_string(),
                        "title".to_string() => "Example image".to_string(),
                        "src".to_string() => "http://example.com".to_string(),
                    },
                    href: "".to_string(),
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
                    attributes: hashmap!{
                        "alt".to_string() => "Example image".to_string(),
                        "title".to_string() => "Example image".to_string(),
                        "src".to_string() => "http://example.com/image.jpg".to_string(),
                    },
                    href: "http://example.com".to_string(),
                },
                64
            ))
        );
    }
}
