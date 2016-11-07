mod abbreviation;
mod bold;
mod citation;
mod code;
mod image;
mod italic;
mod link;
mod strikethrough;
mod subscript;
mod superscript;
mod underlined;

use parser::inline::abbreviation::parse_abbreviation;
use parser::inline::bold::parse_bold_text;
use parser::inline::italic::parse_italic_text;
use parser::inline::strikethrough::parse_strikethrough_text;
use parser::inline::underlined::parse_underlined_text;
use parser::inline::subscript::parse_subscript_text;
use parser::inline::superscript::parse_superscript_text;
use parser::inline::code::parse_code;
use parser::inline::citation::parse_citation;
use parser::inline::image::parse_image;
use parser::inline::link::parse_link;
use parser::Inline;

pub fn parse_inline_elements(text: &str) -> Vec<Inline> {
    let lines = text.lines().collect::<Vec<&str>>();
    let mut tokens = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let mut tmp = String::new();
        let mut pos = 0;

        while pos < line.len() {
            match parse_inline_element(&line[pos..line.len()]) {
                Some((span, consumed_chars)) => {
                    if !tmp.is_empty() {
                        tokens.push(Inline::Text(tmp));
                    }
                    tokens.push(span);
                    tmp = String::new();
                    pos += consumed_chars;
                }
                None => {
                    tmp.push_str(&line[pos..pos + 1]);
                    pos += 1;
                }
            }
        }
        if !tmp.is_empty() {
            tokens.push(Inline::Text(tmp));
        }
        if idx < lines.len() - 1 {
            tokens.push(Inline::Break);
        }
    }
    tokens
}

fn parse_inline_element(text: &str) -> Option<(Inline, usize)> {
    pipe_opt!(
        text
        => parse_bold_text
        => parse_italic_text
        => parse_strikethrough_text
        => parse_underlined_text
        => parse_subscript_text
        => parse_superscript_text
        => parse_abbreviation
        => parse_code
        => parse_citation
        => parse_image
        => parse_link
    )
}
