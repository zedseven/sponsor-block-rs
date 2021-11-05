//! A Rust wrapper for the SponsorBlock API.
//!
//! ## Usage
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sponsor-block = "0.1"
//! ```
//!
//! ### Features
//! This crate only has one feature, which is on by default: `private_searches`.
//! This enables the use of private
//! [hash-based segment searching](https://wiki.sponsor.ajay.app/w/API_Docs#GET_.2Fapi.2FskipSegments.2F:sha256HashPrefix),
//! which significantly improves privacy at a slight bandwidth and performance
//! cost.
//!
//! ## Example
//! The following is a short example of how you might fetch the segments for a
//! video:
//!
//! ```rust,no_run
//! use sponsor_block::{AcceptedCategories, Client};
//!
//! const USER_ID: &str = "your local user id - it should be random and treated like a password";
//!
//! let client = Client::new(USER_ID);
//! let video_segments = client
//!     .fetch_segments("9Yhc6mmdJC4", AcceptedCategories::all())
//!     .await
//!     .ok();
//!
//! // Then do something with your video segments...
//! ```

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
