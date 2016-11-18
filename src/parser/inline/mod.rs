mod abbreviation;
mod bold;
mod citation;
mod code;
mod image;
mod italic;
mod link;
mod no_textile;
mod span;
mod strikethrough;
mod subscript;
mod superscript;
mod underlined;

use self::abbreviation::parse_abbreviation;
use self::bold::parse_bold_text;
use self::italic::parse_italic_text;
use self::strikethrough::parse_strikethrough_text;
use self::underlined::parse_underlined_text;
use self::subscript::parse_subscript_text;
use self::superscript::parse_superscript_text;
use self::code::parse_code;
use self::citation::parse_citation;
use self::image::parse_image;
use self::link::parse_link;
use self::span::parse_span;
use self::no_textile::parse_no_textile;
use parser::Inline;

pub fn parse_inline_elements(lines: &[&str]) -> Vec<Inline> {
    let mut tokens = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let mut tmp = String::new();
        let mut cur_char = 0;

        while cur_char < line.len() {
            match parse_inline_element(&line[cur_char..line.len()]) {
                Some((span, consumed_chars)) => {
                    if !tmp.is_empty() {
                        tokens.push(Inline::Text(tmp));
                    }
                    tokens.push(span);
                    tmp = String::new();
                    cur_char += consumed_chars;
                }
                None => {
                    tmp.push_str(&line[cur_char..cur_char + 1]);
                    cur_char += 1;
                }
            }
        }
        if !tmp.is_empty() {
            tokens.push(Inline::Text(tmp.clone()));
        }
        if idx < lines.len() - 1 && !lines[idx + 1].starts_with(" ") {
            tokens.push(Inline::Break);
        }
    }
    tokens
}

fn parse_inline_element(text: &str) -> Option<(Inline, usize)> {
    lazy_static! {
        static ref FNS: Vec<fn(&str) -> Option<(Inline, usize)>> = vec![
            parse_abbreviation,
            parse_bold_text,
            parse_citation,
            parse_code,
            parse_image,
            parse_italic_text,
            parse_link,
            parse_no_textile,
            parse_span,
            parse_strikethrough_text,
            parse_subscript_text,
            parse_superscript_text,
            parse_underlined_text,
        ];
    }

    for f in FNS.iter() {
        let res = f(text);
        if let Some(_) = res {
            return res;
        }
    }
    None
}
