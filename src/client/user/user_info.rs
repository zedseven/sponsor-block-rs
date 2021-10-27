// Uses
use serde::Deserialize;
use serde_json::from_str as from_json_str;

use crate::{error::SponsorBlockResult, util::get_response_text, Client};

/// The results of a user info request.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
	/// The user's public user ID.
	#[serde(rename = "userID")]
	pub public_user_id: String,
	/// The user's username.
	pub user_name: Option<String>,
	/// The number of minutes this user has saved other users.
	pub minutes_saved: f32,
	/// The total number of segments submitted, excluding ignored & hidden
	/// segments.
	pub segment_count: u32,
	/// The total number of ignored & hidden segments submitted.
	pub ignored_segment_count: u32,
	/// The total number of views, excluding those on ignored & hidden segments
	/// that other users have on this user's segments.
	pub view_count: u32,
	/// The total number of views on ignored & hidden segments that other users
	/// have on this user's segments.
	pub ignored_view_count: u32,
	/// The number of currently-enabled warnings.
	pub warnings: u32,
	/// The user's reputation.
	pub reputation: f32,
	/// The VIP status.
	pub vip: bool,
	/// the UUID of the last submitted segment.
	#[serde(rename = "lastSegmentID")]
	pub last_segment_id: Option<String>,
}

impl UserInfo {
	/// A convenience function that gets the total segment count.
	/// (`segment_count + ignored_segment_count`)
	#[must_use]
	pub fn total_segment_count(&self) -> u32 {
		self.segment_count + self.ignored_segment_count
	}

	/// A convenience function that gets the total view count.
	/// (`view_count + ignored_view_count`)
	#[must_use]
	pub fn total_view_count(&self) -> u32 {
		self.view_count + self.ignored_view_count
	}
}

// Function Implementation
impl Client {
	/// Fetches a user's info using a user's public user ID.
	///
	/// # Errors
	/// Can return any error type from [`SponsorBlockError`]. See the error type
	/// definitions for explanations of when they might be encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_user_info_public(
		&self,
		public_user_id: &str,
	) -> SponsorBlockResult<UserInfo> {
		// Function Constants
		const API_ENDPOINT: &str = "/api/userInfo";

		// Build the request
		let request = self
			.http
			.get(format!("{}{}", &self.base_url, API_ENDPOINT))
			.query(&[("publicUserID", public_user_id)]);

		// Send the request
		let response = get_response_text(request.send().await?).await?;

		// Parse the response
		let mut result = from_json_str::<UserInfo>(response.as_str())?;
		// The user name is set to the public user ID if not set. This converts it to a
		// more idiomatic value transparently.
		if result
			.user_name
			.as_ref()
			.expect("userName field was not set")
			.eq(&result.public_user_id)
		{
			result.user_name = None;
		}
		Ok(result)
	}

	/// Fetches a user's info using a user's local (private) user ID.
	///
	/// # Errors
	/// Can return any error type from [`SponsorBlockError`]. See the error type
	/// definitions for explanations of when they might be encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_user_info_local(&self, local_user_id: &str) -> SponsorBlockResult<UserInfo> {
		// Function Constants
		const API_ENDPOINT: &str = "/api/userInfo";

		// Build the request
		let request = self
			.http
			.get(format!("{}{}", &self.base_url, API_ENDPOINT))
			.query(&[("userID", local_user_id)]);

		// Send the request
		let response = get_response_text(request.send().await?).await?;

		// Parse the response
		let mut result = from_json_str::<UserInfo>(response.as_str())?;
		// The user name is set to the public user ID if not set. This converts it to a
		// more idiomatic value transparently.
		if result
			.user_name
			.as_ref()
			.expect("userName field was not set")
			.eq(&result.public_user_id)
		{
			result.user_name = None;
		}
		Ok(result)
	}
}
