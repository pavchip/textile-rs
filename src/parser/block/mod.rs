mod block_quotation;
mod header;
mod code;
mod paragraph;

use self::block_quotation::parse_block_quotation;
use self::header::parse_header;
use self::code::parse_code;
use self::paragraph::parse_paragraph;
use parser::Block;

pub fn parse_blocks(text: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut cur_line = 0;

    while cur_line < lines.len() {
        if let Some((block, consumed_lines)) = parse_block(&lines[cur_line..lines.len()]) {
            blocks.push(block);
            cur_line += consumed_lines;
        }
    }
    blocks
}

fn parse_block(lines: &[&str]) -> Option<(Block, usize)> {
    pipe_opt!(
        lines
        => parse_header
        => parse_code
        => parse_block_quotation
        => parse_paragraph
    )
}
