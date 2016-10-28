use parser::Inline;
use regex::Regex;

pub fn parse_abbreviation(text: &str) -> Option<(Inline, usize)> {
    let pattern = Regex::new(r"^(?P<abbreviation>\p{Lu}{3,})\((?P<transcript>.*?)\)").unwrap();

    if pattern.is_match(text) {
        let caps = pattern.captures(text).unwrap();
        let abbreviation = caps.name("abbreviation").unwrap();
        let transcript = caps.name("transcript").unwrap();

        Some((Inline::Abbreviation {
            abbr: abbreviation.to_string(),
            transcript: transcript.to_string()
        }, abbreviation.len() + transcript.len() + 2))
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
            Some((Inline::Abbreviation {
                abbr: "ABBR".to_string(),
                transcript: "Abbreviation".to_string()
            }, 18))
        );
    }

    #[test]
    fn not_parse_if_abbr_less_than_3_chars() {
        assert_eq!(parse_abbreviation("AB(Abbreviation)"), None);
    }
}
