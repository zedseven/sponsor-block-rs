// Uses
use core::time::Duration;

use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Deserialize;
use serde_json::from_str as from_json_str;

use crate::{
	error::Result,
	util::{
		de::{duration_from_millis_str, duration_from_seconds_str},
		get_response_text,
	},
	Client,
};

/// The results of an API status request.
#[derive(Deserialize, Debug)]
#[non_exhaustive]
#[serde(default)]
pub struct ApiStatus {
	/// The server process uptime.
	#[serde(deserialize_with = "duration_from_seconds_str")]
	pub uptime: Duration,
	/// The SHA-1 hash of the most recent commit the server is running.
	pub commit: String,
	/// The version of the database.
	#[serde(rename = "db")]
	pub db_version: u32,
	/// The date and time when the request was received.
	#[serde(rename = "startTime", with = "ts_milliseconds")]
	pub request_start_time: DateTime<Utc>,
	/// The time that it took the API to send it's reply.
	#[serde(rename = "processTime", deserialize_with = "duration_from_millis_str")]
	pub request_time_taken: Duration,
	/// The load average for the server. The first entry is the average for 5
	/// minutes, and the second is for 15 minutes.
	///
	/// If you want more information about the source of this information, visit
	/// <https://github.com/ajayyy/SponsorBlockServer/blob/06af78c770b82722be8b03d2b1b82eb7409f675b/src/routes/getStatus.ts#L18>
	#[serde(rename = "loadavg")]
	pub load_average: [f32; 2],
}

impl Default for ApiStatus {
	fn default() -> Self {
		Self {
			uptime: Duration::default(),
			commit: String::default(),
			db_version: u32::default(),
			request_start_time: Utc::now(),
			request_time_taken: Duration::default(),
			load_average: Default::default(),
		}
	}
}

// Function Implementation
impl Client {
	/// Fetches the API status.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_api_status(&self) -> Result<ApiStatus> {
		// Function Constants
		const API_ENDPOINT: &str = "/status";

		// Build the request
		let request = self.http.get(format!("{}{}", &self.base_url, API_ENDPOINT));

		// Send the request
		let response = get_response_text(request.send().await?).await?;

		// Parse the response
		Ok(from_json_str::<ApiStatus>(response.as_str())?)
	}
}
