mod block_quotation;
mod code_block;
mod comment;
mod heading;
mod no_textile;
mod paragraph;
mod pre;

use parser::{BlockElements, Block};
use self::block_quotation::parse_block_quotation;
use self::code_block::parse_code_block;
use self::comment::parse_comment;
use self::heading::parse_heading;
use self::no_textile::parse_no_textile;
use self::paragraph::parse_paragraph;
use self::pre::parse_pre_block;

pub fn parse_blocks(lines: &[&str]) -> BlockElements {
    let mut blocks = Vec::new();
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
    lazy_static! {
        static ref FNS: Vec<fn(&[&str]) -> Option<(Block, usize)>> = vec![
            parse_block_quotation,
            parse_code_block,
            parse_comment,
            parse_heading,
            parse_no_textile,
            parse_pre_block,
            parse_paragraph,
        ];
    }
    // Get index of non-empty string.
    let pos = lines.iter().position(|el| !el.is_empty());
    let lines = match pos {
        Some(value) => &lines[value..],
        None => lines,
    };

    for f in FNS.iter() {
        let res = f(lines);
        if let Some(_) = res {
            return res;
        }
    }
    None
}
