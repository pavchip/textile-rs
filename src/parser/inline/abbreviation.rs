use parser::Inline;
use parser::patterns::ABBREVIATION_PATTERN;

pub fn parse_abbreviation(text: &str) -> Option<(Inline, usize)> {
    if ABBREVIATION_PATTERN.is_match(text) {
        let caps = ABBREVIATION_PATTERN.captures(text).unwrap();
        let abbreviation = caps.name("abbreviation").unwrap().to_string();
        let transcript = caps.name("transcript").unwrap_or("").to_string();

        if transcript.is_empty() {
            Some((
                Inline::Span {
                    attributes: vec![],
                    elements: vec![
                        Inline::Text(abbreviation),
                    ],
                },
                caps.at(0).unwrap().len()
            ))
        } else {
            Some((
                Inline::Abbreviation {
                    abbr: abbreviation,
                    transcript: transcript,
                },
                caps.at(0).unwrap().len()
            ))
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
    fn parses_abbreviation_correctly() {
        assert_eq!(
            parse_abbreviation("ABBR(Abbreviation)"),
            Some((
                Inline::Abbreviation {
                    abbr: "ABBR".to_string(),
                    transcript: "Abbreviation".to_string(),
                },
                18
            ))
        );
    }

    #[test]
    fn parses_uppercase_word_correctly() {
        assert_eq!(
            parse_abbreviation("ABBR"),
            Some((
                Inline::Span {
                    attributes: vec![],
                    elements: vec![
                        Inline::Text("ABBR".to_string()),
                    ],
                },
                4
            ))
        );
    }

    #[test]
    fn not_parse_if_abbr_less_than_3_chars() {
        assert_eq!(parse_abbreviation("AB(Abbreviation)"), None);
    }
}
