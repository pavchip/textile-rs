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

#[cfg(test)]
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate url;

mod into_string;
pub mod parser;
mod renderer;

pub use renderer::*;
