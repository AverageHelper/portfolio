use rand::{Rng, seq::IndexedRandom};

/// A list of names of people to memorialize in X-Clacks-Overhead
const NAMES: &[&str] = &[
	"Terry Pratchett", // 28 April 1948 - 12 March 2015
	"Nex Benedict",    // 11 January 2008 - February 8, 2024
];

/// Returns a random memorial name.
pub fn random_name<R: Rng>(rng: &mut R) -> &'static str {
	NAMES.choose(rng).expect("Names array should not be empty")
}
