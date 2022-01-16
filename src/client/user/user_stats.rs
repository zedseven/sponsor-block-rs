// Uses
use std::{collections::HashMap, result::Result as StdResult};

use serde::{Deserialize, Deserializer};
use serde_json::from_str as from_json_str;

use crate::{
	api::{convert_to_action_type, convert_to_segment_kind},
	error::Result,
	util::{de::map_hashmap_key_from_str, get_response_text},
	Action,
	ActionableSegmentKind,
	Client,
	LocalUserIdSlice,
	PublicUserId,
	PublicUserIdSlice,
};

/// The results of a user info request.
#[derive(Deserialize, Debug, Default)]
#[non_exhaustive]
#[serde(default, rename_all = "camelCase")]
pub struct UserStats {
	/// The user's public user ID.
	#[serde(rename = "userID")]
	pub user_id: PublicUserId,
	/// The user's username.
	pub user_name: Option<String>,
	/// The overall stats for the user.
	pub overall_stats: OverallStats,
	/// The categories with associated segment counts.
	#[serde(deserialize_with = "map_category_kinds")]
	pub category_count: HashMap<ActionableSegmentKind, u32>,
	/// The action types with associated segment counts.
	#[serde(deserialize_with = "map_action_types")]
	pub action_type_count: HashMap<Action, u32>,
}

fn map_category_kinds<'de, D: Deserializer<'de>, O: Deserialize<'de>>(
	deserializer: D,
) -> StdResult<HashMap<ActionableSegmentKind, O>, D::Error> {
	map_hashmap_key_from_str(deserializer, convert_to_segment_kind)
}

fn map_action_types<'de, D: Deserializer<'de>, O: Deserialize<'de>>(
	deserializer: D,
) -> StdResult<HashMap<Action, O>, D::Error> {
	map_hashmap_key_from_str(deserializer, convert_to_action_type)
}

/// The overall stats for a user, similar to what [`UserInfo`] provides.
///
/// TODO: Find a nice way to remove this. <https://github.com/serde-rs/serde/issues/2115>
///
/// [`UserInfo`]: super::user_info::UserInfo
#[derive(Deserialize, Debug, Default)]
#[non_exhaustive]
#[serde(default, rename_all = "camelCase")]
pub struct OverallStats {
	/// The number of minutes this user has saved other users.
	pub minutes_saved: f32,
	/// The total number of segments submitted, excluding ignored & hidden
	/// segments.
	pub segment_count: u32,
}

// Function Constants
const API_ENDPOINT: &str = "/userStats";

// Function Implementation
impl Client {
	/// Fetches a user's info using a public user ID.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_user_stats_public(
		&self,
		public_user_id: &PublicUserIdSlice,
	) -> Result<UserStats> {
		// Build the request
		let request = self
			.http
			.get(format!("{}{}", &self.base_url, API_ENDPOINT))
			.query(&[("publicUserID", public_user_id)])
			.query(&[("fetchCategoryStats", true), ("fetchActionTypeStats", true)]);

		// Send the request
		let response = get_response_text(request.send().await?).await?;

		// Parse the response
		let mut result = from_json_str::<UserStats>(response.as_str())?;
		// The user name is set to the public user ID if not set. This converts it to a
		// more idiomatic value transparently.
		if result
			.user_name
			.as_ref()
			.expect("userName field was not set")
			.eq(&result.user_id)
		{
			result.user_name = None;
		}
		Ok(result)
	}

	/// Fetches a user's info using a local (private) user ID.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_user_stats_local(
		&self,
		local_user_id: &LocalUserIdSlice,
	) -> Result<UserStats> {
		// Build the request
		let request = self
			.http
			.get(format!("{}{}", &self.base_url, API_ENDPOINT))
			.query(&[("userID", local_user_id)])
			.query(&[("fetchCategoryStats", true), ("fetchActionTypeStats", true)]);

		// Send the request
		let response = get_response_text(request.send().await?).await?;

		// Parse the response
		let mut result = from_json_str::<UserStats>(response.as_str())?;
		// The user name is set to the public user ID if not set. This converts it to a
		// more idiomatic value transparently.
		if result
			.user_name
			.as_ref()
			.expect("userName field was not set")
			.eq(&result.user_id)
		{
			result.user_name = None;
		}
		Ok(result)
	}
}
