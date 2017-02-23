use into_string::*;
use parser::*;
use std::iter;

/// Options for rendering Textile markup language.
pub struct RenderOptions {
    pub compress: bool,
    pub indent: u8,
}

impl Default for RenderOptions {
    fn default() -> RenderOptions {
        RenderOptions {
            compress: false,
            indent: 2,
        }
    }
}

/// Renders Textile string into HTML string with default options.
/// Accepts `&str`, `String` or `Path` data type.
///
/// # Example
///
/// ```rust
/// let html = textile::render("h2. *Heading of level 2*");
/// assert_eq!(html, "<h2><strong>Heading of level 2</strong></h2>".to_string());
/// ```
pub fn render<S: IntoString>(text: S) -> String {
    render_blocks(&parse(text.into_string()), &RenderOptions::default())
}

/// Renders Textile string into HTML string with specified options.
/// Accepts `&str`, `String` or `Path` data type.
///
/// # Example
///
/// ```rust
/// let html = textile::render_with("h2. *Heading of level 2*", textile::RenderOptions::default());
/// assert_eq!(html, "<h2><strong>Heading of level 2</strong></h2>".to_string());
/// ```
pub fn render_with<S: IntoString>(text: S, options: RenderOptions) -> String {
    render_blocks(&parse(text.into_string()), &options)
}

fn render_attributes(attributes: &Attributes) -> String {
    if !attributes.is_empty() {
        let mut res = Vec::new();

        for (key, value) in attributes {
            res.push(format!("{}=\"{}\"", key, value));
        }
        format!(" {}", res.join(" "))
    } else {
        String::default()
    }
}

fn render_blocks(elements: &[Block], options: &RenderOptions) -> String {
    let mut res = String::new();

    for (idx, element) in elements.iter().enumerate() {
        if idx > 0 && idx < elements.len() && !options.compress {
            res.push_str("\n");
        }
        res.push_str(&*render_block(element, options));
    }
    res
}

fn render_block(element: &Block, options: &RenderOptions) -> String {
    match *element {
        Block::BlockQuotation { ref attributes, ref cite, ref elements } => {
            let mut attrs = attributes.clone();

            if !cite.is_empty() {
                attrs.insert("cite".to_string(), cite.to_string());
            }

            if !options.compress {
                let mut res = String::new();
                let spaces: String = iter::repeat(" ").take(options.indent as usize).collect();

                for element in elements {
                    res.push_str(&*format!("\n{}{}", spaces, render_block(element, options)));
                }
                format!("<blockquote{}>{}\n</blockquote>",
                        render_attributes(&attrs),
                        res)
            } else {
                format!("<blockquote{}>{}</blockquote>",
                        render_attributes(&attrs),
                        render_blocks(elements, options))
            }
        }
        Block::CodeBlock { ref attributes, ref code } => {
            format!("<pre{}><code>{}</code></pre>",
                    render_attributes(attributes),
                    code)
        }
        Block::Heading { ref attributes, level, ref elements } => {
            format!("<h{0}{1}>{2}</h{0}>",
                    level,
                    render_attributes(attributes),
                    render_inline_elements(elements, options))
        }
        Block::NoTextileBlock(ref strings) => strings.join("\n"),
        Block::OrderedList { ref attributes, ref elements, level, start } => {
            let mut res = String::new();
            let mut attrs = attributes.clone();
            let list_item_indent: String = iter::repeat(" ").take((options.indent * (level + 1)) as usize).collect();
            let list_indent: String = iter::repeat(" ").take((options.indent * level) as usize).collect();

            if let Some(val) = start {
                attrs.insert("start".to_string(), val.to_string());
            }

            for element in elements {
                let html = match *element {
                    ListElement::ListItem { ref attributes, ref elements } => {
                        format!("\n{}<li{}>{}</li>",
                                list_item_indent,
                                render_attributes(attributes),
                                render_inline_elements(elements, options))
                    },
                    ListElement::List(ref list) => {
                        format!("\n{}", render_block(list, options))
                    }
                };
                res.push_str(&html);
            }
            format!("{0}<ol{1}>{2}\n{0}</ol>",
                    list_indent,
                    render_attributes(&attrs),
                    res)
        },
        Block::Paragraph { ref attributes, ref elements, .. } => {
            format!("<p{}>{}</p>",
                    render_attributes(attributes),
                    render_inline_elements(elements, options))
        },
        Block::Pre {ref attributes, ref lines} => {
            format!("<pre{}>{}</pre>",
                    render_attributes(attributes),
                    lines.join("\n"))
        },
        Block::UnorderedList { ref attributes, ref elements, level } => {
            let mut res = String::new();
            let list_item_indent: String = iter::repeat(" ").take((options.indent * (level + 1)) as usize).collect();
            let list_indent: String = iter::repeat(" ").take((options.indent * level) as usize).collect();

            for element in elements {
                let html = match *element {
                    ListElement::ListItem { ref attributes, ref elements } => {
                        format!("\n{}<li{}>{}</li>",
                                list_item_indent,
                                render_attributes(attributes),
                                render_inline_elements(elements, options))
                    },
                    ListElement::List(ref list) => {
                        format!("\n{}", render_block(list, options))
                    }
                };
                res.push_str(&html);
            }
            format!("{0}<ul{1}>{2}\n{0}</ul>",
                    list_indent,
                    render_attributes(attributes),
                    res)
        },
        _ => "".to_string(),
    }
}

