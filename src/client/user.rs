//! Standard user functions.

// Uses
use serde::Deserialize;
use serde_json::from_str as from_json_str;
#[cfg(feature = "private_searches")]
use sha2::{Digest, Sha256};

#[cfg(feature = "private_searches")]
use crate::util::bytes_to_hex_string;
use crate::{
	api::{
		ACTION_MUTE_NAME,
		ACTION_SKIP_NAME,
		ENDCARDS_CREDITS_NAME,
		HIGHLIGHT_NAME,
		INTERACTION_REMINDER_NAME,
		INTERMISSION_INTRO_ANIMATION_NAME,
		NON_MUSIC_NAME,
		PREVIEW_RECAP_NAME,
		SPONSOR_NAME,
		UNPAID_SELF_PROMOTION_NAME,
	},
	error::{SponsorBlockError, SponsorBlockResult},
	segment::{AcceptedCategories, Action, ActionableSegment, Segment, TimePoint, TimeSection},
	util::{get_response_text, to_url_array},
	Client,
};

impl Client {
	/// Fetches the segments for a given video ID.
	///
	/// # Errors
	/// Can return any error type from [`SponsorBlockError`]. See the error type
	/// definitions for explanations of when they might be encountered.
	///
	/// The only error type among them you may want to handle differently is
	/// [`HttpClient(404)`], as that indicates that no videos could be found in
	/// the database matching what was provided.
	///
	/// [`HttpClient(404)`]: crate::SponsorBlockError::HttpClient
	pub async fn fetch_segments(
		&self,
		video_id: &str,
		accepted_categories: AcceptedCategories,
	) -> SponsorBlockResult<Vec<Segment>> {
		self.fetch_segments_with_required::<&str>(video_id, accepted_categories, &[])
			.await
	}

