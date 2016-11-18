use parser::Inline;
use parser::patterns::NO_TEXTILE_INLINE_PATTERN;

pub fn parse_no_textile(text: &str) -> Option<(Inline, usize)> {
    if NO_TEXTILE_INLINE_PATTERN.is_match(text) {
        let caps = NO_TEXTILE_INLINE_PATTERN.captures(text).unwrap();
        let group_0 = caps.at(0).unwrap();
        let string = caps.name("string").unwrap().to_string();

        Some((Inline::Text(string), group_0.len()))
    } else {
        None
    }
}
