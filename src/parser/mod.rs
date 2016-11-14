mod block;
mod inline;
mod utils;

use parser::block::parse_blocks;
use std::collections::HashMap;

pub type Attributes = Vec<Attribute>;

/// Block element, e.g. heading, paragraph or code block.
#[derive(Debug, PartialEq)]
pub enum Block {
    /// Heading, e.g. `h3. Some text`.
    Heading {
        attributes: Attributes,
        elements: Vec<Inline>,
        level: u8,
    },
    /// Paragraph of inline elements.
    Paragraph {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Block quotation, e.g. `bq. Some quote`.
    BlockQuotation {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Code block, e.g. `bc. print("Hello World")`.
    CodeBlock {
        attributes: Attributes,
        code: String,
    },
}

/// Inline element, e.g. bold text, link or image.
#[derive(Debug, PartialEq)]
pub enum Inline {
    /// Line break. Converts to `<br>` tag in HTML.
    Break,
    /// String with text.
    Text(String),
    /// Code, e.g. `@puts "Hello world!"@`.
    Code(String),
    /// Bold text, e.g. `*Text*` or `**Text**`.
    Bold {
        attributes: Attributes,
        elements: Vec<Inline>,
        tag_type: BoldTagType,
    },
    /// Italic text, e.g. `_Text_` or `__Text__`.
    Italic {
        attributes: Attributes,
        tag_type: ItalicTagType,
        elements: Vec<Inline>,
    },
    /// Strikethrough text, e.g. `-Text-`.
    Strikethrough {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Underlined text, e.g. `+Text+`.
    Underlined {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Subscript text, e.g. `~Text~`.
    Subscript {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Superscript text, e.g. `^Text^`.
    Superscript {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Citation, e.g. `??Some citation??`.
    Citation {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Abbreviation, e.g. `ABBR(Abbreviation)`.
    Abbreviation {
        abbr: String,
        transcript: String,
    },
    /// Link, e.g. `"Link":http://example.com`.
    Link {
        attributes: Attributes,
        description: Vec<Inline>,
        url: String,
    },
    /// Image, e.g. `!http://example.com/image.jpg(Image)!`.
    Image {
        attributes: Attributes,
        alt: String,
        url: String,
    },
}

/// Tag type for bold text, e.g. `<b>` or `<strong>`.
#[derive(Debug, PartialEq)]
pub enum BoldTagType {
    Strong,
    Bold,
}

/// Tag type for italic text, e.g. `<i>` or `<em>`.
#[derive(Debug, PartialEq)]
pub enum ItalicTagType {
    Emphasis,
    Italic,
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Align(String),
    Class(Vec<String>),
    Id(String),
    Language(String),
    Style(HashMap<String, String>)
}

/// Splits text into tokens. Returns vector of block elements.
///
/// # Example
///
/// ```rust
/// let text = "h1. _String with text_.";
/// textile::parser::parse(text); // [Heading { attributes: {}, level: 1, elements: [Italic([Text("String with text")], Emphasis), Text(".")] }]
/// ```
pub fn parse(text: &str) -> Vec<Block> {
    parse_blocks(text)
}
