// Uses
use serde::Deserialize;
use serde_json::from_str as from_json_str;

use crate::{error::SponsorBlockResult, util::get_response_text, Client};

/// The results of an API status request.
#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct ApiStatus {
	/// The server uptime in seconds.
	#[serde(rename = "uptime")]
	pub up_time: f32,
	/// The SHA hash of the most recent commit the server is running.
	pub commit: String,
	/// The version of the database.
	#[serde(rename = "db")]
	pub db_version: u32,
	/// The time in milliseconds of when the request was received, since the
	/// Unix epoch.
	#[serde(rename = "startTime")]
	pub request_start_time: u64,
	/// The time in milliseconds that it took the API to send it's reply.
	#[serde(rename = "processTime")]
	pub request_time_taken: u32,
	/// The load average for the server. The first entry is the average for 5
	/// minutes, and the second is for 15 minutes.
	///
	/// If you want more information about the source of this information, visit
	/// <https://github.com/ajayyy/SponsorBlockServer/blob/06af78c770b82722be8b03d2b1b82eb7409f675b/src/routes/getStatus.ts#L18>
	#[serde(rename = "loadavg")]
	pub load_average: [f32; 2],
}

// Function Implementation
impl Client {
	/// Fetches the API status.
	///
	/// # Errors
	/// Can return any error type from [`SponsorBlockError`]. See the error type
	/// definitions for explanations of when they might be encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_api_status(&self) -> SponsorBlockResult<ApiStatus> {
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
