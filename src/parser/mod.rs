mod block;
mod inline;

use parser::block::parse_blocks;

/// Block element, e.g. header, paragraph or code block.
#[derive(Debug, PartialEq)]
pub enum Block {
    /// Header, e.g. `h3. Header`.
    Header {
        elements: Vec<Inline>,
        level: u8
    },
    /// Paragraph of inline elements.
    Paragraph(Vec<Inline>),
    /// Block quotation, e.g. `bq. Some quote`.
    BlockQuotation(Vec<Inline>),
    /// Code block, e.g. `bc. print("Hello World")`.
    Code(String),
}

/// Inline element, e.g. bold text, link or image.
#[derive(Debug, PartialEq)]
pub enum Inline {
    /// Line break. Converts to `<br>` tag in HTML.
    Break,
    /// String with text.
    Text(String),
    /// Bold text, e.g. `*Text*` or `**Text**`.
    Bold(Vec<Inline>, BoldTagType),
    /// Italic text, e.g. `_Text_` or `__Text__`.
    Italic(Vec<Inline>, ItalicTagType),
    /// Strikethrough text, e.g. `-Text-`.
    Strikethrough(Vec<Inline>),
    /// Underlined text, e.g. `+Text+`.
    Underlined(Vec<Inline>),
    /// Subscript text, e.g. `~Text~`.
    Subscript(Vec<Inline>),
    /// Superscript text, e.g. `^Text^`.
    Superscript(Vec<Inline>),
    /// Code, e.g. `@puts "Hello world!"@`.
    Code(String),
    /// Citation, e.g. `??Some citation??`.
    Citation(Vec<Inline>),
    /// Abbreviation, e.g. `ABBR(Abbreviation)`.
    Abbreviation {
        abbr: String,
        transcript: String
    },
    /// Link, e.g. `"Link":http://example.com`.
    Link {
        description: Vec<Inline>,
        url: String
    },
    /// Image, e.g. `!http://example.com/image.jpg(Image)!`.
    Image {
        alt: Option<String>,
        url: String
    }
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

/// Splits text into tokens. Returns vector of block elements.
///
/// # Example
///
/// ```rust
/// let text = "h1. _String with text_.";
/// textile::parser::parse(text); // [Header { elements: [Italic([Text("String with text")], Emphasis), Text(".")], level: 1 }]
/// ```
pub fn parse(text: &str) -> Vec<Block> {
    parse_blocks(text)
}
