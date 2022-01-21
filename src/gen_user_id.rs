// Uses
use rand::{
	distributions::{Distribution, Uniform},
	thread_rng,
};

/// A utility function that generates a new local user ID.
///
/// *Do not* call this every time you start up a client - prefer using a single
/// saved ID for the same 'user', and treat it like a password. Store it outside
/// of runtime if necessary. This function is for cases where you may want to
/// generate new user IDs for users of your application, giving each user their
/// own ID.
///
/// This function is based directly on
/// [how the official extension does it](https://github.com/ajayyy/SponsorBlock/blob/a9e43f95f51dbf7f3517a0cb6956397fbe2b622f/src/utils.ts#L299).
#[must_use]
pub fn gen_user_id() -> String {
	const LENGTH: usize = 36;
	const CHAR_SET: &[char] = &[
		'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
		'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
		'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
		'2', '3', '4', '5', '6', '7', '8', '9',
	];

	let mut result = String::with_capacity(LENGTH);
	let uniform = Uniform::from(0..CHAR_SET.len());
	let mut rng = thread_rng();
	for _ in 0..LENGTH {
		result.push(CHAR_SET[uniform.sample(&mut rng)]);
	}

	result
}
