use parser::*;

/// Renders Textile string into HTML string.
///
/// # Example
///
/// ```rust
/// let html = textile::render("h2. *Heading of level 2*");
/// assert_eq!(html, "<h2><strong>Heading of level 2</strong></h2>".to_string());
/// ```
pub fn render(text: &str) -> String {
    render_blocks(parse(text))
}

fn render_attributes(attributes: &[Attribute]) -> String {
     if !attributes.is_empty() {
         let mut res = Vec::new();

         for attribute in attributes {
             let attr = match *attribute {
                 Attribute::Align(ref align) => format!("align=\"{}\"", align),
                 Attribute::Class(ref list) => format!("class=\"{}\"", list.join(" ")),
                 Attribute::Id(ref id) => format!("id=\"{}\"", id),
                 Attribute::Language(ref lang) => format!("lang=\"{}\"", lang),
                 Attribute::Style(ref props) => {
                     let mut res = Vec::new();

                     for (key, value) in props {
                         res.push(format!("{}: {}", key, value))
                     }

                     format!("style=\"{}\"", res.join("; "))
                 }
             };
             res.push(attr);
         }
         format!(" {}", res.join(" "))
     } else {
         String::default()
     }
}

fn render_blocks(elements: Vec<Block>) -> String {
    let mut res = String::new();

    for element in &elements {
        let html = match *element {
            Block::Heading {ref attributes, level, ref elements} => format!("<h{0}{1}>{2}</h{0}>", level, render_attributes(attributes), render_inline_elements(elements)),
            Block::Paragraph {ref attributes, ref elements} => format!("<p{}>{}</p>", render_attributes(attributes), render_inline_elements(elements)),
            Block::BlockQuotation {ref attributes, ref elements} => format!("<blockquote{}>{}</blockquote>", render_attributes(attributes), render_inline_elements(elements)),
            Block::CodeBlock {ref attributes, ref code} => format!("<pre{}><code>{}</code></pre>", render_attributes(attributes), code),
        };
        res.push_str(&*html);
    }
    res
}

fn render_inline_elements(elements: &[Inline]) -> String {
    let mut res = String::new();

    for element in elements {
        let html = match *element {
            Inline::Break => "<br>".to_string(),
            Inline::Text(ref text) => text.to_string(),
            Inline::Code(ref text) => format!("<code>{}</code>", text),
            Inline::Bold {ref attributes, ref elements, ref tag_type} => {
                let tag = match *tag_type {
                    BoldTagType::Strong => "strong",
                    BoldTagType::Bold => "b",
                };
                format!("<{0}{1}>{2}</{0}>", tag, render_attributes(attributes), render_inline_elements(elements))
            },
            Inline::Italic {ref attributes, ref elements, ref tag_type} => {
                let tag = match *tag_type {
                    ItalicTagType::Emphasis => "em",
                    ItalicTagType::Italic => "i",
                };
                format!("<{0}{1}>{2}</{0}>", tag, render_attributes(attributes), render_inline_elements(elements))
            },
            Inline::Strikethrough {ref attributes, ref elements} => format!("<del{}>{}</del>", render_attributes(attributes), render_inline_elements(elements)),
            Inline::Underlined {ref attributes, ref elements} => format!("<ins{}>{}</ins>", render_attributes(attributes), render_inline_elements(elements)),
            Inline::Subscript {ref attributes, ref elements} => format!("<sub{}>{}</sub>", render_attributes(attributes), render_inline_elements(elements)),
            Inline::Superscript {ref attributes, ref elements} => format!("<sup{}>{}</sup>", render_attributes(attributes), render_inline_elements(elements)),
            Inline::Citation {ref attributes, ref elements} => format!("<cite{}>{}</cite>", render_attributes(attributes), render_inline_elements(elements)),
            Inline::Abbreviation {ref abbr, ref transcript} => format!("<acronym title=\"{}\"><span>{}</span></acronym>", transcript, abbr),
            Inline::Link {ref attributes, ref description, ref url} => format!("<a href=\"{}\"{}>{}</a>", url, render_attributes(attributes), render_inline_elements(description)),
            Inline::Image {ref attributes, ref url, alt: Some(ref text)} => format!("<img src=\"{}\" alt=\"{}\"{}>", url, text, render_attributes(attributes)),
            Inline::Image {ref attributes, ref url, alt: None} => format!("<img src=\"{}\"{}>", url, render_attributes(attributes)),
        };
        res.push_str(&html);
    }
    res
}
