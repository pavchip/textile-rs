# Textile

Native Rust crate for parsing and rendering into HTML the Textile markup language.

## Installation

Put this into `Cargo.toml`:

```toml
[dependencies]
textile = "0.1.0"
```

## Usage

```rust
extern crate textile;

let html = textile::render("h1. *Header*");
assert_eq!(html, "<h1><strong>Header</strong></h1>".to_string());
```

## Development Status
+ [ ] Block elements
  + [x] Block quotation
  + [x] Code block
  + [x] Header
  + [ ] Raw HTML
  + [ ] Comments
  + [ ] Pre-formatted text
  + [ ] No Textile formatting
+ [x] Inline elements
  + [x] Bold text
  + [x] Italic text
  + [x] Subscript text
  + [x] Superscript text
  + [x] Strikethrough text
  + [x] Underlined text
  + [x] Citation
  + [x] Inline code
  + [x] Link
  + [x] Image
  + [x] Abbreviations
+ [ ] Lists
  + [ ] Bulleted list
  + [ ] Numbered list
  + [ ] Definition list
  + [ ] Footnotes
+ [ ] Tables
+ [ ] Attributes in elements
+ [ ] CSS styles, classes and ID's
+ [ ] Span with styles
+ [ ] Characters escaping
