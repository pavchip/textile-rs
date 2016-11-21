use parser::Inline;
use parser::patterns::CODE_PATTERN;

pub fn parse_code(text: &str) -> Option<(Inline, usize)> {
    if CODE_PATTERN.is_match(text) {
        let caps = CODE_PATTERN.captures(text).unwrap();
        let code = caps.name("code").unwrap();

        Some((Inline::Code(code.to_string()), caps.at(0).unwrap().len()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use parser::Inline;
    use super::*;

    #[test]
    fn parses_code_correctly() {
        assert_eq!(
            parse_code("@print('Hello World')@"),
            Some((Inline::Code("print('Hello World')".to_string()), 22))
        );
        assert_eq!(
            parse_code("@@"),
            Some((Inline::Code("".to_string()), 2))
        );
    }
}
