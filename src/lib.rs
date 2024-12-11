//! BDF font handler
//!
//! This crate allows you to read and write BDF fonts in Rust.
//!
//! # Example
//! This example will draw a given glyph in your terminal using the given font.
//!
//! ```no_run
//! use std::{env, process};
//! use bdf::Font;
//!
//! let font = Font::open(env::args().nth(1).expect("missing font file")).unwrap();
//! let codepoint = char::from_u32(
//!     env::args()
//!         .nth(2)
//!         .expect("missing codepoint")
//!         .parse()
//!         .unwrap(),
//! )
//! .expect("invalid codepoint");
//! let glyph = font.glyphs().get(&codepoint).unwrap_or_else(|| process::exit(1));
//!
//! for y in 0..glyph.height() {
//!     for x in 0..glyph.width() {
//!         if glyph.get(x, y) {
//!             print!("██");
//!         } else {
//!             print!("  ");
//!         }
//!     }
//!     print!("\n");
//! }
//! ```

#![warn(missing_docs)]
#![allow(clippy::empty_docs)]

#[cfg(test)]
mod tests;

mod error;
mod font;
mod glyph;
mod reader;
mod writer;

pub use error::Error;
pub use font::{Bitmap, Direction, Entry, Font, Property, Size};
pub use glyph::{BoundingBox, Glyph};
pub use reader::Reader;
pub use writer::Writer;
