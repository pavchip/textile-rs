use parser::inline::parse_inline_elements;
use parser::Inline;
use regex::Regex;

pub fn parse_underlined_text(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^(?P<count1>\++)(?P<text>.+?)(?P<count2>\++)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 {
            Some((Inline::Underlined(parse_inline_elements(content)), content.len() + 2))
        } else {
            let text = caps.at(0).unwrap();
            Some((Inline::Text(text.to_string()), text.len()))
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
    fn parses_underlined_text_correctly() {
        assert_eq!(
            parse_underlined_text("+Underlined text+"),
            Some((Inline::Underlined(
                vec![
                    Inline::Text("Underlined text".to_string())
                ]
            ), 17))
        );
    }
}
