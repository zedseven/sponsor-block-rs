//! Everything associated with library errors.

// Uses
use thiserror::Error;

/// The library error type.
#[derive(Error, Debug)]
pub enum SponsorBlockError {
	// HTTP-Related Error Types
	/// An internal server error with the API.
	///
	/// Contains the status code returned by the server.
	#[error("internal API error, with status code {0}")]
	HttpApi(StatusCode),
	/// A client-side error during communication with the API. A value of 404
	/// simply means no segments could be found in the database for the video ID
	/// you requested.
	///
	/// If encountering this, it's possible the library version you're using is
	/// out of date with the API. If that's the case, please open an issue.
	///
	/// Contains the status code returned by the server.
	#[error("client HTTP error, with status code {0}")]
	HttpClient(StatusCode),
	/// An unknown error during communication with the API.
	///
	/// Contains the status code returned by the server.
	#[error("unknown HTTP error, with status code {0}")]
	HttpUnknown(StatusCode),
	/// An actual communication error. Likely a network or protocol issue.
	/// Contains the internal [`reqwest::Error`].
	#[error("unable to communicate with the API")]
	HttpCommunication(#[from] reqwest::Error),

	// Other API Errors
	/// The API does not have any segments in the database for the requested
	/// video ID.
	///
	/// This should effectively be treated the same way as an
	/// [`HttpClient(404)`].
	///
	/// [`HttpClient(404)`]: crate::SponsorBlockError::HttpClient
	#[cfg(feature = "private_searches")]
	#[error("unable to find a matching hash for the provided video ID")]
	NoMatchingVideoHash,

	// Serialization
	/// An error encountered when deserializing JSON data received from the API.
	///
	/// If encountering this, it's likely the library version you're using is
	/// out of date with the API. If that's the case, please open an issue.
	///
	/// Contains the internal [`serde_json::Error`].
	#[error("unable to deserialize data from the API")]
	Deserialization(#[from] serde_json::Error),

	// Data Verification
	/// Data received from the API does not make sense or fails to meet sanity
	/// requirements.
	#[error("data received from the API does not meet verification: {0}")]
	BadData(String),
}

/// An HTTP status code number.
pub type StatusCode = u16;

pub(crate) type SponsorBlockResult<T> = Result<T, SponsorBlockError>;
