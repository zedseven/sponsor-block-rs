//! API Constants and components for directly interfacing with the API.

use crate::{Action, ActionableSegmentKind, SponsorBlockError, SponsorBlockResult};

// The API names for categories
pub(crate) const SPONSOR_NAME: &str = "sponsor";
pub(crate) const UNPAID_SELF_PROMOTION_NAME: &str = "selfpromo";
pub(crate) const INTERACTION_REMINDER_NAME: &str = "interaction";
pub(crate) const HIGHLIGHT_NAME: &str = "poi_highlight";
pub(crate) const INTERMISSION_INTRO_ANIMATION_NAME: &str = "intro";
pub(crate) const ENDCARDS_CREDITS_NAME: &str = "outro";
pub(crate) const PREVIEW_RECAP_NAME: &str = "preview";
pub(crate) const NON_MUSIC_NAME: &str = "music_offtopic";

// The API names for actions
pub(crate) const ACTION_SKIP_NAME: &str = "skip";
pub(crate) const ACTION_MUTE_NAME: &str = "mute";

// API value conversion functions. The goal here is to make it so everything
// else in the library need not interface with raw category names.
pub(crate) fn api_convert_segment_kind(
	category: &str,
) -> SponsorBlockResult<ActionableSegmentKind> {
	match category {
		SPONSOR_NAME => Ok(ActionableSegmentKind::Sponsor),
		UNPAID_SELF_PROMOTION_NAME => Ok(ActionableSegmentKind::UnpaidSelfPromotion),
		INTERACTION_REMINDER_NAME => Ok(ActionableSegmentKind::InteractionReminder),
		HIGHLIGHT_NAME => Ok(ActionableSegmentKind::Highlight),
		INTERMISSION_INTRO_ANIMATION_NAME => Ok(ActionableSegmentKind::IntermissionIntroAnimation),
		ENDCARDS_CREDITS_NAME => Ok(ActionableSegmentKind::EndcardsCredits),
		PREVIEW_RECAP_NAME => Ok(ActionableSegmentKind::PreviewRecap),
		NON_MUSIC_NAME => Ok(ActionableSegmentKind::NonMusic),
		unknown_value => Err(SponsorBlockError::UnknownValue {
			r#type: "category".to_owned(),
			value: unknown_value.to_owned(),
		}),
	}
}

pub(crate) fn api_convert_action_type(action_type: &str) -> SponsorBlockResult<Action> {
	match action_type {
		ACTION_SKIP_NAME => Ok(Action::Skip),
		ACTION_MUTE_NAME => Ok(Action::Mute),
		unknown_value => Err(SponsorBlockError::UnknownValue {
			r#type: "actionType".to_owned(),
			value: unknown_value.to_owned(),
		}),
	}
}
