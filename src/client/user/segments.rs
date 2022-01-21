//! The functions for retrieving segments and segment info for videos.

// Uses
use serde::Deserialize;
use serde_json::from_str as from_json_str;
#[cfg(feature = "private_searches")]
use sha2::{Digest, Sha256};

#[cfg(feature = "private_searches")]
use crate::util::bytes_to_hex_string;
use crate::{
	api::{convert_action_bitflags_to_url, convert_category_bitflags_to_url},
	error::{Result, SponsorBlockError},
	segment::{AcceptedActions, AcceptedCategories, ActionKind, Category, Segment},
	util::{
		de::{bool_from_integer_str, none_on_0_0_from_str},
		get_response_text,
		to_url_array,
	},
	AdditionalSegmentInfo,
	Client,
};

// Function-Specific Deserialization Structs
#[cfg(feature = "private_searches")]
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RawHashMatch {
	#[serde(rename = "videoID")]
	video_id: String,
	hash: String,
	segments: Vec<RawSegment>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct RawSegment {
	category: Category,
	action_type: ActionKind,
	#[serde(rename = "segment")]
	time_points: Option<[f32; 2]>,
	start_time: Option<f32>,
	end_time: Option<f32>,
	#[serde(rename = "UUID")]
	uuid: String,
	#[serde(deserialize_with = "bool_from_integer_str")]
	locked: bool,
	votes: i32,
	#[serde(rename = "videoDuration", deserialize_with = "none_on_0_0_from_str")]
	video_duration_upon_submission: Option<f32>,
	#[serde(flatten)]
	additional_info: AdditionalSegmentInfo,
}

impl RawSegment {
	/// Converts a raw segment that more closely matches the structure returned
	/// by the API to the proper rusty [`Segment`] type.
	///
	/// `additional_info` determines whether or not to include
	/// `RawSegment.additional_info`, since it is always populated by Serde but
	/// not with useful values under certain circumstances.
	fn convert_to_segment(self, additional_info: bool) -> Result<Segment> {
		// Process the raw time information
		let time_points = if let Some(points) = self.time_points {
			points
		} else {
			[
				self.start_time
					.expect("time_points was empty but so is start_time"),
				self.end_time
					.expect("time_points was empty but so is end_time"),
			]
		};
		if time_points[0] > time_points[1] {
			return Err(SponsorBlockError::BadData(format!(
				"segment start ({}) > end ({})",
				time_points[0], time_points[1]
			)));
		}
		if time_points[0] < 0.0 {
			return Err(SponsorBlockError::BadData(format!(
				"segment start ({}) < 0",
				time_points[0]
			)));
		}
		if time_points[1] < 0.0 {
			return Err(SponsorBlockError::BadData(format!(
				"segment end ({}) < 0",
				time_points[1]
			)));
		}
		if let Some(video_duration_upon_submission) = self.video_duration_upon_submission {
			if video_duration_upon_submission < 0.0 {
				return Err(SponsorBlockError::BadData(format!(
					"video duration upon submission ({}) < 0",
					video_duration_upon_submission
				)));
			}
		}

		// For backwards-compatibility, the API returns `skip` as the action type for
		// Highlight unless one of the requested action types is `poi`.
		// This makes it so we always return the correct action type regardless.
		// https://github.com/ajayyy/SponsorBlockServer/pull/448
		let mut action_type = self.action_type;
		if self.category == Category::Highlight {
			action_type = ActionKind::PointOfInterest;
		}

		// Build the clean segment
		Ok(Segment {
			category: self.category,
			action: action_type.to_action(time_points),
			uuid: self.uuid,
			locked: self.locked,
			votes: self.votes,
			video_duration_on_submission: self.video_duration_upon_submission,
			additional_info: additional_info.then(|| self.additional_info),
		})
	}
}

// Function Implementation
impl Client {
	/// Fetches the segments for a given video ID.
	///
	/// This function *does not* return additional segment info.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// The only error types among them you may want to handle differently are
	/// [`HttpClient(404)`] and [`NoMatchingVideoHash`], as they indicate that
	/// no videos could be found in the database matching what was provided.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	/// [`HttpClient(404)`]: crate::SponsorBlockError::HttpClient
	/// [`NoMatchingVideoHash`]: crate::SponsorBlockError::NoMatchingVideoHash
	pub async fn fetch_segments<V>(
		&self,
		video_id: V,
		accepted_categories: AcceptedCategories,
		accepted_actions: AcceptedActions,
	) -> Result<Vec<Segment>>
	where
		V: AsRef<str>,
	{
		self.fetch_segments_with_required::<V, &str>(
			video_id,
			accepted_categories,
			accepted_actions,
			&[],
		)
		.await
	}

	/// Fetches the segments for a given video ID.
	///
	/// This variant allows you to specify segment UUIDs to require to be
	/// retrieved, even if they don't meet the minimum vote threshold. If this
	/// isn't something you need, use the regular [`fetch_segments`] instead.
	///
	/// This function *does not* return additional segment info.
	///
	/// # Errors
	/// See the Errors section of the [base version of this
	/// function](Self::fetch_segments).
	///
	/// [`fetch_segments`]: Self::fetch_segments
	pub async fn fetch_segments_with_required<V, S>(
		&self,
		video_id: V,
		accepted_categories: AcceptedCategories,
		accepted_actions: AcceptedActions,
		required_segments: &[S],
	) -> Result<Vec<Segment>>
	where
		V: AsRef<str>,
		S: AsRef<str>,
	{
		// Function Constants
		const API_ENDPOINT: &str = "/skipSegments";

		// Build the request and send it
		let mut request;
		#[cfg(not(feature = "private_searches"))]
		{
			request = self
				.http
				.get(format!("{}{}", &self.base_url, API_ENDPOINT))
				.query(&[("videoID", video_id.as_ref())]);
		}
		#[cfg(feature = "private_searches")]
		{
			let video_id_hash = {
				let mut hasher = Sha256::new();
				hasher.update(video_id.as_ref().as_bytes());
				bytes_to_hex_string(&hasher.finalize()[..])
			};
			request = self.http.get(format!(
				"{}{}/{}",
				&self.base_url,
				API_ENDPOINT,
				&video_id_hash[0..self.hash_prefix_length as usize]
			));
		}

		request = request
			.query(&[(
				"categories",
				convert_category_bitflags_to_url(accepted_categories),
			)])
			.query(&[(
				"actionTypes",
				convert_action_bitflags_to_url(accepted_actions),
			)])
			.query(&[("service", &self.service)]);
		if !required_segments.is_empty() {
			request = request.query(&[("requiredSegments", to_url_array(required_segments))]);
		}
		let response = get_response_text(request.send().await?).await?;

		// Deserialize the response and parse it into the output
		let mut video_segments;
		#[cfg(not(feature = "private_searches"))]
		{
			video_segments = from_json_str::<Vec<RawSegment>>(response.as_str())?;
		}
		#[cfg(feature = "private_searches")]
		{
			let mut found_match = false;
			video_segments = Vec::new();
			for hash_match in from_json_str::<Vec<RawHashMatch>>(response.as_str())?.drain(..) {
				if hash_match.video_id == video_id.as_ref() {
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
			.map(|s| s.convert_to_segment(false))
			.collect()
	}

	/// Fetches complete info for a segment.
	///
	/// This function *does* return additional segment info.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_segment_info<S>(&self, segment_uuid: S) -> Result<Segment>
	where
		S: AsRef<str>,
	{
		Ok(self
			.fetch_segment_info_multiple(&[segment_uuid])
			.await?
			.pop()
			.ok_or_else(|| SponsorBlockError::BadData("no segments found".to_owned()))?)
	}

	/// Fetches complete info for segments.
	///
	/// This function *does* return additional segment info.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	pub async fn fetch_segment_info_multiple<S>(&self, segment_uuids: &[S]) -> Result<Vec<Segment>>
	where
		S: AsRef<str>,
	{
		// Function Constants
		const API_ENDPOINT: &str = "/segmentInfo";

		// Build the request and send it
		let request = self
			.http
			.get(format!("{}{}", &self.base_url, API_ENDPOINT))
			.query(&[("UUIDs", to_url_array(segment_uuids))]);
		let response = get_response_text(request.send().await?).await?;

		// Deserialize the response and parse it into the output
		from_json_str::<Vec<RawSegment>>(response.as_str())?
			.drain(..)
			.map(|s| s.convert_to_segment(true))
			.collect()
	}
}
