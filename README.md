# Textile

[![Build Status](https://travis-ci.org/pavchip/textile-rs.svg?branch=master)](https://travis-ci.org/pavchip/textile-rs)
[![Crate version](https://img.shields.io/crates/v/textile.svg)](https://crates.io/crates/textile)

Native Rust crate for parsing and rendering into HTML the Textile markup language.

[Documentation](https://docs.rs/textile/0.2.1/textile/)

## Installation

Put this into `Cargo.toml`:

```toml
[dependencies]
textile = "*"
```

## Usage

```rust
extern crate textile;

let html = textile::render("h1. *Textile markup language*");
assert_eq!(html, "<h1><strong>Textile markup language</strong></h1>".to_string());
```

## Development Status
+ [ ] Block elements
  + [x] Block quotation
  + [x] Code block
  + [x] Heading
  + [ ] Raw HTML
  + [x] Comments
  + [x] Pre-formatted text
  + [x] Disable Textile formatting block element
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
  + [x] Span element
  + [x] Disable Textile formatting inline element
+ [ ] Lists
  + [x] Bulleted list
  + [x] Numbered list
  + [ ] Definition list
  + [ ] Footnotes
+ [ ] Tables
+ [x] Attributes in elements
+ [x] CSS properties, classes and ID's in elements
+ [x] Unicode support
