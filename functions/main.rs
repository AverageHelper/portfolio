mod capsule;
mod config;
mod factories;
mod middleware;
mod utils;

use capsule::gemini_service;
use config::Config;
use factories::{UserAgent, WebFinger};
use include_dir::{Dir, include_dir};
use middleware::{Clacks, ExtraSecurityHeaders, PronounsAcceptable};
use middleware::{CorsAllowAllResponse, CorsOnlyProdResponse, PRONOUNS_EN, TrimSlash, shield};
use rocket::http::{ContentType, Status};
use rocket::response::content::RawJson;
use rocket::response::status::BadRequest;
use rocket::response::{Redirect, content::RawHtml, status::NotFound};
use rocket::{Build, Rocket, catch, catchers, get, routes, uri};
use rocket_async_compression::CachedCompression;
use std::ffi::OsStr;
use std::path::PathBuf;

// MARK: - Routes

/// Favicon is always not found
#[get("/favicon.ico")]
fn favicon() -> NotFound<&'static str> {
	NotFound("")
}

#[get("/ip")]
fn ip() -> Redirect {
	Redirect::found(uri!("https://ip.average.name"))
}

// MARK: Ways

#[get("/how")]
fn how() -> Redirect {
	Redirect::found(uri!("/ways"))
}

#[get("/how.html")]
fn how_html() -> Redirect {
	Redirect::found(uri!("/ways.html"))
}

// MARK: Links

#[get("/bookmarks")]
fn bookmarks() -> Redirect {
	Redirect::found(uri!("/links"))
}

#[get("/bookmarks.html")]
fn bookmarks_html() -> Redirect {
	Redirect::found(uri!("/links.html"))
}

// MARK: Pronouns

#[get("/pronouns")]
fn pronouns() -> Redirect {
	Redirect::found(uri!(well_known_pronouns()))
}

#[get("/.well-known/pronouns")]
fn well_known_pronouns() -> CorsAllowAllResponse<&'static str> {
	CorsAllowAllResponse(PRONOUNS_EN)
}

// MARK: Fursona

#[get("/fursona.json")]
fn root_fursona_json() -> Redirect {
	Redirect::found(uri!(fursona()))
}

#[get("/.well-known/fursona")]
fn well_known_fursona() -> Redirect {
	Redirect::found(uri!(fursona()))
}

const AVATAR_IMAGE: &'static [u8] = include_bytes!("../dist/images/refs/AverageHelper-avatar.png");
const FURSONA_JSON: &'static str = include_str!("../dist/.well-known/fursona.json");

/// Serves my fursona avatar image, without CORS so external readers can access the file.
#[get("/images/refs/AverageHelper-avatar.png")]
fn avatar() -> CorsAllowAllResponse<(ContentType, &'static [u8])> {
	CorsAllowAllResponse((ContentType::PNG, AVATAR_IMAGE))
}

/// Serves fursona.json, without CORS so external readers can access the file.
#[get("/.well-known/fursona.json")]
fn fursona() -> CorsAllowAllResponse<RawJson<&'static str>> {
	CorsAllowAllResponse(RawJson(FURSONA_JSON))
}

// MARK: Fediverse aliases

#[get("/@avg")]
fn at_avg() -> Redirect {
	Redirect::found(uri!("https://gts.average.name/@avghelper"))
}

#[get("/@avghelper")]
fn at_avghelper() -> Redirect {
	Redirect::found(uri!("https://gts.average.name/@avghelper"))
}

#[get("/@average")]
fn at_average() -> Redirect {
	Redirect::found(uri!("https://gts.average.name/@avghelper"))
}

#[get("/.well-known/webfinger?<resource>&<rel>")]
fn webfinger<'r>(
	resource: Option<&'r str>,
	rel: Option<Vec<&'r str>>,
) -> Result<WebFinger<'r>, Status> {
	match resource {
		None => Err(Status::BadRequest),
		Some(resource) => factories::webfinger(resource, rel),
	}
}

#[get("/.well-known/nodeinfo")]
fn nodeinfo(user_agent: UserAgent<'_>) -> Result<Redirect, Status> {
	factories::nodeinfo(user_agent)
}

// MARK: On-demand TLS

/// Handles Caddy's on-demand TLS requests.
#[get("/.well-known/domains?<domain>")]
fn on_demand_tls(domain: Option<&str>) -> (Status, ()) {
	match domain {
		None => (Status::BadRequest, ()),
		Some(domain) => factories::on_demand_tls(domain),
	}
}

