//! Additional interface for segment categories.

use std::collections::HashMap;

// Uses
use bitflags::bitflags;

use crate::{
	api::{
		ENDCARDS_CREDITS_NAME,
		HIGHLIGHT_NAME,
		INTERACTION_REMINDER_NAME,
		INTERMISSION_INTRO_ANIMATION_NAME,
		NON_MUSIC_NAME,
		PREVIEW_RECAP_NAME,
		SPONSOR_NAME,
		UNPAID_SELF_PROMOTION_NAME,
	},
	ActionableSegmentKind,
};

bitflags! {
	/// A struct for supplying the categories you want to look for in a video.
	pub struct AcceptedCategories: u32 {
		/// A convenience constant for having no accepted categories.
		const NONE = 0b0000_0000;
		/// Sponsor - take a look at [`crate::ActionableSegment::Sponsor`] for more information.
		const SPONSOR = 0b0000_0001;
		/// Unpaid/Self-Promotion - take a look at [`crate::ActionableSegment::UnpaidSelfPromotion`] for more information.
		const UNPAID_SELF_PROMOTION = 0b0000_0010;
		/// Interaction Reminder - take a look at [`crate::ActionableSegment::InteractionReminder`] for more information.
		const INTERACTION_REMINDER = 0b0000_0100;
		/// Highlight - take a look at [`crate::ActionableSegment::Highlight`] for more information.
		const HIGHLIGHT = 0b0000_1000;
		/// Intermission/Intro Animation - take a look at [`crate::ActionableSegment::IntermissionIntroAnimation`] for more information.
		const INTERMISSION_INTRO_ANIMATION = 0b0001_0000;
		/// Endcards/Credits - take a look at [`crate::ActionableSegment::EndcardsCredits`] for more information.
		const ENDCARDS_CREDITS = 0b0010_0000;
		/// Preview/Recap - take a look at [`crate::ActionableSegment::PreviewRecap`] for more information.
		const PREVIEW_RECAP = 0b0100_0000;
		/// Non-Music - take a look at [`crate::ActionableSegment::NonMusic`] for more information.
		const NON_MUSIC = 0b1000_0000;
	}
}

impl AcceptedCategories {
	pub(crate) fn gen_url_value(self) -> String {
		/// Maps category values to their API names according to https://github.com/ajayyy/SponsorBlock/wiki/Types
		const CATEGORY_PAIRS: &[(AcceptedCategories, &str)] = &[
			(AcceptedCategories::SPONSOR, SPONSOR_NAME),
			(
				AcceptedCategories::UNPAID_SELF_PROMOTION,
				UNPAID_SELF_PROMOTION_NAME,
			),
			(
				AcceptedCategories::INTERACTION_REMINDER,
				INTERACTION_REMINDER_NAME,
			),
			(AcceptedCategories::HIGHLIGHT, HIGHLIGHT_NAME),
			(
				AcceptedCategories::INTERMISSION_INTRO_ANIMATION,
				INTERMISSION_INTRO_ANIMATION_NAME,
			),
			(AcceptedCategories::ENDCARDS_CREDITS, ENDCARDS_CREDITS_NAME),
			(AcceptedCategories::PREVIEW_RECAP, PREVIEW_RECAP_NAME),
			(AcceptedCategories::NON_MUSIC, NON_MUSIC_NAME),
		];

		let mut result = String::from('[');

		let mut pushed_already = false;
		for &(flag, name) in CATEGORY_PAIRS.iter() {
			if self.contains(flag) {
				if pushed_already {
					result.push(',');
				}

				result.push('"');
				result.push_str(name);
				result.push('"');

				pushed_already = true;
			}
		}

		result.push(']');
		result
	}
}

impl Default for AcceptedCategories {
	fn default() -> Self {
		Self::all()
	}
}
