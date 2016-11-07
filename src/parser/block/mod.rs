mod block_quotation;
mod header;
mod code;
mod paragraph;

use self::block_quotation::parse_block_quotation;
use self::header::parse_header;
use self::code::parse_code;
use self::paragraph::parse_paragraph;
use parser::Block;
use regex::Regex;

pub fn parse_blocks(text: &str) -> Vec<Block> {
    let paragraph_pattern = Regex::new("\r{2,}|\n{2,}|(?:\r\n){2,}").unwrap();
    let paragraphs = paragraph_pattern.split(&text.trim()).collect::<Vec<&str>>();
    let mut tokens = Vec::new();

    for paragraph in &paragraphs {
        match parse_block(paragraph) {
            Some(block) => {
                tokens.push(block);
            }
            None => {
                tokens.push(parse_paragraph(paragraph));
            }
        }
    }
    tokens
}

fn parse_block(text: &str) -> Option<Block> {
    pipe_opt!(
        text
        => parse_header
        => parse_code
        => parse_block_quotation
    )
}
