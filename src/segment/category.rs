//! Additional interface for segment categories.

// Uses
use bitflags::bitflags;

bitflags! {
	/// A struct for supplying the categories you want to look for in a video.
	pub struct AcceptedCategories: u32 {
		/// A convenience constant for having no accepted categories.
		const NONE = 0b0000_0000_0000;
		/// Sponsor - take a look at [`crate::ActionableSegment::Sponsor`] for more information.
		const SPONSOR = 0b0000_0000_0001;
		/// Unpaid/Self-Promotion - take a look at [`crate::ActionableSegment::UnpaidSelfPromotion`] for more information.
		const UNPAID_SELF_PROMOTION = 0b0000_0000_0010;
		/// Interaction Reminder - take a look at [`crate::ActionableSegment::InteractionReminder`] for more information.
		const INTERACTION_REMINDER = 0b0000_0000_0100;
		/// Highlight - take a look at [`crate::ActionableSegment::Highlight`] for more information.
		const HIGHLIGHT = 0b0000_0000_1000;
		/// Intermission/Intro Animation - take a look at [`crate::ActionableSegment::IntermissionIntroAnimation`] for more information.
		const INTERMISSION_INTRO_ANIMATION = 0b0000_0001_0000;
		/// Endcards/Credits - take a look at [`crate::ActionableSegment::EndcardsCredits`] for more information.
		const ENDCARDS_CREDITS = 0b0000_0010_0000;
		/// Preview/Recap - take a look at [`crate::ActionableSegment::PreviewRecap`] for more information.
		const PREVIEW_RECAP = 0b0000_0100_0000;
		/// Non-Music - take a look at [`crate::ActionableSegment::NonMusic`] for more information.
		const NON_MUSIC = 0b0000_1000_0000;
		/// Filler Tangent - take a look at [`crate::ActionableSegment::FillerTangent`] for more information.
		const FILLER_TANGENT = 0b0001_0000_0000;
		/// Exclusive Access - take a look at [`crate::ActionableSegment::ExclusiveAccess`] for more information.
		const EXCLUSIVE_ACCESS = 0b0010_0000_0000;
	}
}

impl Default for AcceptedCategories {
	fn default() -> Self {
		Self::all()
	}
}
