use crate::utils::random_name;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub const X_CLACKS_OVERHEAD: &'static str = "X-Clacks-Overhead";

/// A Rocket [Fairing](https://rocket.rs/guide/v0.5/fairings/#fairings) that sets
/// [`X-Clacks-Overhead`](https://xclacksoverhead.org/home/about) on every response.
pub struct Clacks;

#[rocket::async_trait]
impl Fairing for Clacks {
	fn info(&self) -> Info {
		Info {
			name: "Clacks Overhead",
			kind: Kind::Response,
		}
	}

	async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
		let name = random_name();

		response.set_raw_header(X_CLACKS_OVERHEAD, format!("GNU {name}"));
	}
}
