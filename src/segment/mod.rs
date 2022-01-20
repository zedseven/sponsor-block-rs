//! Everything to do with segments.

// Uses
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
	util::de::{bool_from_integer_str, datetime_from_millis_timestamp},
	Client,
	PublicUserId,
	Result,
	SegmentUuid,
	VideoId,
};

// Modules
mod action;
mod category;

// Public Exports
pub use self::{action::*, category::*};

/// A segment, representing a section or point in time in a video that is worth
/// skipping or otherwise treating specially.
#[derive(Debug)]
#[non_exhaustive]
pub struct Segment {
	/// The kind of segment.
	pub category: Category,
	/// What action the submitter recommended to take for the segment.
	/// This also encodes the time information if it is relevant.
	pub action: Action,
	/// The UUID of the segment submitter.
	pub uuid: SegmentUuid,
	/// Whether the segment is locked or not.
	pub locked: bool,
	/// How many votes the segment has.
	pub votes: i32,
	/// The video duration upon submission of the segment. Used to determine if
	/// the segment is out of date.
	///
	/// It's an [`Option`] because segments submitted before video duration was
	/// tracked don't have this value.
	///
	/// If [`None`], it doesn't immediately mean the segment is out of date,
	/// just that the segment is old.
	pub video_duration_on_submission: Option<f32>,
	/// Additional segment information that isn't always provided by the API,
	/// depending on the function.
	///
	/// Whether or not a function supplies this information will be
	/// noted in its documentation.
	pub additional_info: Option<AdditionalSegmentInfo>,
}

impl Segment {
	/// Fetches the additional information for the segment, filling in the
	/// [`additional_info`] field.
	///
	/// If the information is already present, no API requests are made.
	///
	/// This function returns whether or not it had to request information from
	/// the API.
	///
	/// # Errors
	/// Can return pretty much any error type from [`SponsorBlockError`]. See
	/// the error type definitions for explanations of when they might be
	/// encountered.
	///
	/// [`additional_info`]: Self::additional_info
	/// [`SponsorBlockError`]: crate::SponsorBlockError
	#[cfg(feature = "user")]
	pub async fn fetch_additional_info(&mut self, client: &Client) -> Result<bool> {
		if self.additional_info.is_some() {
			return Ok(false);
		}

		self.additional_info = client.fetch_segment_info(&self.uuid).await?.additional_info;

		Ok(true)
	}
}

/// Additional segment information that isn't always provided by the API,
/// depending on the function.
///
/// Whether or not a function supplies this information will be
/// noted in its documentation.
#[derive(Deserialize, Debug)]
#[non_exhaustive]
#[serde(default, rename_all = "camelCase")]
pub struct AdditionalSegmentInfo {
	/// The video ID associated with the segment.
	#[serde(rename = "videoID")]
	pub video_id: VideoId,
	/// The number of incorrect votes.
	pub incorrect_votes: u32,
	/// The public user ID of the segment submitter.
	#[serde(rename = "userID")]
	pub submitter_id: PublicUserId,
	/// The date and time that the segment was submitted.
	#[serde(deserialize_with = "datetime_from_millis_timestamp")]
	pub time_submitted: OffsetDateTime,
	/// The number of views the segment has.
	pub views: u32,
	/// The service the segment is associated with.
	pub service: String,
	/// Whether or not the segment is hidden.
	#[serde(deserialize_with = "bool_from_integer_str")]
	pub hidden: bool,
	/// The reputation of the submitter upon submission of the segment.
	pub submitter_reputation: f32,
	/// Whether or not the submitter is shadow-banned.
	#[serde(deserialize_with = "bool_from_integer_str")]
	pub shadow_banned: bool,
	/// The user agent string of the submitter upon submission.
	pub submitter_user_agent: String,
}

impl Default for AdditionalSegmentInfo {
	fn default() -> Self {
		Self {
			video_id: VideoId::default(),
			incorrect_votes: u32::default(),
			submitter_id: PublicUserId::default(),
			time_submitted: OffsetDateTime::UNIX_EPOCH, /* Not great, but this should in theory
			                                             * never be called */
			views: u32::default(),
			service: String::default(),
			hidden: bool::default(),
			submitter_reputation: f32::default(),
			shadow_banned: bool::default(),
			submitter_user_agent: String::default(),
		}
	}
}
