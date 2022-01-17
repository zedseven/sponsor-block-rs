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
	segment::{AcceptedCategories, ActionableSegmentKind, Segment},
	util::{
		de::{bool_from_integer_str, none_on_0_0_from_str},
		get_response_text,
		to_url_array,
	},
	AcceptedActions,
	Action,
	AdditionalSegmentInfo,
	Client,
	SegmentUuid,
	SegmentUuidSlice,
	VideoId,
	VideoIdSlice,
};

// Function-Specific Deserialization Structs
#[cfg(feature = "private_searches")]
#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct RawHashMatch {
	#[serde(rename = "videoID")]
	video_id: VideoId,
	hash: String,
	segments: Vec<RawSegment>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
struct RawSegment {
	category: ActionableSegmentKind,
	#[serde(rename = "actionType")]
	action_type: Action,
	#[serde(rename = "segment")]
	time_points: Option<[f32; 2]>,
	start_time: Option<f32>,
	end_time: Option<f32>,
	#[serde(rename = "UUID")]
	uuid: SegmentUuid,
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

		Ok(Segment {
			segment: self.category.to_actionable_segment(time_points),
			action_type: self.action_type,
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
	pub async fn fetch_segments(
		&self,
		video_id: &VideoIdSlice,
		accepted_categories: AcceptedCategories,
		accepted_actions: AcceptedActions,
	) -> Result<Vec<Segment>> {
		self.fetch_segments_with_required::<&SegmentUuidSlice>(
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
	pub async fn fetch_segments_with_required<S: AsRef<SegmentUuidSlice>>(
		&self,
		video_id: &VideoIdSlice,
		accepted_categories: AcceptedCategories,
		accepted_actions: AcceptedActions,
		required_segments: &[S],
	) -> Result<Vec<Segment>> {
		// Function Constants
		const API_ENDPOINT: &str = "/skipSegments";

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
	pub async fn fetch_segment_info<S: AsRef<SegmentUuidSlice>>(
		&self,
		segment_uuid: S,
	) -> Result<Segment> {
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
	pub async fn fetch_segment_info_multiple<S: AsRef<SegmentUuidSlice>>(
		&self,
		segment_uuids: &[S],
	) -> Result<Vec<Segment>> {
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
