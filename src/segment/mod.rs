//! Everything to do with segments.

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
	pub uuid: String,
	/// Whether the segment is locked or not.
	pub locked: bool,
	/// How many votes the segment has.
	pub votes: i32,
	/// The video duration upon submission of the segment. Used to determine if
	/// the segment is out of date.
	pub video_duration_upon_submission: f32,
}

/// The action to take on a segment.
///
/// This is declared for segments upon submission, and basically just recommends
/// whether to mute or skip the entire section.
#[derive(Debug)]
pub enum Action {
	/// Skip the segment.
	Skip,
	/// Mute the segment without skipping.
	Mute,
}

/// A video segment, containing timestamp information.
///
/// For segment types, visit: <https://wiki.sponsor.ajay.app/w/Segment_Categories>
#[derive(Debug)]
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