// MARK: /dist

static DIST: Dir = include_dir!("dist");
const ROOT: &'static str = include_str!("../dist/index.html");

#[get("/")]
fn root() -> CorsOnlyProdResponse<RawHtml<&'static str>> {
	CorsOnlyProdResponse(RawHtml(ROOT))
}

#[get("/<path..>")]
fn dist(path: PathBuf) -> CorsOnlyProdResponse<Option<(ContentType, &'static [u8])>> {
	let mut path = path;

	// If a directory, try adding .html and see if that exists.
	if DIST.get_dir(&path).is_some() {
		let adjacent_html = path.with_extension("html");
		let inner_html = path.join("index.html");
		if DIST.contains(&adjacent_html) {
			path = adjacent_html;
		} else if DIST.contains(&inner_html) {
			path = inner_html;
		}
	} else if !DIST.contains(&path) {
		let as_html = path.with_extension("html");
		if DIST.contains(&as_html) {
			path = as_html;
		}
	}

	let res = match DIST.get_file(&path) {
		None => None,
		Some(asset) => {
			let content_type = path
				.extension()
				.and_then(OsStr::to_str)
				.and_then(ContentType::from_extension)
				.unwrap_or(ContentType::Bytes);
			Some((content_type, asset.contents()))
		}
	};
	CorsOnlyProdResponse(res)
}

// MARK: - Service

#[catch(400)]
fn bad_request() -> BadRequest<&'static str> {
	BadRequest("Bad Request")
}

#[catch(404)]
fn not_found() -> NotFound<RawHtml<&'static str>> {
	let not_found = include_str!("../dist/404.html");
	NotFound(RawHtml(not_found))
}

fn http_service(config: &Config) -> Rocket<Build> {
	let config = config.rocket_config();

	let suffixes = vec![".css", ".html", ".svg", ".xml", ".txt"]
		.iter()
		.map(ToString::to_string)
		.collect();

	rocket::build()
		.configure(config)
		.attach(CachedCompression::path_suffix_fairing(suffixes))
		.attach(TrimSlash)
		.attach(shield())
		.attach(ExtraSecurityHeaders)
		.attach(Clacks)
		.attach(PronounsAcceptable)
		.mount(
			"/",
			routes![
				favicon,
				ip,
				how,
				how_html,
				bookmarks,
				bookmarks_html,
				pronouns,
				well_known_pronouns,
				root_fursona_json,
				well_known_fursona,
				avatar,
				fursona,
				at_avg,
				at_avghelper,
				at_average,
				webfinger,
				nodeinfo,
				on_demand_tls,
				root,
				dist,
			],
		)
		.register("/", catchers![bad_request, not_found])
}

async fn start_gemini_service(config: &Config) {
	if let Err(err) = gemini_service(config).await {
		eprintln!("{err}");
		std::process::exit(1);
	}
}

async fn start_http_service(config: &Config) {
	let rocket = match http_service(&config).ignite().await {
		Err(err) => {
			eprintln!("{err}");
			std::process::exit(1);
		}
		Ok(rocket) => rocket,
	};
	println!("HTTP: Serving on port {}", config.http_port);
	if let Err(err) = rocket.launch().await {
		eprintln!("{err}");
		std::process::exit(1);
	}
}

