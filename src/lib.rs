//! BDF font handler
//!
//! This crate allows you to read and write BDF fonts in Rust.
//!
//! # Example
//! This example will draw a given glyph in your terminal using the given font.
//!
//! ```no_run
//! use std::{env, process};
//!
//! let font = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
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

mod bitmap;
mod bounding_box;
mod direction;
mod entry;
mod error;
mod font;
mod glyph;
mod property;
mod reader;
mod writer;

pub use bitmap::Bitmap;
pub use bounding_box::BoundingBox;
pub use direction::Direction;
pub use entry::Entry;
pub use error::Error;
pub use font::*;
pub use glyph::Glyph;
pub use property::Property;
pub use reader::{open, read, Reader};
pub use writer::{save, write, Writer};
