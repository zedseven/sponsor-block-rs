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
	to_url_array_conditional(slice, |_| true)
}

pub(crate) fn to_url_array_conditional<S, P>(slice: &[S], predicate: P) -> String
where
	S: AsRef<str>,
	P: Fn(&S) -> bool,
{
	to_url_array_conditional_convert(slice, predicate, |s| s)
}

pub(crate) fn to_url_array_conditional_convert<'e, E, S, P, C>(
	slice: &'e [E],
	predicate: P,
	convert: C,
) -> String
where
	S: AsRef<str>,
	P: Fn(&'e E) -> bool,
	C: Fn(&'e E) -> S,
{
	let mut result = String::from('[');

	let mut pushed_already = false;
	for s in slice.iter() {
		if !predicate(s) {
			continue;
		}

		if pushed_already {
			result.push(',');
		}

		result.push('"');
		result.push_str(convert(s).as_ref());
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

/// For all deserialization helper functions.
pub(crate) mod de {
	// Uses
	use core::time::Duration;
	use std::{collections::HashMap, hash::Hash, result::Result as StdResult};

	use serde::{Deserialize, Deserializer};

	/// A custom deserializer that maps an integer to a boolean value based on
	/// whether it equals `0`.
	///
	/// Many API fields that are boolean in nature use an integer
	/// representation, which is why.
	pub(crate) fn bool_from_integer_str<'de, D>(deserializer: D) -> StdResult<bool, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = isize::deserialize(deserializer)?;
		Ok(raw != 0)
	}

	/// A custom deserializer that maps an `f32` value to `None` if it's `0.0`.
	///
	/// This is because `f32` segments submitted before a field was added
	/// default to `0.0`.
	pub(crate) fn none_on_0_0_from_str<'de, D>(deserializer: D) -> StdResult<Option<f32>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = f32::deserialize(deserializer)?;
		Ok(if raw == 0.0 { None } else { Some(raw) })
	}

	/// A custom deserializer that maps a [`HashMap`]'s keys using an arbitrary
	/// function.
	///
	/// Failed conversions are silently dropped. This is so an existing version
	/// of the library can remain functional if new keys are added to the API.
	///
	/// This cannot be used directly with [`serde`] - a wrapper deserializer is
	/// required for each type mapping, specifying the conversion function to
	/// use.
	pub(crate) fn map_hashmap_key_from_str<'de, D, T, O, C, E>(
		deserializer: D,
		convert_func: C,
	) -> StdResult<HashMap<T, O>, D::Error>
	where
		D: Deserializer<'de>,
		T: Hash + Eq,
		O: Deserialize<'de>,
		C: Fn(&str) -> StdResult<T, E>,
	{
		let raw: HashMap<&str, O> = HashMap::deserialize(deserializer)?;
		Ok(raw
			.into_iter()
			.flat_map(|e| {
				let convert_result: StdResult<(T, O), E> = Ok((convert_func(e.0)?, e.1));
				convert_result
			})
			.collect())
	}

	/// A custom deserializer that converts an amount of milliseconds in string
	/// format to a [`Duration`].
	pub(crate) fn duration_from_millis_str<'de, D>(deserializer: D) -> StdResult<Duration, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = u64::deserialize(deserializer)?;
		Ok(Duration::from_millis(raw))
	}

	/// A custom deserializer that converts an amount of seconds in string
	/// format to a [`Duration`].
	pub(crate) fn duration_from_seconds_str<'de, D>(
		deserializer: D,
	) -> StdResult<Duration, D::Error>
	where
		D: Deserializer<'de>,
	{
		let raw = f32::deserialize(deserializer)?;
		Ok(Duration::from_secs_f32(raw))
	}
}
