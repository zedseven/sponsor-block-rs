//! A Rust wrapper for the SponsorBlock API.

// Linting rules
#![warn(
	clippy::complexity,
	clippy::correctness,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::pedantic,
	clippy::filetype_is_file,
	clippy::str_to_string,
	missing_docs,
	rustdoc::missing_crate_level_docs
)]
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::doc_markdown,
	clippy::module_name_repetitions,
	clippy::similar_names,
	clippy::too_many_lines,
	clippy::unnecessary_wraps,
	dead_code,
	unused_macros
)]

// Modules
mod api;
mod client;
mod error;
mod segment;
mod util;

// Public Exports
pub use self::{client::*, error::*, segment::*};
