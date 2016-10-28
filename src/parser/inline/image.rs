use parser::Inline;
use regex::Regex;

pub fn parse_image(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^!(?P<url>[^ \(\)]+)(?:\((?P<alt>.+?)\))?!").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let url = caps.name("url").unwrap();
        let alt = caps.name("alt");

        match alt {
            Some(alt_text) => Some((Inline::Image {
                url: url.to_string(),
                alt: Some(alt_text.to_string())
            }, url.len() + alt_text.len() + 4)),
            None => Some((Inline::Image {
                url: url.to_string(),
                alt: None
            }, url.len() + 2))
        }
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
            Some((Inline::Image {
                url: "http://example.com".to_string(),
                alt: Some("Example image".to_string())
            }, 35))
        );
    }

    #[test]
    fn parses_image_without_alt_text_correctly() {
        assert_eq!(
            parse_image("!http://example.com!"),
            Some((Inline::Image {
                url: "http://example.com".to_string(),
                alt: None
            }, 20))
        );
    }
}
