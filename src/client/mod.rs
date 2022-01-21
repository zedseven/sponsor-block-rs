//! The SponsorBlock client.

// Modules
#[cfg(feature = "user")]
mod user;
#[cfg(feature = "vip")]
mod vip;

// Uses
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};
use time::Duration;

// Public Exports
#[cfg(feature = "user")]
pub use self::user::*;
#[cfg(feature = "vip")]
pub use self::vip::*;

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
	pub fn new<U>(user_id: U) -> Self
	where
		U: Into<String>,
	{
		ClientBuilder::new(user_id).build()
	}

	/// Creates a new instance of the [`ClientBuilder`].
	#[must_use]
	pub fn builder<U>(user_id: U) -> ClientBuilder
	where
		U: Into<String>,
	{
		ClientBuilder::new(user_id)
	}
}

/// The builder for the [`Client`].
#[derive(Clone)]
pub struct ClientBuilder {
	// Internal
	user_agent: String,

	// Config
	user_id: String,
	base_url: String,
	#[cfg(feature = "private_searches")]
	hash_prefix_length: u8,
	service: String,
	timeout: Option<Duration>,
}

impl ClientBuilder {
	/// The base URL for the official SponsorBlock API.
	///
	/// This is the default value.
	///
	/// See [`base_url`] for more information.
	///
	/// [`base_url`]: Self::base_url
	pub const BASE_URL_MAIN: &'static str = "https://sponsor.ajay.app/api";
	/// The base URL for the SponsorBlock testing database.
	///
	/// See [`base_url`] for more information.
	///
	/// [`base_url`]: Self::base_url
	pub const BASE_URL_TESTING: &'static str = "https://sponsor.ajay.app/test/api";
	/// The default hash prefix length.
	///
	/// See [`hash_prefix_length`] for more information.
	///
	/// [`hash_prefix_length`]: Self::hash_prefix_length
	#[cfg(feature = "private_searches")]
	pub const DEFAULT_HASH_PREFIX_LENGTH: u8 = 4;
	/// The default service value to use.
	///
	/// See [`service`] for more information.
	///
	/// [`service`]: Self::service
	pub const DEFAULT_SERVICE: &'static str = "YouTube";
	/// The user agent used by the library for requests to the API.
	pub const DEFAULT_USER_AGENT: &'static str =
		concat!(env!("CARGO_PKG_NAME"), "-rs/", env!("CARGO_PKG_VERSION"));
	/// The default HTTP request timeout.
	///
	/// See [`timeout`] for more information.
	///
	/// [`timeout`]: Self::timeout
	pub const DEFAULT_TIMEOUT: Duration = Duration::seconds(5);

	/// Creates a new instance of the struct, with default values for all
	/// configuration.
	#[must_use]
	pub fn new<U>(user_id: U) -> Self
	where
		U: Into<String>,
	{
		Self {
			user_agent: Self::DEFAULT_USER_AGENT.to_owned(),
			user_id: user_id.into(),
			base_url: Self::BASE_URL_MAIN.to_owned(),
			#[cfg(feature = "private_searches")]
			hash_prefix_length: Self::DEFAULT_HASH_PREFIX_LENGTH,
			service: Self::DEFAULT_SERVICE.to_owned(),
			timeout: Some(Self::DEFAULT_TIMEOUT),
		}
	}

	/// Builds the struct into an instance of [`Client`].
	///
	/// # Panics
	/// - If the underlying HTTP client fails to build for some reason.
	/// - If the timeout is of a value that is incompatible with the std
	///   library.
	///
	/// If either happens, please open an issue.
	#[must_use]
	pub fn build(&self) -> Client {
		let mut http = ReqwestClientBuilder::new().user_agent(self.user_agent.clone());
		if let Some(timeout) = self.timeout {
			http = http.timeout(timeout.try_into().expect(
				"the Duration value provided for the HTTP timeout is incompatible with the std \
				 library implementation",
			));
		}
		Client {
			http: http.build().expect("unable to build the HTTP client"),
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
	///
	/// The default value is [`BASE_URL_MAIN`].
	///
	/// [`BASE_URL_MAIN`]: Self::BASE_URL_MAIN
	pub fn base_url<U>(&mut self, base_url: U) -> &mut Self
	where
		U: AsRef<str>,
	{
		self.base_url = base_url.as_ref().trim_end_matches('/').to_owned();
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
	pub fn service<S>(&mut self, service: S) -> &mut Self
	where
		S: Into<String>,
	{
		self.service = service.into();
		self
	}

	/// Sets the HTTP request timeout.
	///
	/// The timeout is applied from when the request starts connecting until the
	/// response body has finished.
	///
	/// The default is no timeout.
	///
	/// # Panics
	/// Panics if not in the range `duration > 0`.
	pub fn timeout(&mut self, duration: Option<Duration>) -> &mut Self {
		if let Some(duration_value) = duration {
			assert!(duration_value.is_positive());
		}

		self.timeout = duration;
		self
	}

	/// Sets the HTTP request timeout, in milliseconds.
	///
	/// The timeout is applied from when the request starts connecting until the
	/// response body has finished.
	///
	/// The default is no timeout.
	///
	/// # Panics
	/// Panics if not in the range `millis > 0`.
	pub fn timeout_millis(&mut self, millis: Option<i64>) -> &mut Self {
		self.timeout(millis.map(Duration::milliseconds));
		self
	}
}
