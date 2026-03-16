use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub static X_PRONOUNS_ACCEPTABLE: &str = "X-Pronouns-Acceptable";
pub static PRONOUNS_EN: &str = "she/her";

/// A Rocket [Fairing](https://rocket.rs/guide/v0.5/fairings/#fairings) that sets
/// [`X-Pronouns-Acceptable`](https://www.andrewyu.org/article/x-pronouns.html) on every response.
pub struct PronounsAcceptable;

#[rocket::async_trait]
impl Fairing for PronounsAcceptable {
	fn info(&self) -> Info {
		Info {
			name: "Acceptable Pronouns",
			kind: Kind::Response,
		}
	}

	async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
		response.set_raw_header(X_PRONOUNS_ACCEPTABLE, format!("en:{PRONOUNS_EN}"));
	}
}
