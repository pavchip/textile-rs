use parser::Inline;
use parser::attributes::parse_inline_attributes;
use parser::inline::parse_inline_elements;
use parser::patterns::{LINK_PATTERN, LINK_TITLE_PATTERN};
use url::{Origin, Url};

pub fn parse_link(text: &str) -> Option<(Inline, usize)> {
    if LINK_PATTERN.is_match(text) {
        let caps = LINK_PATTERN.captures(text).unwrap();
        let (mut attrs, mut string) = parse_inline_attributes(caps.name("string").unwrap());
        let href = caps.name("href").unwrap().to_string();

        if let Some(title) = LINK_TITLE_PATTERN.captures(&*string).unwrap().at(1) {
            attrs.insert("title".to_string(), title.to_string());
        }
        
        string = LINK_TITLE_PATTERN.replace(&*string, "");
        let elements = if string != "$" {
            parse_inline_elements(&[&*string])
        } else {
            let desc = match Url::parse(&*href) {
                Ok(url) => {
                    match url.origin() {
                        Origin::Tuple(..) => {
                            let path = url.path();
                            let domain = url.domain().unwrap().to_string();

                            if path != "/" {
                                domain + path
                            } else {
                                domain
                            }
                        }
                        Origin::Opaque(_) => href.replace(&format!("{}:", url.scheme()), "")
                    }
                }
                Err(_) => href.to_string()
            };
            vec![Inline::Text(desc)]
        };
        attrs.insert("href".to_string(), href);

        Some((
            Inline::Link {
                attributes: attrs,
                elements: elements,
            },
            caps.at(0).unwrap().len()
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
    fn parses_link_correctly() {
        assert_eq!(
            parse_link("\"_Text_\":http://example.com"),
            Some((
                Inline::Link {
                    attributes: hashmap!{
                        "href".to_string() => "http://example.com".to_string(),
                    },
                    elements: vec![
                        Inline::Italic {
                            attributes: Attributes::new(),
                            elements: vec![
                                Inline::Text("Text".to_string()),
                            ],
                            tag_type: "em".to_string(),
                        }
                    ],
                },
                27
            ))
        );
    }

    #[test]
    fn parses_link_with_title_correctly() {
        assert_eq!(
            parse_link("\"Link(With title)\":http://example.com"),
            Some((
                Inline::Link {
                    attributes: hashmap!{
                        "href".to_string() => "http://example.com".to_string(),
                        "title".to_string() => "With title".to_string(),
                    },
                    elements: vec![
                        Inline::Text("Link".to_string()),
                    ],
                },
                37
            ))
        );
    }

    #[test]
    fn parses_link_with_dollar_shorthand_correctly() {
        assert_eq!(
            parse_link("\"$\":http://example.com"),
            Some((
                Inline::Link {
                    attributes: hashmap!{
                        "href".to_string() => "http://example.com".to_string(),
                    },
                    elements: vec![
                        Inline::Text("example.com".to_string()),
                    ],
                },
                22
            ))
        );
        assert_eq!(
            parse_link("\"$\":mailto:user@example.com"),
            Some((
                Inline::Link {
                    attributes: hashmap!{
                        "href".to_string() => "mailto:user@example.com".to_string(),
                    },
                    elements: vec![
                        Inline::Text("user@example.com".to_string()),
                    ],
                },
                27
            ))
        );
    }
}
