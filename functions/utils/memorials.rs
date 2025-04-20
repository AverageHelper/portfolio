use rand::seq::SliceRandom;

/// A list of names of people to memorialize in X-Clacks-Overhead
const NAMES: &'static [&'static str] = &[
	"Terry Pratchett", // 28 April 1948 - 12 March 2015
	"Nex Benedict",    // 11 January 2008 - February 8, 2024
];

/// Returns a random memorial name.
pub fn random_name() -> &'static str {
	*NAMES
		.choose(&mut rand::thread_rng())
		.expect("Names array should not be empty")
}
