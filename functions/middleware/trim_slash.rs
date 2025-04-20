use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::ext::IntoOwned;
use rocket::http::{Header, Status};
use rocket::{Request, Response};
use std::io::Cursor;

/// A Rocket [Fairing](https://rocket.rs/guide/v0.5/fairings/#fairings) that redirects
/// requests with a trailing slash to one without.
pub struct TrimSlash;

#[rocket::async_trait]
impl Fairing for TrimSlash {
	fn info(&self) -> Info {
		Info {
			name: "Trim Trailing Slashes",
			kind: Kind::Response,
		}
	}

	async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
		if req.uri().path().ends_with("/") && req.uri().path() != "/" {
			let new_path = req
				.uri()
				.map_path(|p| p.strip_suffix("/").expect("Path should end with /"))
				.expect("Removing a trailing slash from a known good path results in a valid path")
				.into_owned();

			response.set_status(Status::MovedPermanently);
			response.set_header(Header::new("Location", new_path.to_string()));
			response.set_sized_body(0, Cursor::new(""));
		}
	}
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;
	use rocket::local::blocking::{Client, LocalResponse};

	fn build_client() -> Client {
		let service = rocket::build().attach(TrimSlash);
		Client::tracked(service).expect("Test client should launch")
	}

	fn get<'r>(client: &'r Client, path: &'static str) -> LocalResponse<'r> {
		let req = client.get(path);
		req.dispatch()
	}

	#[test]
	fn test_fairing_does_nothing_for_valid_paths() {
		let cases = vec![
			"/", //
			"/foo",
			"/foo/bar",
			"/foo/bar/baz",
			"/foo/bar/42",
		];

		let client = build_client();
		for path in cases {
			let res = get(&client, path);
			assert_eq!(res.status(), Status::NotFound);
		}
		client.terminate();
	}

	#[test]
	fn test_fairing_redirects_trailing_slashes_appropriately() {
		let cases = vec![
			("/foo/", "/foo"),
			("/foo/bar/", "/foo/bar"),
			("/foo/bar/baz/", "/foo/bar/baz"),
			("/foo/bar/42/", "/foo/bar/42"),
		];

		let client = build_client();
		for (path, dest) in cases {
			let res = get(&client, path);
			assert_eq!(res.status(), Status::MovedPermanently);
			assert_eq!(res.headers().get_one("Location"), Some(dest));
		}
		client.terminate();
	}
}
