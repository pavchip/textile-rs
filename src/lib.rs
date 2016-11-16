//! Native Rust crate for parsing and rendering into HTML the Textile markup language.
//!
//! # Installation
//!
//! Put this into `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! textile = "0.1.0"
//! ```
//!
//! # Usage
//!
//! ```rust
//! extern crate textile;
//!
//! let html = textile::render("h1. *Textile markup language*");
//! assert_eq!(html, "<h1><strong>Textile markup language</strong></h1>".to_string());
//! ```

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pipeline;
extern crate regex;

pub mod parser;
mod renderer;

pub use renderer::*;
