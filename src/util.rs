//! Utility functions that don't necessarily fit into a neat category but assist
//! with the process.

// Uses
use std::{collections::HashMap, fmt::Write, hash::Hash};

use reqwest::Response;
use serde::{Deserialize, Deserializer};

use crate::error::{SponsorBlockError, SponsorBlockResult};

/// Parses the [`Response`] and categorizes errors depending on their source.
pub(crate) async fn get_response_text(response: Response) -> SponsorBlockResult<String> {
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

/// A custom deserializer that maps a [`HashMap`]'s keys using an arbitrary
/// function.
///
/// Failed conversions are silently dropped. This is so an existing version of
/// the library can remain functional if new keys are added to the API.
///
/// This cannot be used directly with [`serde`] - a wrapper deserializer is
/// required for each type mapping, specifying the conversion function to use.
pub(crate) fn map_hashmap_key_from_str<'de, D, T, O, F, E>(
	deserializer: D,
	convert_func: F,
) -> Result<HashMap<T, O>, D::Error>
where
	D: Deserializer<'de>,
	T: Hash + Eq,
	O: Deserialize<'de>,
	F: Fn(&str) -> Result<T, E>,
{
	let raw: HashMap<&str, O> = HashMap::deserialize(deserializer)?;
	Ok(raw
		.into_iter()
		.flat_map(|e| {
			let convert_result: Result<(T, O), E> = Ok((convert_func(e.0)?, e.1));
			convert_result
		})
		.collect())
}
