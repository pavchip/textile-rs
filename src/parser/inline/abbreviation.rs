use parser::Inline;
use regex::Regex;

pub fn parse_abbreviation(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^(?P<abbreviation>\p{Lu}{3,})\((?P<transcript>.*?)\)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let abbreviation = caps.name("abbreviation").unwrap().to_string();
        let transcript = caps.name("transcript").unwrap().to_string();

        Some((
            Inline::Abbreviation {
                abbr: abbreviation,
                transcript: transcript
            },
            caps.at(0).unwrap().len()
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
    fn parses_abbreviation_correctly() {
        assert_eq!(
            parse_abbreviation("ABBR(Abbreviation)"),
            Some((
                Inline::Abbreviation {
                    abbr: "ABBR".to_string(),
                    transcript: "Abbreviation".to_string()
                },
                18
            ))
        );
    }

    #[test]
    fn not_parse_if_abbr_less_than_3_chars() {
        assert_eq!(parse_abbreviation("AB(Abbreviation)"), None);
    }
}
