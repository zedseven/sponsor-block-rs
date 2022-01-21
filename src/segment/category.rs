//! The interface for segment categories.

// Uses
use std::result::Result as StdResult;

use bitflags::bitflags;
use serde::{de::Error, Deserialize, Deserializer};

use crate::api::convert_to_category;

/// A video segment category, containing timestamp information.
///
/// For a list of all types, visit: <https://wiki.sponsor.ajay.app/w/Segment_Categories>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Category {
	/// [Sponsor](https://wiki.sponsor.ajay.app/w/Sponsor)
	///
	/// A paid promotion, paid referral, or direct advertisement.
	Sponsor,

	/// [Unpaid/Self-Promotion](https://wiki.sponsor.ajay.app/w/Unpaid/Self_Promotion)
	///
	/// Similar to a sponsor, except it's unpaid or self-promotion. This
	/// includes sections about merchandise, donations, or information about who
	/// the creator collaborated with.
	UnpaidSelfPromotion,

	/// [Interaction Reminder](https://wiki.sponsor.ajay.app/w/Interaction_Reminder_(Subscribe))
	///
	/// When there is a short reminder to like, subscribe, or follow in the
	/// middle of content.
	InteractionReminder,

	/// [Highlight](https://wiki.sponsor.ajay.app/w/Highlight)
	///
	/// For getting to the point or highlight of the video.
	Highlight,

	/// [Intermission/Intro Animation](https://wiki.sponsor.ajay.app/w/Intermission/Intro_Animation)
	///
	/// An interval without actual content. It could be a pause, static frame,
	/// or repeating animation.
	IntermissionIntroAnimation,

	/// [Endcards/Credits](https://wiki.sponsor.ajay.app/w/Endcards/Credits)
	///
	/// Credits, or when the YouTube endcards appear.
	EndcardsCredits,

	/// [Preview/Recap](https://wiki.sponsor.ajay.app/w/Preview/Recap)
	///
	/// A quick recap of previous episodes, or a preview of what's coming up
	/// later in the current video.
	PreviewRecap,

	/// [Non-Music](https://wiki.sponsor.ajay.app/w/Music:_Non-Music_Section)
	///
	/// Only for use in music videos. A section of the video with non-music
	/// content.
	NonMusic,

	/// [Filler Tangent](https://wiki.sponsor.ajay.app/w/Filler_Tangent)
	///
	/// Tangential scenes added only for filler or humor that are not required
	/// to understand the main content of the video.
	FillerTangent,

	/// [Exclusive Access](https://wiki.sponsor.ajay.app/w/Exclusive_Access)
	///
	/// Only used when the creator showcases a product, service or location that
	/// they've received free or subsidised access to in the video that cannot
	/// be completely removed by cuts.
	ExclusiveAccess,
}

impl<'de> Deserialize<'de> for Category {
	fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let category_string = String::deserialize(deserializer)?;
		convert_to_category(category_string.as_str()).map_err(D::Error::custom)
	}
}

impl Default for Category {
	fn default() -> Self {
		Self::Sponsor
	}
}

bitflags! {
	/// A struct for supplying the categories you want to look for in a video.
	#[repr(transparent)]
	pub struct AcceptedCategories: u32 {
		/// A convenience constant for having no accepted categories.
		const NONE = 0b0000_0000_0000;
		/// Sponsor - take a look at [`Category::Sponsor`] for more information.
		const SPONSOR = 0b0000_0000_0001;
		/// Unpaid/Self-Promotion - take a look at [`Category::UnpaidSelfPromotion`] for more information.
		const UNPAID_SELF_PROMOTION = 0b0000_0000_0010;
		/// Interaction Reminder - take a look at [`Category::InteractionReminder`] for more information.
		const INTERACTION_REMINDER = 0b0000_0000_0100;
		/// Highlight - take a look at [`Category::Highlight`] for more information.
		const HIGHLIGHT = 0b0000_0000_1000;
		/// Intermission/Intro Animation - take a look at [`Category::IntermissionIntroAnimation`] for more information.
		const INTERMISSION_INTRO_ANIMATION = 0b0000_0001_0000;
		/// Endcards/Credits - take a look at [`Category::EndcardsCredits`] for more information.
		const ENDCARDS_CREDITS = 0b0000_0010_0000;
		/// Preview/Recap - take a look at [`Category::PreviewRecap`] for more information.
		const PREVIEW_RECAP = 0b0000_0100_0000;
		/// Non-Music - take a look at [`Category::NonMusic`] for more information.
		const NON_MUSIC = 0b0000_1000_0000;
		/// Filler Tangent - take a look at [`Category::FillerTangent`] for more information.
		const FILLER_TANGENT = 0b0001_0000_0000;
		/// Exclusive Access - take a look at [`Category::ExclusiveAccess`] for more information.
		const EXCLUSIVE_ACCESS = 0b0010_0000_0000;
	}
}

impl Default for AcceptedCategories {
	fn default() -> Self {
		Self::all()
	}
}
