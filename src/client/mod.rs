//! The SponsorBlock client.

// Modules
mod admin;
mod user;
mod vip;

// Uses
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};

// Public Exports
pub use self::{admin::*, user::*, vip::*};

// Type Definitions
/// A video ID.
pub type VideoId = String;
/// The ref version of [`VideoId`] for use in functions.
pub type VideoIdSlice = str;
/// A public user ID. This value is a hash of the [`LocalUserId`] and is used
/// publicly.
pub type PublicUserId = String;
/// The ref version of [`PublicUserId`] for use in functions.
pub type PublicUserIdSlice = str;
/// A local/private user ID. This value should be kept private and treated like
/// a password.
pub type LocalUserId = String;
/// The ref version of [`LocalUserId`] for use in functions.
pub type LocalUserIdSlice = str;
/// A UUID for a segment, uniquely identifying it in the database.
pub type SegmentUuid = String;
/// The ref version of [`SegmentUuid`] for use in functions.
pub type SegmentUuidSlice = str;

/// The client for interfacing with SponsorBlock.
pub struct Client {
	// Internal
	http: ReqwestClient,

	// Config
	user_id: String,
	base_url: String,
	#[cfg(feature = "private_searches")]
	hash_prefix_length: u8,
	service: String,
}

impl Client {
	/// Creates a new instance of the client with default configuration values.
	#[must_use]
	pub fn new<S: Into<String>>(user_id: S) -> Self {
		ClientBuilder::new(user_id).build()
	}

	/// Creates a new instance of the [`ClientBuilder`].
	#[must_use]
	pub fn builder<S: Into<String>>(user_id: S) -> ClientBuilder {
		ClientBuilder::new(user_id)
	}
}

/// The builder for the [`Client`].
pub struct ClientBuilder {
	// Internal
	user_agent: String,

	// Config
	user_id: String,
	base_url: String,
	#[cfg(feature = "private_searches")]
	hash_prefix_length: u8,
	service: String,
}

impl ClientBuilder {
	/// The default API URL to use.
	pub const DEFAULT_BASE_URL: &'static str = "https://sponsor.ajay.app/api";
	/// The default hash prefix length.
	#[cfg(feature = "private_searches")]
	pub const DEFAULT_HASH_PREFIX_LENGTH: u8 = 4;
	/// The default service value to use.
	pub const DEFAULT_SERVICE: &'static str = "YouTube";
	/// The user agent used by the library for requests to the API.
	pub const DEFAULT_USER_AGENT: &'static str =
		concat!(env!("CARGO_PKG_NAME"), "-rs/", env!("CARGO_PKG_VERSION"));

	/// Creates a new instance of the struct, with default values for all
	/// configuration.
	#[must_use]
	pub fn new<S: Into<String>>(user_id: S) -> Self {
		Self {
			user_agent: Self::DEFAULT_USER_AGENT.to_owned(),
			user_id: user_id.into(),
			base_url: Self::DEFAULT_BASE_URL.to_owned(),
			#[cfg(feature = "private_searches")]
			hash_prefix_length: Self::DEFAULT_HASH_PREFIX_LENGTH,
			service: Self::DEFAULT_SERVICE.to_owned(),
		}
	}

	/// Builds the struct into an instance of [`Client`].
	#[must_use]
	pub fn build(&self) -> Client {
		Client {
			http: ReqwestClientBuilder::new()
				.user_agent(self.user_agent.clone())
				.build()
				.expect("unable to build the HTTP client"),
			user_id: self.user_id.clone(),
			base_url: self.base_url.clone(),
			#[cfg(feature = "private_searches")]
			hash_prefix_length: self.hash_prefix_length,
			service: self.service.clone(),
		}
	}

	/// Sets the base URL to access for the API. This includes the `/api` in
	/// official instances.
	///
	/// You should only have to change this if working with a different instance
	/// of the SponsorBlock database.
	pub fn base_url(&mut self, base_url: &str) -> &mut Self {
		self.base_url = base_url.trim_end_matches('/').to_owned();
		self
	}

	/// Sets the hash prefix length to use for private searches.
	///
	/// This is the number of characters of the hash sent to the server. Smaller
	/// values will in theory mean more potential matches will have to be sent
	/// by the API, but will provide more privacy.
	///
	/// # Panics
	/// Panics if not in the range `4 <= hash_prefix_length <= 32`.
	#[cfg(feature = "private_searches")]
	pub fn hash_prefix_length(&mut self, hash_prefix_length: u8) -> &mut Self {
		assert!(hash_prefix_length >= 4);
		assert!(hash_prefix_length <= 32);
		self.hash_prefix_length = hash_prefix_length;
		self
	}

	/// Sets the service value to use with the API.
	///
	/// See <https://wiki.sponsor.ajay.app/w/Types#Service> for more information.
	pub fn service(&mut self, service: &str) -> &mut Self {
		self.service = service.to_owned();
		self
	}
}
