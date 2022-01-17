//! The interface for segment action types.

// Uses
use std::result::Result as StdResult;

use bitflags::bitflags;
use serde::{de::Error, Deserialize, Deserializer};

use crate::api::convert_to_action_type;

/// The action to take on a segment.
///
/// This is declared for segments upon submission, and basically just recommends
/// how to handle the segment.
///
/// See <https://wiki.sponsor.ajay.app/w/Types#Action_Type> for more information.
#[derive(Debug, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub enum Action {
	/// Skip the segment. This is the default action type.
	Skip,
	/// Mute the segment without skipping.
	///
	/// See the [Mute Segment](https://wiki.sponsor.ajay.app/w/Mute_Segment) article for more
	/// information.
	Mute,
	/// The segment applies to the entire video.
	///
	/// See the [Full Video Labels](https://wiki.sponsor.ajay.app/w/Full_Video_Labels) article for
	/// more information.
	FullVideo,
}

impl<'de> Deserialize<'de> for Action {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
		let action_string = String::deserialize(deserializer)?;
		convert_to_action_type(action_string.as_str()).map_err(D::Error::custom)
	}
}

impl Default for Action {
	fn default() -> Self {
		Self::Skip
	}
}

bitflags! {
	/// A struct for supplying the action types of segments you want to look for in a video.
	pub struct AcceptedActions: u32 {
		/// A convenience constant for having no accepted action types.
		const NONE = 0b0000;
		/// Skip - take a look at [`crate::Action::Skip`] for more information.
		const SKIP = 0b0001;
		/// Mute - take a look at [`crate::Action::Mute`] for more information.
		const MUTE = 0b0010;
		/// Full Video - take a look at [`crate::Action::FullVideo`] for more information.
		const FULL_VIDEO = 0b0100;
	}
}

impl Default for AcceptedActions {
	fn default() -> Self {
		Self::all()
	}
}
