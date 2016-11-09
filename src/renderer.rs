use parser::*;

/// Renders Textile string into HTML string.
///
/// # Example
///
/// ```rust
/// let html = textile::render("h2. *Header of level 2*");
/// assert_eq!(html, "<h2><strong>Header of level 2</strong></h2>".to_string());
/// ```
pub fn render(text: &str) -> String {
    render_blocks(parse(text))
}

fn render_attributes(attributes: &Attributes) -> String {
     if !attributes.is_empty() {
         let mut attrs = Vec::new();

         for (key, value) in attributes {
             attrs.push(format!("{}: {}", key, value));
         }

         format!(" style=\"{};\"", attrs.join("; "))
     } else {
         String::default()
     }
}

fn render_blocks(elements: Vec<Block>) -> String {
    let mut res = String::new();

    for element in &elements {
        let html = match *element {
            Block::Heading {ref attributes, level, ref elements} => format!("<h{0}{1}>{2}</h{0}>", level, render_attributes(attributes), render_inline_elements(elements)),
            Block::Paragraph {ref elements} => format!("<p>{}</p>", render_inline_elements(elements)),
            Block::BlockQuotation(ref elements) => format!("<blockquote>{}</blockquote>", render_inline_elements(elements)),
            Block::CodeBlock(ref code) => format!("<pre><code>{}</code></pre>", code)
        };
        res.push_str(&html);
    }
    res
}

fn render_inline_elements(elements: &[Inline]) -> String {
    let mut res = String::new();

    for element in elements {
        let html = match *element {
            Inline::Break => "<br>".to_string(),
            Inline::Text(ref text) => text.to_string(),
            Inline::Bold(ref elements, ref tag_type) => {
                let tag = match *tag_type {
                    BoldTagType::Strong => "strong",
                    BoldTagType::Bold => "b",
                };
                format!("<{0}>{1}</{0}>", tag, render_inline_elements(elements))
            },
            Inline::Italic(ref elements, ref tag_type) => {
                let tag = match *tag_type {
                    ItalicTagType::Emphasis => "em",
                    ItalicTagType::Italic => "i",
                };
                format!("<{0}>{1}</{0}>", tag, render_inline_elements(elements))
            },
            Inline::Strikethrough(ref elements) => format!("<del>{}</del>", render_inline_elements(elements)),
            Inline::Underlined(ref elements) => format!("<ins>{}</ins>", render_inline_elements(elements)),
            Inline::Subscript(ref elements) => format!("<sub>{}</sub>", render_inline_elements(elements)),
            Inline::Superscript(ref elements) => format!("<sup>{}</sup>", render_inline_elements(elements)),
            Inline::Code(ref text) => format!("<code>{}</code>", text),
            Inline::Citation(ref elements) => format!("<cite>{}</cite>", render_inline_elements(elements)),
            Inline::Abbreviation {ref abbr, ref transcript} => format!("<acronym title=\"{}\"><span>{}</span></acronym>", transcript, abbr),
            Inline::Link {ref description, ref url} => format!("<a href=\"{}\">{}</a>", url, render_inline_elements(description)),
            Inline::Image {ref url, alt: Some(ref text)} => format!("<img src=\"{}\" alt=\"{}\">", url, text),
            Inline::Image {ref url, alt: None} => format!("<img src=\"{}\">", url),
        };
        res.push_str(&html);
    }
    res
}
