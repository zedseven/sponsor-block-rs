//! The interface for segment action types.

// Uses
use std::result::Result as StdResult;

use bitflags::bitflags;
use enum_kinds::EnumKind;
use serde::{de::Error, Deserialize, Deserializer};

use crate::api::convert_to_action_kind;

/// The action to take on a segment.
///
/// This is declared for segments upon submission, and basically just recommends
/// how to handle the segment.
///
/// See <https://wiki.sponsor.ajay.app/w/Types#Action_Type> for more information.
#[derive(Debug, EnumKind)]
#[non_exhaustive]
#[enum_kind(ActionKind, non_exhaustive, derive(Hash))]
pub enum Action {
	/// Skip the segment. This is the default action type.
	Skip(TimeSection),

	/// [Mute](https://wiki.sponsor.ajay.app/w/Mute_Segment)
	///
	/// Mute the segment without skipping.
	Mute(TimeSection),

	/// A single point in the video.
	/// Not a skippable segment, but used as a point to potentially *skip to*.
	PointOfInterest(TimePoint),

	/// [Full Video Label](https://wiki.sponsor.ajay.app/w/Full_Video_Labels)
	///
	/// The segment applies to the entire video. The associated Action is too
	/// tightly integrated with the video so if relevant content was skipped,
	/// the majority of the video would be cut. There may still be associated
	/// segments for parts that can be cleanly skipped.
	///
	/// This is mostly an informational action type; not much action can be
	/// taken with it.
	FullVideo,
}

bitflags! {
	/// A struct for supplying the action types of segments you want to look for in a video.
	pub struct AcceptedActions: u32 {
		/// A convenience constant for having no accepted action types.
		const NONE = 0b0000;
		/// Skip - take a look at [`crate::Action::Skip`] for more information.
		const SKIP = 0b0001;
		/// Mute - take a look at [`crate::Action::PointOfInterest`] for more information.
		const MUTE = 0b0010;
		/// Point of Interest - take a look at [`crate::Action::Mute`] for more information.
		const POINT_OF_INTEREST = 0b0100;
		/// Full Video - take a look at [`crate::Action::FullVideo`] for more information.
		const FULL_VIDEO = 0b1000;
	}
}

impl Default for AcceptedActions {
	fn default() -> Self {
		Self::all()
	}
}

/// A skippable section, category-agnostic. Contains a start and end time.
///
/// `start` is guaranteed to be <= `end`.
#[derive(Debug)]
#[non_exhaustive]
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
#[non_exhaustive]
pub struct TimePoint {
	/// The singular point in time.
	pub point: f32,
}

impl ActionKind {
	pub(crate) fn to_action(self, time_points: [f32; 2]) -> Action {
		match self {
			ActionKind::Skip => Action::Skip(TimeSection {
				start: time_points[0],
				end: time_points[1],
			}),
			ActionKind::Mute => Action::Mute(TimeSection {
				start: time_points[0],
				end: time_points[1],
			}),
			ActionKind::PointOfInterest => Action::PointOfInterest(TimePoint {
				point: time_points[0],
			}),
			ActionKind::FullVideo => Action::FullVideo,
		}
	}
}

impl<'de> Deserialize<'de> for ActionKind {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
		let action_string = String::deserialize(deserializer)?;
		convert_to_action_kind(action_string.as_str()).map_err(D::Error::custom)
	}
}

impl Default for ActionKind {
	fn default() -> Self {
		Self::Skip
	}
}