fn render_inline_elements(elements: &[Inline], options: &RenderOptions) -> String {
    let mut res = String::new();

    for element in elements {
        let html = match *element {
            Inline::Abbreviation { ref abbr, ref transcript } => {
                format!("<acronym title=\"{}\"><span>{}</span></acronym>",
                        transcript,
                        abbr)
            }
            Inline::Bold { ref attributes, ref elements, ref tag_type } => {
                let tag = match *tag_type {
                    BoldTagType::Strong => "strong",
                    BoldTagType::Bold => "b",
                };
                format!("<{0}{1}>{2}</{0}>",
                        tag,
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Break => "<br>".to_string(),
            Inline::Citation { ref attributes, ref elements } => {
                format!("<cite{}>{}</cite>",
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Code(ref text) => format!("<code>{}</code>", text),
            Inline::Image { ref attributes, ref align, ref alt, ref href, ref src } => {
                let mut attrs = attributes.clone();
                attrs.insert("src".to_string(), src.to_string());

                if !align.is_empty() {
                    attrs.insert("align".to_string(), align.to_string());
                }

                if !alt.is_empty() {
                    attrs.insert("alt".to_string(), alt.to_string());
                    attrs.insert("title".to_string(), alt.to_string());
                }

                let img = format!("<img{}>", render_attributes(&attrs));

                if !href.is_empty() {
                    format!("<a href=\"{}\">{}</a>", href, img)
                } else {
                    img
                }
            }
            Inline::Italic { ref attributes, ref elements, ref tag_type } => {
                let tag = match *tag_type {
                    ItalicTagType::Emphasis => "em",
                    ItalicTagType::Italic => "i",
                };
                format!("<{0}{1}>{2}</{0}>",
                        tag,
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Link { ref attributes, ref elements, ref href, ref title } => {
                let mut attrs = attributes.clone();
                attrs.insert("href".to_string(), href.to_string());

                if !title.is_empty() {
                    attrs.insert("title".to_string(), title.to_string());
                }

                format!("<a href=\"{}\"{}>{}</a>",
                        href,
                        render_attributes(&attrs),
                        render_inline_elements(elements, options))
            }
            Inline::Span { ref attributes, ref elements } => {
                format!("<span{}>{}</span>",
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Strikethrough { ref attributes, ref elements } => {
                format!("<del{}>{}</del>",
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Subscript { ref attributes, ref elements } => {
                format!("<sub{}>{}</sub>",
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Superscript { ref attributes, ref elements } => {
                format!("<sup{}>{}</sup>",
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
            Inline::Text(ref text) => text.to_string(),
            Inline::Underlined { ref attributes, ref elements } => {
                format!("<ins{}>{}</ins>",
                        render_attributes(attributes),
                        render_inline_elements(elements, options))
            }
        };
        res.push_str(&html);
    }
    res
}
