use parser::inline::parse_inline_elements;
use parser::Inline;
use regex::Regex;

pub fn parse_subscript_text(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new("^(?P<count1>~+)(?P<text>.+?)(?P<count2>~+)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let content = caps.name("text").unwrap();
        let count1 = caps.name("count1").unwrap().len();
        let count2 = caps.name("count2").unwrap().len();

        if count1 == count2 && count1 == 1 {
            Some((Inline::Subscript(parse_inline_elements(content)), content.len() + 2))
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
    fn parses_subscript_text_correctly() {
        assert_eq!(
            parse_subscript_text("~Subscript text~"),
            Some((Inline::Subscript(
                vec![
                    Inline::Text("Subscript text".to_string())
                ]
            ), 16))
        );
    }
}