#[rocket::main]
async fn main() {
	let config = Config::default();

	// Run both webservers at once, and kill the other when one dies:
	tokio::select! {
		_ = start_gemini_service(&config) => {},
		_ = start_http_service(&config) => {},
	};
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;
	use http::header;
	use middleware::{X_CLACKS_OVERHEAD, X_PRONOUNS_ACCEPTABLE};
	use rocket::{
		http::{ContentType, Header, MediaType},
		local::blocking::{Client, LocalResponse},
	};

	fn assert_header(res: &LocalResponse, key: &str, value: &str) {
		assert_eq!(
			res.headers()
				.get_one(key)
				.expect(&format!("{key} value should exist")),
			value
		);
	}

	fn assert_headers(res: &LocalResponse) {
		assert!(res.headers().contains(header::CONTENT_SECURITY_POLICY));
		assert!(
			res.headers()
				.get_one(header::CONTENT_SECURITY_POLICY.as_str())
				.expect("Content-Security-Policy value should exist")
				.contains("upgrade-insecure-requests")
		);
		assert_header(&res, header::REFERRER_POLICY.as_str(), "no-referrer");
		assert!(res.headers().contains(header::STRICT_TRANSPORT_SECURITY));
		assert!(res.headers().contains(header::X_CONTENT_TYPE_OPTIONS));
		assert!(res.headers().contains(header::X_FRAME_OPTIONS));
		assert!(res.headers().contains(X_CLACKS_OVERHEAD));
		assert!(res.headers().contains(X_PRONOUNS_ACCEPTABLE));
	}

	fn assert_cors(res: &LocalResponse, allowed_origin: &'static str) {
		let actual_allowed_origin = res
			.headers()
			.get_one(header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str());
		assert_eq!(actual_allowed_origin, Some(allowed_origin));
	}

	fn assert_redir(res: &LocalResponse, location: &str) {
		assert_eq!(
			res.headers()
				.get_one(header::LOCATION.as_str())
				.expect("Location header should be present"),
			location
		);
	}

	fn assert_status(res: &LocalResponse, status_code: Status) {
		assert_eq!(res.status(), status_code, "Status should be {status_code}")
	}

	fn assert_content_type(res: &LocalResponse, content_type: ContentType) {
		assert_eq!(res.content_type(), Some(content_type));
	}

	fn assert_file_contents_match(response_contents: String, expected_file_path: &str) {
		let file_contents =
			std::fs::read_to_string(expected_file_path).expect("File should exist and be readable");
		assert_eq!(response_contents, file_contents);
	}

	fn assert_file_bytes_match(response_contents: Vec<u8>, expected_file_path: &str) {
		let file_contents =
			std::fs::read(expected_file_path).expect("File should exist and be readable");
		assert_eq!(response_contents, file_contents);
	}

	fn build_client() -> Client {
		let config = Config::default();
		Client::tracked(http_service(&config)).expect("Test client should launch")
	}

	struct Origin(&'static str);
	impl Into<Header<'static>> for Origin {
		fn into(self) -> Header<'static> {
			Header::new(header::ORIGIN.as_str(), self.0)
		}
	}

	struct UserAgent(String);
	impl Into<Header<'static>> for UserAgent {
		fn into(self) -> Header<'static> {
			Header::new(header::USER_AGENT.as_str(), self.0)
		}
	}

	fn get<'r>(client: &'r Client, path: &'static str) -> LocalResponse<'r> {
		let req = client.get(path).header(Origin("https://average.name"));
		req.dispatch()
	}
	fn get_with_origin<'r>(
		client: &'r Client,
		path: &'static str,
		origin: Option<&'static str>,
	) -> LocalResponse<'r> {
		let mut req = client.get(path);
		if let Some(origin) = origin {
			req = req.header(Origin(origin))
		}
		req.dispatch()
	}
	fn get_with_user_agent<'r>(
		client: &'r Client,
		path: &'r str,
		user_agent: &'r str,
	) -> LocalResponse<'r> {
		let req = client.get(path).header(UserAgent(user_agent.to_string()));
		req.dispatch()
	}
	fn head<'r>(client: &'r Client, path: &'r str) -> LocalResponse<'r> {
		let req = client.head(path);
		req.dispatch()
	}

	fn response_bytes(res: LocalResponse<'_>) -> Vec<u8> {
		let bytes = res.into_bytes().expect("Body should download safely");
		let hundred_mb = 100_000_000;
		assert!(bytes.len() < hundred_mb, "Body should be under 100 MB");
		return bytes;
	}

	fn response_body(res: LocalResponse) -> String {
		let body_bytes = response_bytes(res);
		String::from_utf8(body_bytes.to_vec()).expect("Body should be valid UTF-8")
	}

	#[test]
	fn test_answers_favicon() {
		let client = build_client();
		let res = get(&client, "/favicon.ico");
		assert_status(&res, Status::NotFound);
		assert_headers(&res);
		let body = response_body(res);
		assert!(body.is_empty());
		client.terminate();
	}

	#[test]
	fn test_answers_pronouns() {
		let client = build_client();
		let res = get(&client, "/.well-known/pronouns");
		assert_status(&res, Status::Ok);
		assert_headers(&res);
		assert_cors(&res, "*");
		let text = response_body(res);
		assert_eq!(text, "she/her");
		client.terminate();
	}

	#[test]
	fn test_serves_webfinger() {
		let client = build_client();
		{
			let res = get(
				&client,
				"/.well-known/webfinger?resource=acct:average@average.name",
			);
			assert_status(&res, Status::Ok);
			assert_headers(&res);
			assert_cors(&res, "*");
			let content_type = res.content_type().expect("Content-Type should be sent");
			let mime = "application/jrd+json; charset=UTF-8"
				.parse::<MediaType>()
				.expect("MIME string should be valid");
			assert_eq!(content_type.0, mime);

			let body = res.into_string().expect("Result should be a string");
			let result: WebFinger =
				serde_json::from_str(&body).expect("Result should be a JSON string");
			assert_eq!(result.subject, "acct:avghelper@gts.average.name");
			assert_eq!(result.links.len(), 2);
		}
		client.terminate();
	}

	#[test]
	fn test_serves_webfinger_with_rel() {
		let client = build_client();
		{
			let res = get(
				&client,
				"/.well-known/webfinger?resource=acct:average@average.name&rel=self",
			);
			assert_status(&res, Status::Ok);
			assert_headers(&res);
			assert_cors(&res, "*");
			let content_type = res.content_type().expect("Content-Type should be sent");
			let mime = "application/jrd+json; charset=UTF-8"
				.parse::<MediaType>()
				.expect("MIME string should be valid");
			assert_eq!(content_type.0, mime);

			let body = res.into_string().expect("Result should be a string");
			let result: WebFinger =
				serde_json::from_str(&body).expect("Result should be a JSON string");
			assert_eq!(result.subject, "acct:avghelper@gts.average.name");
			assert_eq!(result.links.len(), 1);
		}
		client.terminate();
	}

	#[test]
	fn test_webfinger_fails_without_resource_param() {
		let client = build_client();
		{
			let res = get(&client, "/.well-known/webfinger");
			assert_status(&res, Status::BadRequest);
			assert_headers(&res);
		}
		client.terminate();
	}

	#[test]
	fn test_serves_nodeinfo() {
		let client = build_client();
		{
			let res = get(&client, "/.well-known/nodeinfo");
			assert_status(&res, Status::NotFound);
			assert_headers(&res);

			let res2 =
				get_with_user_agent(&client, "/.well-known/nodeinfo", "GitHub-NodeinfoQuery");
			assert_status(&res2, Status::Found);
			assert_headers(&res2);
		}
		client.terminate();
	}

	#[test]
	fn test_on_demand_tls_serves_400_without_domain() {
		let client = build_client();
		{
			let res = get(&client, "/.well-known/domains");
			assert_status(&res, Status::BadRequest);
			assert_headers(&res);
		}
		client.terminate();
	}

	#[test]
	fn test_on_demand_tls_serves_404_with_unknown_domain() {
		let client = build_client();
		{
			let res = get(&client, "/.well-known/domains?domain=example.com");
			assert_status(&res, Status::NotFound);
			assert_headers(&res);
		}
		client.terminate();
	}

	#[test]
	fn test_on_demand_tls_serves_204_with_known_domain() {
		let client = build_client();
		{
			let res = get(&client, "/.well-known/domains?domain=www.avg.name");
			assert_status(&res, Status::NoContent);
			assert_headers(&res);
		}
		client.terminate();
	}

	#[test]
	fn test_serves_static_files() {
		let file_paths = vec![
			("/robots.txt", ContentType::Plain, "https://average.name"),
			("/sitemap.html", ContentType::HTML, "https://average.name"),
			(
				"/sitemap-index.xml",
				ContentType::XML,
				"https://average.name",
			),
			("/sitemap-0.xml", ContentType::XML, "https://average.name"),
			("/.well-known/fursona.json", ContentType::JSON, "*"),
			("/index.html", ContentType::HTML, "https://average.name"),
			("/contact.html", ContentType::HTML, "https://average.name"),
		];

		let client = build_client();
		for (path, mime, origin) in file_paths {
			let res = get(&client, path);
			assert_status(&res, Status::Ok);
			assert_headers(&res);
			assert_cors(&res, origin);
			assert_content_type(&res, mime.clone());
			let response_contents = response_body(res);
			let expected_file_path = format!("./dist{path}");
			assert_file_contents_match(response_contents, &expected_file_path);
		}

		client.terminate();
	}

	#[test]
	fn test_omits_cors_header_for_unknown_origin() {
		let file_paths = vec![
			("/robots.txt", ContentType::Plain),
			("/sitemap.html", ContentType::HTML),
			("/sitemap-index.xml", ContentType::XML),
			("/sitemap-0.xml", ContentType::XML),
			("/index.html", ContentType::HTML),
			("/contact.html", ContentType::HTML),
		];

		let client = build_client();
		for (path, mime) in file_paths {
			let res = get_with_origin(&client, path, None);
			assert_status(&res, Status::Ok);
			assert_headers(&res);
			assert_content_type(&res, mime.clone());
			let has_allowed_origin = res
				.headers()
				.contains(header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str());
			assert_eq!(has_allowed_origin, false);
			let response_contents = response_body(res);
			let expected_file_path = format!("./dist{path}");
			assert_file_contents_match(response_contents, &expected_file_path);
		}
		client.terminate();
	}

	#[test]
	fn test_serves_static_files_without_extension() {
		let file_paths = vec![
			("/sitemap", "sitemap.html"),
			("/", "index.html"),
			("/contact", "contact.html"),
		];

		let client = build_client();
		for (path, file_name) in file_paths {
			let res = get(&client, path);
			assert_status(&res, Status::Ok);
			assert_headers(&res);
			assert_content_type(&res, ContentType::HTML);
			let response_contents = response_body(res);
			let expected_file_path = format!("./dist/{file_name}");
			assert_file_contents_match(response_contents, &expected_file_path);
		}

		client.terminate();
	}

	#[test]
	fn test_serves_404_for_unknown_page() {
		let file_paths = vec![
			"/foo_bar_nothing_to_see_here", //
			"/contact/no-thanks",
		];

		let client = build_client();
		for path in file_paths {
			let res = get(&client, path);
			assert_status(&res, Status::NotFound);
			assert_headers(&res);
			assert_content_type(&res, ContentType::HTML);
			let response_contents = response_body(res);
			assert_file_contents_match(response_contents, "./dist/404.html");
		}

		client.terminate();
	}

	fn assert_would_serve_file(path: &'static str, content_type: ContentType) {
		let client = build_client();
		let res = head(&client, path);
		assert_status(&res, Status::Ok);
		assert_headers(&res);
		assert_content_type(&res, content_type);
		// TODO: Should we send Content-Length here?

		// CORS should permit any origin
		assert_cors(&res, "*");

		let response_contents = response_bytes(res);
		assert_eq!(response_contents.len(), 0);
		client.terminate();
	}

	fn assert_serves_file(path: &'static str, content_type: ContentType) {
		let client = build_client();
		let res = get(&client, path);
		assert_status(&res, Status::Ok);
		assert_headers(&res);
		assert_content_type(&res, content_type);
		// TODO: Should we send Content-Length here?

		// CORS should permit any origin
		assert_cors(&res, "*");

		let response_contents = response_bytes(res);
		assert_file_bytes_match(response_contents, &format!("./dist{path}"));

		client.terminate();
	}

	#[test]
	fn test_serves_fursona_ref() {
		let path = "/images/refs/AverageHelper-avatar.png";
		assert_would_serve_file(path, ContentType::PNG);
		assert_serves_file("/images/refs/AverageHelper-avatar.png", ContentType::PNG);
	}

	#[test]
	fn test_serves_fursona_json() {
		assert_would_serve_file("/.well-known/fursona.json", ContentType::JSON);
		assert_serves_file("/.well-known/fursona.json", ContentType::JSON);
	}

	// TODO: Test that all internal links go where they're supposed to go

	#[test]
	fn test_redirects() {
		let redirects = vec![
			("/ip", "https://ip.average.name"),
			("/how", "/ways"),
			("/how.html", "/ways.html"),
			("/bookmarks", "/links"),
			("/bookmarks.html", "/links.html"),
			("/pronouns", "/.well-known/pronouns"),
			("/fursona.json", "/.well-known/fursona.json"),
			("/.well-known/fursona", "/.well-known/fursona.json"),
			("/@avg", "https://gts.average.name/@avghelper"),
			("/@avghelper", "https://gts.average.name/@avghelper"),
			("/@average", "https://gts.average.name/@avghelper"),
		];

		let client = build_client();
		for (from, to) in redirects {
			let res = get(&client, from);
			assert_status(&res, Status::Found);
			assert_headers(&res);
			assert_redir(&res, to);
		}

		client.terminate();
	}
}
