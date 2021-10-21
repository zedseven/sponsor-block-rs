//! Utility functions that don't necessarily fit into a neat category but assist
//! with the process.

// Uses
use reqwest::Response;

use crate::error::{Result, SponsorBlockError};

/// Parses the [`Response`] and categorizes errors depending on their source.
pub(crate) async fn get_response_text(response: Response) -> Result<String> {
	let status = response.status();
	if status.is_success() {
		Ok(response.text().await?)
	} else if status.is_server_error() {
		Err(SponsorBlockError::HttpApi(status.as_u16()))
	} else if status.is_client_error() {
		Err(SponsorBlockError::HttpClient(status.as_u16()))
	} else {
		Err(SponsorBlockError::HttpUnknown(status.as_u16()))
	}
}
