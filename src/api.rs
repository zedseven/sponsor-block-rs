//! API Constants and components for directly interfacing with the API.

use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use serde::de::{Error as DeserializationError, StdError};
use thiserror::Error;

use crate::{
	util::to_url_array_conditional_convert,
	AcceptedCategories,
	Action,
	ActionableSegmentKind,
	SponsorBlockError,
	SponsorBlockResult,
};

// The API names for categories
const SPONSOR_NAME: &str = "sponsor";
const UNPAID_SELF_PROMOTION_NAME: &str = "selfpromo";
const INTERACTION_REMINDER_NAME: &str = "interaction";
const HIGHLIGHT_NAME: &str = "poi_highlight";
const INTERMISSION_INTRO_ANIMATION_NAME: &str = "intro";
const ENDCARDS_CREDITS_NAME: &str = "outro";
const PREVIEW_RECAP_NAME: &str = "preview";
const NON_MUSIC_NAME: &str = "music_offtopic";

// The API names for actions
const ACTION_SKIP_NAME: &str = "skip";
const ACTION_MUTE_NAME: &str = "mute";

/// A value received from the API is not recognized.
///
/// If encountering this, it's likely the library version is out of date with
/// the API.
#[derive(Error, Debug)]
#[error("received an unrecognized value of type '{r#type}' from the API: {value}")]
pub(crate) struct UnknownValueError {
	/// The value's category or type.
	r#type: String,
	/// The value that isn't recognized.
	value: String,
}

// API value conversion functions. The goal here is to make it so everything
// else in the library need not interface with raw category names.
pub(crate) fn convert_to_segment_kind(
	category: &str,
) -> Result<ActionableSegmentKind, UnknownValueError> {
	match category {
		SPONSOR_NAME => Ok(ActionableSegmentKind::Sponsor),
		UNPAID_SELF_PROMOTION_NAME => Ok(ActionableSegmentKind::UnpaidSelfPromotion),
		INTERACTION_REMINDER_NAME => Ok(ActionableSegmentKind::InteractionReminder),
		HIGHLIGHT_NAME => Ok(ActionableSegmentKind::Highlight),
		INTERMISSION_INTRO_ANIMATION_NAME => Ok(ActionableSegmentKind::IntermissionIntroAnimation),
		ENDCARDS_CREDITS_NAME => Ok(ActionableSegmentKind::EndcardsCredits),
		PREVIEW_RECAP_NAME => Ok(ActionableSegmentKind::PreviewRecap),
		NON_MUSIC_NAME => Ok(ActionableSegmentKind::NonMusic),
		unknown_value => Err(UnknownValueError {
			r#type: "category".to_owned(),
			value: unknown_value.to_owned(),
		}),
	}
}

pub(crate) fn convert_to_action_type(action_type: &str) -> Result<Action, UnknownValueError> {
	match action_type {
		ACTION_SKIP_NAME => Ok(Action::Skip),
		ACTION_MUTE_NAME => Ok(Action::Mute),
		unknown_value => Err(UnknownValueError {
			r#type: "actionType".to_owned(),
			value: unknown_value.to_owned(),
		}),
	}
}

pub(crate) fn convert_category_bitflags_to_url(accepted_categories: AcceptedCategories) -> String {
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

	to_url_array_conditional_convert(
		CATEGORY_PAIRS,
		|&(flag, _)| accepted_categories.contains(flag),
		|&(_, name)| name,
	)
}
