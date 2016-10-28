mod block_quotation;
mod header;
mod code;

use parser::block::block_quotation::parse_block_quotation;
use parser::block::header::parse_header;
use parser::block::code::parse_code;
use parser::inline::parse_inline_elements;
use parser::Block;
use regex::Regex;

pub fn parse_blocks(text: &str) -> Vec<Block> {
    let paragraph_pattern = Regex::new("\r{2,}|\n{2,}|(?:\r\n){2,}").unwrap();
    let paragraphs = paragraph_pattern.split(&text).collect::<Vec<&str>>();
    let mut tokens = Vec::new();

    for paragraph in &paragraphs {
        match parse_block(paragraph) {
            Some(block) => {
                tokens.push(block);
            }
            None => {
                tokens.push(Block::Paragraph(parse_inline_elements(paragraph)));
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
