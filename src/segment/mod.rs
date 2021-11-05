//! Everything to do with segments.

// Uses
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use enum_kinds::EnumKind;
use serde::{de::Error, Deserialize, Deserializer};

use crate::{
	api::{convert_to_action_type, convert_to_segment_kind},
	util::bool_from_integer_str,
	Client,
	PublicUserId,
	SegmentUuid,
	SponsorBlockResult,
	VideoId,
};

// Modules
mod category;

// Public Exports
pub use self::category::*;

/// A segment, representing a section or point in time in a video that is worth
/// skipping or otherwise treating specially.
#[derive(Debug)]
pub struct Segment {
	/// The section with timestamp values to act upon.
	pub segment: ActionableSegment,
	/// What action the submitter recommended to take on the segment.
	/// (skip/mute)
	pub action_type: Action,
	/// The UUID of the segment submitter.
	pub uuid: SegmentUuid,
	/// Whether the segment is locked or not.
	pub locked: bool,
	/// How many votes the segment has.
	pub votes: i32,
	/// The video duration upon submission of the segment. Used to determine if
	/// the segment is out of date.
	pub video_duration_on_submission: f32,
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
	pub async fn fetch_additional_info(&mut self, client: &Client) -> SponsorBlockResult<bool> {
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
	#[serde(with = "ts_milliseconds")]
	pub time_submitted: DateTime<Utc>,
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
			incorrect_votes: Default::default(),
			submitter_id: PublicUserId::default(),
			time_submitted: Utc::now(), // Not great, but this should in theory never be called
			views: Default::default(),
			service: String::default(),
			hidden: Default::default(),
			submitter_reputation: Default::default(),
			shadow_banned: Default::default(),
			submitter_user_agent: String::default(),
		}
	}
}

/// The action to take on a segment.
///
/// This is declared for segments upon submission, and basically just recommends
/// whether to mute or skip the entire section.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Action {
	/// Skip the segment.
	Skip,
	/// Mute the segment without skipping.
	Mute,
}

impl<'de> Deserialize<'de> for Action {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let action_string = String::deserialize(deserializer)?;
		convert_to_action_type(action_string.as_str()).map_err(D::Error::custom)
	}
}

impl Default for Action {
	fn default() -> Self {
		Self::Skip
	}
}

/// A video segment, containing timestamp information.
///
/// For segment types, visit: <https://wiki.sponsor.ajay.app/w/Segment_Categories>
#[derive(EnumKind, Debug)]
#[enum_kind(ActionableSegmentKind, derive(Hash))]
pub enum ActionableSegment {
	/// Sponsor
	///
	/// A paid promotion, paid referral, or direct advertisement.
	Sponsor(TimeSection),

	/// Unpaid/Self-Promotion
	///
	/// Similar to a sponsor, except it's unpaid or self-promotion. This
	/// includes sections about merchandise, donations, or information about who
	/// the creator collaborated with.
	UnpaidSelfPromotion(TimeSection),

	/// Interaction Reminder
	///
	/// When there is a short reminder to like, subscribe, or follow in the
	/// middle of content.
	InteractionReminder(TimeSection),

	/// Highlight
	///
	/// For getting to the point or highlight of the video.
	Highlight(TimePoint),

	/// Intermission/Intro Animation
	///
	/// An interval without actual content. It could be a pause, static frame,
	/// or repeating animation.
	IntermissionIntroAnimation(TimeSection),

	/// Endcards/Credits
	///
	/// Credits, or when the YouTube endcards appear.
	EndcardsCredits(TimeSection),

	/// Preview/Recap
	///
	/// A quick recap of previous episodes, or a preview of what's coming up
	/// later in the current video.
	PreviewRecap(TimeSection),

	/// Non-Music
	///
	/// Only for use in music videos. A section of the video with non-music
	/// content.
	NonMusic(TimeSection),
}

impl ActionableSegmentKind {
	pub(crate) fn to_actionable_segment(self, time_points: (f32, f32)) -> ActionableSegment {
		match self {
			ActionableSegmentKind::Sponsor => ActionableSegment::Sponsor(TimeSection {
				start: time_points.0,
				end: time_points.1,
			}),
			ActionableSegmentKind::UnpaidSelfPromotion => {
				ActionableSegment::UnpaidSelfPromotion(TimeSection {
					start: time_points.0,
					end: time_points.1,
				})
			}
			ActionableSegmentKind::InteractionReminder => {
				ActionableSegment::InteractionReminder(TimeSection {
					start: time_points.0,
					end: time_points.1,
				})
			}
			ActionableSegmentKind::Highlight => ActionableSegment::Highlight(TimePoint {
				point: time_points.0,
			}),
			ActionableSegmentKind::IntermissionIntroAnimation => {
				ActionableSegment::IntermissionIntroAnimation(TimeSection {
					start: time_points.0,
					end: time_points.1,
				})
			}
			ActionableSegmentKind::EndcardsCredits => {
				ActionableSegment::EndcardsCredits(TimeSection {
					start: time_points.0,
					end: time_points.1,
				})
			}
			ActionableSegmentKind::PreviewRecap => ActionableSegment::PreviewRecap(TimeSection {
				start: time_points.0,
				end: time_points.1,
			}),
			ActionableSegmentKind::NonMusic => ActionableSegment::NonMusic(TimeSection {
				start: time_points.0,
				end: time_points.1,
			}),
		}
	}
}

impl<'de> Deserialize<'de> for ActionableSegmentKind {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let action_string = String::deserialize(deserializer)?;
		convert_to_segment_kind(action_string.as_str()).map_err(D::Error::custom)
	}
}

impl Default for ActionableSegmentKind {
	fn default() -> Self {
		Self::Sponsor
	}
}

/// A skippable section, category-agnostic. Contains a start and end time.
///
/// `start` is guaranteed to be <= `end`.
#[derive(Debug)]
pub struct TimeSection {
	/// The start point of the section.
	pub start: f32,
	/// The end point of the section.
	pub end: f32,
}

impl TimeSection {
	/// Gets the duration of the section.
	#[must_use]
	pub fn duration(&self) -> f32 {
		self.end - self.start
	}
}

/// A singular point in the video, category-agnostic.
#[derive(Debug)]
pub struct TimePoint {
	/// The singular point in time.
	pub point: f32,
}
