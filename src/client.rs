//! The SponsorBlock client.

// Uses
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};
use serde::Deserialize;
use serde_json::from_str as from_json_str;

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
	error::{Result, SponsorBlockError},
	segment::{AcceptedCategories, Action, ActionableSegment, Segment, TimePoint, TimeSection},
	util::get_response_text,
};

/// The client for interfacing with SponsorBlock.
pub struct Client {
	// Internal
	http: ReqwestClient,

	// Config
	user_id: String,
	base_url: String,
	hash_prefix_length: u8,
	service: String,
}

impl Client {
	/// Creates a new instance of the client.
	pub fn new<S: Into<String>>(user_id: S) -> Self {
		const DEFAULT_BASE_URL: &str = "https://sponsor.ajay.app";
		const DEFAULT_HASH_PREFIX_LENGTH: u8 = 4;
		const DEFAULT_SERVICE: &str = "YouTube";
		const DEFAULT_USER_AGENT: &str =
			concat!(env!("CARGO_PKG_NAME"), "-rs/", env!("CARGO_PKG_VERSION"));

		Self {
			http: ReqwestClientBuilder::new()
				.user_agent(DEFAULT_USER_AGENT)
				.build()
				.expect("unable to build the HTTP client"),
			user_id: user_id.into(),
			base_url: String::from(DEFAULT_BASE_URL),
			hash_prefix_length: DEFAULT_HASH_PREFIX_LENGTH,
			service: String::from(DEFAULT_SERVICE),
		}
	}

	/// Fetches the segments for a given video ID.
	///
	/// # Errors
	pub async fn fetch_segments(
		&self,
		video_id: &str,
		accepted_categories: AcceptedCategories,
		//required_segments: &[S],
	) -> Result<Vec<Segment>> {
		const API_ENDPOINT: &str = "/api/skipSegments";

		#[derive(Deserialize, Debug)]
		struct RawSegment {
			category: String,
			#[serde(rename = "actionType")]
			action_type: String,
			segment: [f32; 2],
			#[serde(rename = "UUID")]
			uuid: String,
			locked: u8,
			votes: u32,
			#[serde(rename = "videoDuration")]
			video_duration_upon_submission: f32,
		}

		let request = self
			.http
			.get(format!("{}{}", &self.base_url, API_ENDPOINT))
			.query(&[("videoID", video_id)])
			.query(&[("categories", accepted_categories.gen_url_value())])
			.query(&[("service", &self.service)]);
		let response_result = get_response_text(request.send().await?).await;
		// Special case to just return an empty Vec if the video has no segments, rather
		// than a 404 error.
		if let Err(SponsorBlockError::HttpClient(404)) = response_result {
			return Ok(Vec::with_capacity(0));
		}
		let response = response_result?;
		dbg!(&response);
		from_json_str::<Vec<RawSegment>>(response.as_str())?
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
