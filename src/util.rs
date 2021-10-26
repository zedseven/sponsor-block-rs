//! Utility functions that don't necessarily fit into a neat category but assist
//! with the process.

// Uses
use std::fmt::Write;

use reqwest::Response;

use crate::error::{Result, SponsorBlockError};

/// Parses the [`Response`] and categorizes errors depending on their source.
pub(crate) async fn get_response_text(response: Response) -> Result<String> {
	let status = response.status();
	if status.is_success() {
		Ok(response.text().await?)
	} else if status.is_server_error() {
		Err(SponsorBlockError::HttpApi(status.as_u16()))
	} else if status.is_client_error() {
		Err(SponsorBlockError::HttpClient(status.as_u16()))
	} else {
		Err(SponsorBlockError::HttpUnknown(status.as_u16()))
	}
}

pub(crate) fn to_url_array<S: AsRef<str>>(slice: &[S]) -> String {
	let mut result = String::from('[');

	let mut pushed_already = false;
	for s in slice.iter() {
		if pushed_already {
			result.push(',');
		}

		result.push('"');
		result.push_str(s.as_ref());
		result.push('"');

		pushed_already = true;
	}

	result.push(']');
	result
}

#[cfg(feature = "private_searches")]
pub(crate) fn bytes_to_hex_string(bytes: &[u8]) -> String {
	let mut result = String::with_capacity(bytes.len() * 2);
	for byte in bytes {
		write!(result, "{:02x}", byte).expect("unable to write byte to string");
	}
	result
}
