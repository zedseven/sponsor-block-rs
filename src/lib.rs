//! A Rust wrapper for the SponsorBlock API.
//!
//! ## Usage
//! Simply add it to your `Cargo.toml` as you would any other crate.
//!
//! ### Features
//! Default features:
//! - `private_searches`: This enables the use of private [hash-based segment searching](https://wiki.sponsor.ajay.app/w/API_Docs#GET_.2Fapi.2FskipSegments.2F:sha256HashPrefix),
//!   which significantly improves privacy at a slight bandwidth and performance
//!   cost.
//!
//!   You should almost certainly leave this on.
//! - `user`: The standard set of user functions.
//!
//! Optional features:
//! - `vip`: The set of functions for only VIP users.
//! - `gen_user_id`: A utility function for generating local user IDs for use
//!   with the service.
//!
//!   *Do not* use this every time you start up a client - prefer using a single
//!   saved ID for the same 'user'. This is for cases where you may want to
//!   generate new user IDs for users of your application, giving each user
//!   their own ID.
//!
//! ## Example
//! The following is a short example of how you might fetch the segments for a
//! video:
//!
//! ```rust,no_run
//! use sponsor_block::{AcceptedCategories, Client};
//!
//! // This should be random, treated like a password, and stored across sessions
//! const USER_ID: &str = "your local user id";
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
	clippy::dbg_macro,
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
#[cfg(feature = "gen_user_id")]
mod gen_user_id;
mod segment;
mod util;

// Public Exports
#[cfg(feature = "gen_user_id")]
pub use self::gen_user_id::*;
pub use self::{client::*, error::*, segment::*};
