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
	/// A client-side error during communication with the API.
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

	// Serialization
	/// An error encountered when deserializing JSON data received from the API.
	///
	/// If encountering this, it's possible the library version you're using is
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

	// Out-of-date Library(?)
	/// The library doesn't recognize a value it received from the API.
	///
	/// If encountering this, it's likely the library version you're using is
	/// out of date with the API. If that's the case, please open an issue.
	#[error("received an unrecognized value of type '{r#type}' from the API: {value}")]
	UnknownValue {
		/// The value's category or type.
		r#type: String,
		/// The value that isn't recognized.
		value: String,
	},
}

/// An HTTP status code number.
pub type StatusCode = u16;

pub(crate) type Result<T> = std::result::Result<T, SponsorBlockError>;