	/// Fetches the segments for a given video ID.
	///
	/// This variant allows you to specify segment UUIDs to require to be
	/// retrieved, even if they don't meet the minimum vote threshold. If this
	/// isn't something you need, use the regular [`fetch_segments`] instead.
	///
	/// # Errors
	/// See the Errors section of the [base version of this
	/// function](Self::fetch_segments).
	///
	/// [`fetch_segments`]: Self::fetch_segments
	pub async fn fetch_segments_with_required<S: AsRef<str>>(
		&self,
		video_id: &str,
		accepted_categories: AcceptedCategories,
		required_segments: &[S],
	) -> SponsorBlockResult<Vec<Segment>> {
		// Function Constants
		const API_ENDPOINT: &str = "/api/skipSegments";

		// Function-Specific Deserialization Structs
		#[cfg(feature = "private_searches")]
		#[derive(Deserialize, Debug)]
		struct RawHashMatch {
			#[serde(rename = "videoID")]
			video_id: String,
			hash: String,
			segments: Vec<RawSegment>,
		}

		#[derive(Deserialize, Debug)]
		struct RawSegment {
			category: String,
			#[serde(rename = "actionType")]
			action_type: String,
			segment: [f32; 2],
			#[serde(rename = "UUID")]
			uuid: String,
			locked: u8,
			votes: i32,
			#[serde(rename = "videoDuration")]
			video_duration_upon_submission: f32,
		}

		// Build the request and send it
		let mut request;
		#[cfg(not(feature = "private_searches"))]
		{
			request = self
				.http
				.get(format!("{}{}", &self.base_url, API_ENDPOINT))
				.query(&[("videoID", video_id)]);
		}
		#[cfg(feature = "private_searches")]
		{
			let video_id_hash = {
				let mut hasher = Sha256::new();
				Digest::update(&mut hasher, video_id.as_bytes());
				bytes_to_hex_string(&hasher.finalize()[..])
			};
			request = self.http.get(format!(
				"{}{}",
				&self.base_url,
				format!(
					"{}/{}",
					API_ENDPOINT,
					&video_id_hash[0..self.hash_prefix_length as usize]
				)
			));
		}

		request = request
			.query(&[("categories", accepted_categories.gen_url_value())])
			.query(&[("service", &self.service)]);
		if !required_segments.is_empty() {
			request = request.query(&[("requiredSegments", to_url_array(required_segments))]);
		}
		dbg!(&request);
		let response = get_response_text(request.send().await?).await?;
		dbg!(&response);

		// Deserialize the response and parse it into the output
		let mut video_segments;
		#[cfg(not(feature = "private_searches"))]
		{
			video_segments = from_json_str::<Vec<RawSegment>>(response.as_str())?
		}
		#[cfg(feature = "private_searches")]
		{
			let mut found_match = false;
			video_segments = Vec::new();
			for hash_match in from_json_str::<Vec<RawHashMatch>>(response.as_str())?.drain(..) {
				if hash_match.video_id == video_id {
					video_segments = hash_match.segments;
					found_match = true;
					break;
				}
			}
			if !found_match {
				return Err(SponsorBlockError::NoMatchingVideoHash);
			}
		}

		video_segments
			.drain(..)
			.map(|s| {
				if s.segment[0] > s.segment[1] {
					return Err(SponsorBlockError::BadData(format!(
						"segment start ({}) > end ({})",
						s.segment[0], s.segment[1]
					)));
				}
				if s.segment[0] < 0.0 {
					return Err(SponsorBlockError::BadData(format!(
						"segment start ({}) < 0",
						s.segment[0]
					)));
				}
				if s.segment[1] < 0.0 {
					return Err(SponsorBlockError::BadData(format!(
						"segment end ({}) < 0",
						s.segment[1]
					)));
				}

				Ok(Segment {
					segment: match s.category.as_str() {
						SPONSOR_NAME => ActionableSegment::Sponsor(TimeSection {
							start: s.segment[0],
							end: s.segment[1],
						}),
						UNPAID_SELF_PROMOTION_NAME => {
							ActionableSegment::UnpaidSelfPromotion(TimeSection {
								start: s.segment[0],
								end: s.segment[1],
							})
						}
						INTERACTION_REMINDER_NAME => {
							ActionableSegment::InteractionReminder(TimeSection {
								start: s.segment[0],
								end: s.segment[1],
							})
						}
						HIGHLIGHT_NAME => ActionableSegment::Highlight(TimePoint {
							point: s.segment[0],
						}),
						INTERMISSION_INTRO_ANIMATION_NAME => {
							ActionableSegment::IntermissionIntroAnimation(TimeSection {
								start: s.segment[0],
								end: s.segment[1],
							})
						}
						ENDCARDS_CREDITS_NAME => ActionableSegment::EndcardsCredits(TimeSection {
							start: s.segment[0],
							end: s.segment[1],
						}),
						PREVIEW_RECAP_NAME => ActionableSegment::PreviewRecap(TimeSection {
							start: s.segment[0],
							end: s.segment[1],
						}),
						NON_MUSIC_NAME => ActionableSegment::NonMusic(TimeSection {
							start: s.segment[0],
							end: s.segment[1],
						}),
						unknown_value => {
							return Err(SponsorBlockError::UnknownValue {
								r#type: "category".to_owned(),
								value: unknown_value.to_owned(),
							})
						}
					},
					action_type: match s.action_type.as_str() {
						ACTION_SKIP_NAME => Action::Skip,
						ACTION_MUTE_NAME => Action::Mute,
						unknown_value => {
							return Err(SponsorBlockError::UnknownValue {
								r#type: "actionType".to_owned(),
								value: unknown_value.to_owned(),
							})
						}
					},
					uuid: s.uuid,
					locked: s.locked != 0,
					votes: s.votes,
					video_duration_upon_submission: s.video_duration_upon_submission,
				})
			})
			.collect()
	}

	/// Fetches a user's info using a user's public user ID.
	///
	/// # Errors
	/// Can return any error type from [`SponsorBlockError`]. See the error type
	/// definitions for explanations of when they might be encountered.
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

	/// Fetches the API status.
	///
	/// # Errors
	/// Can return any error type from [`SponsorBlockError`]. See the error type
	/// definitions for explanations of when they might be encountered.
	pub async fn fetch_api_status(&self) -> SponsorBlockResult<ApiStatus> {
		// Function Constants
		const API_ENDPOINT: &str = "/api/status";

		// Build the request
		let request = self.http.get(format!("{}{}", &self.base_url, API_ENDPOINT));

		// Send the request
		let response = get_response_text(request.send().await?).await?;

		// Parse the response
		Ok(from_json_str::<ApiStatus>(response.as_str())?)
	}
}

// Data Structs
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

/// The results of an API status request.
#[derive(Deserialize, Debug)]
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
}
