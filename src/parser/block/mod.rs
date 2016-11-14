mod block_quotation;
mod code_block;
mod heading;
mod paragraph;

use self::block_quotation::parse_block_quotation;
use self::code_block::parse_code_block;
use self::heading::parse_heading;
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

pub fn parse_block(lines: &[&str]) -> Option<(Block, usize)> {
    pipe_opt!(
        lines
        => parse_block_quotation
        => parse_code_block
        => parse_heading
        => parse_paragraph
    )
}
