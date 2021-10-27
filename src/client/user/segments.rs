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

// Function Implementation
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
	/// [`SponsorBlockError`]: crate::SponsorBlockError
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
}
