//! Parser module for Textile language.

mod attributes;
mod block;
mod inline;
mod patterns;

use into_string::*;
use parser::block::parse_blocks;
use std::collections::HashMap;

/// Vector of Textile attributes, e.g. classes, ID's or CSS styles.
pub type Attributes = Vec<Attribute>;

/// Block element, e.g. heading, paragraph or code block.
#[derive(Debug, PartialEq)]
pub enum Block {
    /// Block quotation, e.g. `bq. Some quote`.
    BlockQuotation {
        attributes: Attributes,
        elements: Vec<Block>,
    },
    /// Code block, e.g. `bc. print("Hello World")`.
    CodeBlock {
        attributes: Attributes,
        code: String,
    },
    /// Comment block.
    Comment(Vec<String>),
    /// Heading, e.g. `h3. Some text`.
    Heading {
        attributes: Attributes,
        elements: Vec<Inline>,
        level: u8,
    },
    /// Disable Textile formatting in a block of text.
    NoTextileBlock(Vec<String>),
    /// Paragraph, e.g. `p. Some text` or `Some text`.
    Paragraph {
        attributes: Attributes,
        elements: Vec<Inline>,
        starts_with_p: bool,
    },
}

/// Inline element, e.g. bold text, link or image.
#[derive(Debug, PartialEq)]
pub enum Inline {
    /// Abbreviation, e.g. `ABBR(Abbreviation)`.
    Abbreviation {
        abbr: String,
        transcript: String,
    },
    /// Bold text, e.g. `*Text*` or `**Text**`.
    Bold {
        attributes: Attributes,
        elements: Vec<Inline>,
        tag_type: BoldTagType,
    },
    /// Line break. Converts to `<br>` tag in HTML.
    Break,
    /// Citation, e.g. `??Some citation??`.
    Citation {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Code, e.g. `@puts "Hello world!"@`.
    Code(String),
    /// Image, e.g. `!http://example.com/image.jpg(Image)!`.
    Image {
        attributes: Attributes,
        alt: String,
        url: String,
    },
    /// Italic text, e.g. `_Text_` or `__Text__`.
    Italic {
        attributes: Attributes,
        tag_type: ItalicTagType,
        elements: Vec<Inline>,
    },
    /// Link, e.g. `"Link":http://example.com`.
    Link {
        attributes: Attributes,
        description: Vec<Inline>,
        url: String,
    },
    /// Span element, e.g. `%Span text%`.
    Span {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
    /// Strikethrough text, e.g. `-Text-`.
    Strikethrough {
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
    /// String with text.
    Text(String),
    /// Underlined text, e.g. `+Text+`.
    Underlined {
        attributes: Attributes,
        elements: Vec<Inline>,
    },
}

/// Tag type for bold text.
#[derive(Debug, PartialEq)]
pub enum BoldTagType {
    /// Converts into HTML `<b>` tag.
    Bold,
    /// Converts into HTML `<strong>` tag.
    Strong,
}

/// Tag type for italic text.
#[derive(Debug, PartialEq)]
pub enum ItalicTagType {
    /// Converts into HTML `<em>` tag.
    Emphasis,
    /// Converts into HTML `<i>` tag.
    Italic,
}

/// Textile attribute.
#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    /// Image align. Convertes into HTML `align` attribute.
    Align(String),
    /// Vector of HTML classes. Converts into HTML `class` attribute.
    Class(Vec<String>),
    /// Converts into HTML `id` attribute.
    Id(String),
    /// Converts into HTML `lang` attribute.
    Language(String),
    /// CSS styles. Converts into HTML `style` attribute.
    Style(HashMap<String, String>)
}

/// Splits text into tokens. Accepts `&str`, `String` or `Path` data type. Returns vector of block elements.
///
/// # Example
///
/// ```rust
/// use textile::parser::*;
///
/// let text = "h1. _String with text_.";
/// assert_eq!(parse(text), vec![
///     Block::Heading {
///         attributes: vec![],
///         elements: vec![
///             Inline::Italic {
///                 attributes: vec![],
///                 tag_type: ItalicTagType::Emphasis,
///                 elements: vec![
///                     Inline::Text("String with text".to_string())
///                 ]
///             },
///             Inline::Text(".".to_string())
///         ],
///         level: 1
///     }
/// ]);
/// ```
pub fn parse<S: IntoString>(text: S) -> Vec<Block> {
    parse_blocks(&text.into_string().lines().collect::<Vec<&str>>())
}
