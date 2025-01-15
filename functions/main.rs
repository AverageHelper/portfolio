mod factories;
mod middleware;

use actix_files::NamedFile;
use actix_http::{
	header::{self, HeaderValue},
	Method,
};
use actix_service::ServiceFactory;
use actix_web::{
	body::{BoxBody, MessageBody},
	dev::{ServiceRequest, ServiceResponse},
	http::StatusCode,
	middleware::from_fn,
	route, web, App, HttpRequest, HttpResponse, HttpServer,
};
use factories::{nodeinfo, on_demand_tls, webfinger, ServeFiles};
use middleware::{
	clacks, cors, cors_any_origin, pronouns_acceptable, security_headers, PRONOUNS_EN,
};

fn redir(from: &'static str, to: &'static str) -> web::Redirect {
	web::redirect(from, to).using_status_code(StatusCode::FOUND)
}

#[route(
	"/images/refs/AverageHelper-avatar.png",
	method = "GET",
	method = "HEAD"
)]
async fn avatar(req: HttpRequest) -> Result<HttpResponse<BoxBody>, actix_web::Error> {
	let file = NamedFile::open("./dist/images/refs/AverageHelper-avatar.png")?.prefer_utf8(true);
	let file_size: u64 = file.metadata().len();
	let content_length = HeaderValue::from_str(&file_size.to_string())?;

	let mut res = file.into_response(&req);

	if let Method::HEAD = *req.method() {
		// Drop body when request is HEAD
		res = res.set_body(BoxBody::new(()));
	}

	// Add Content-Length, since NamedFile doesn't by default
	// FIXME: NamedFile should handle this
	res.headers_mut()
		.insert(header::CONTENT_LENGTH, content_length);
	Ok(res)
}

#[route("/.well-known/fursona.json", method = "GET", method = "HEAD")]
async fn fursona(req: HttpRequest) -> Result<HttpResponse<BoxBody>, actix_web::Error> {
	let file = NamedFile::open("./dist/.well-known/fursona.json")?.prefer_utf8(true);
	let file_size: u64 = file.metadata().len();
	let content_length = HeaderValue::from_str(&file_size.to_string())?;

	let mut res = file.into_response(&req);

	if let Method::HEAD = *req.method() {
		// Drop body when request is HEAD
		res = res.set_body(BoxBody::new(()));
	}

	// Add Content-Length, since NamedFile doesn't by default
	// FIXME: NamedFile should handle this
	res.headers_mut()
		.insert(header::CONTENT_LENGTH, content_length);
	Ok(res)
}

fn service() -> App<
	impl ServiceFactory<
		ServiceRequest,
		Response = ServiceResponse<impl MessageBody>,
		Config = (),
		InitError = (),
		Error = actix_web::Error,
	>,
> {
	App::new()
		.wrap(actix_web::middleware::Compress::default())
		.wrap(actix_web::middleware::NormalizePath::new(
			actix_web::middleware::TrailingSlash::Trim,
		))
		.wrap(from_fn(security_headers))
		// .wrap(cache_control)
		.wrap(from_fn(clacks))
		.wrap(pronouns_acceptable())
		//
		.route("/favicon.ico", web::get().to(|| HttpResponse::NotFound()))
		.service(redir("/ip", "https://ip.average.name"))
		//
		.service(redir("/how", "/ways"))
		.service(redir("/how.html", "/ways.html"))
		//
		.service(redir("/bookmarks", "/links"))
		.service(redir("/bookmarks.html", "/links.html"))
		//
		// ** Pronouns
		.service(redir("/pronouns", "/.well-known/pronouns"))
		.route(
			"/.well-known/pronouns",
			web::get()
				.wrap(cors_any_origin())
				.to(|| async { PRONOUNS_EN }),
		)
		//
		// ** Fursona
		.service(redir("/fursona.json", "/.well-known/fursona.json"))
		.service(redir("/.well-known/fursona", "/.well-known/fursona.json"))
		.service(avatar) // without CORS so external readers can access the file (#TODO: Use cors_any_origin())
		.service(fursona) // FIXME: actix Files won't serve .well-known without turning on hidden_files (See https://github.com/actix/actix-web/pull/3519)
		//
		// ** Fediverse aliases
		.service(redir("/@avg", "https://fosstodon.org/@avghelper"))
		.service(redir("/@avghelper", "https://fosstodon.org/@avghelper"))
		.service(redir("/@average", "https://fosstodon.org/@avghelper"))
		.route(
			"/.well-known/webfinger",
			web::get().wrap(cors()).to(webfinger),
		)
		.route(
			"/.well-known/nodeinfo",
			web::get().wrap(cors()).to(nodeinfo),
		)
		//
		// ** Caddy On-Demand TLS
		.route("/.well-known/domains", web::get().to(on_demand_tls))
		//
		// ** Serve the /dist dir
		.service(ServeFiles::new("/", "./dist"))
	// .service(ServeFiles)
	// 				.head(dir_head("./dist")), // FIXME: Need a way to turn off the body
}

static HOSTNAME: &'static str = "0.0.0.0";
static PORT: u16 = 8787;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	println!("Running server at http://{HOSTNAME}:{PORT}");

	HttpServer::new(service).bind((HOSTNAME, PORT))?.run().await
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;
	use actix_http::{
		header::{self, HeaderName},
		Method,
	};
	use actix_web::{
		body::{self, BodySize},
		test,
	};
	use middleware::{X_CLACKS_OVERHEAD, X_PRONOUNS_ACCEPTABLE};
	use mime_infer::mime;
	use std::str::FromStr;
	use web::Bytes;

	fn assert_header(res: &ServiceResponse<impl MessageBody>, key: HeaderName, value: &str) {
		let expectation = format!("{key} value should exist");
		assert_eq!(res.headers().get(key).expect(&expectation), value);
	}

	fn assert_headers(res: &ServiceResponse<impl MessageBody>) {
		assert!(res.headers().contains_key(header::CONTENT_SECURITY_POLICY));
		assert!(res
			.headers()
			.get(header::CONTENT_SECURITY_POLICY)
			.expect("Content-Security-Policy value should exist")
			.to_str()
			.expect("Header value should be a valid string")
			.contains("upgrade-insecure-requests"));
		assert_header(&res, header::REFERRER_POLICY, "no-referrer");
		assert!(res
			.headers()
			.contains_key(header::STRICT_TRANSPORT_SECURITY));
		assert!(res.headers().contains_key(header::X_CONTENT_TYPE_OPTIONS));
		assert!(res.headers().contains_key(header::X_FRAME_OPTIONS));
		assert!(res.headers().contains_key(header::X_XSS_PROTECTION));
		// assert!(res.headers().contains_key(header::VARY)); // TODO
		assert!(res.headers().contains_key(X_CLACKS_OVERHEAD));
		assert!(res.headers().contains_key(X_PRONOUNS_ACCEPTABLE));
	}

	fn assert_redir(res: &ServiceResponse<impl MessageBody>, location: &str) {
		assert_eq!(
			res.headers()
				.get(header::LOCATION)
				.expect("Location header should be present"),
			location
		);
	}

	fn assert_status(res: &ServiceResponse<impl MessageBody>, status_code: StatusCode) {
		assert_eq!(res.status(), status_code, "Status should be {status_code}")
	}

	fn assert_mime(res: &ServiceResponse<impl MessageBody>, mime: mime::Mime) {
		match res.headers().get(header::CONTENT_TYPE) {
			None => assert!(false, "Response should have a status code"),
			Some(given_mime) => {
				let mime_str = given_mime
					.to_str()
					.expect("Header value should be a string");
				assert_eq!(
					mime::Mime::from_str(mime_str)
						.expect("The header value should be a valid MIME type"),
					mime,
					"MIME type should be {mime}"
				)
			}
		}
	}

	fn assert_file_contents_match(response_contents: String, expected_file_path: &str) {
		let file_contents =
			std::fs::read_to_string(expected_file_path).expect("File should exist and be readable");
		assert_eq!(response_contents, file_contents);
	}

	fn assert_file_bytes_match(response_contents: Bytes, expected_file_path: &str) {
		let file_contents =
			std::fs::read(expected_file_path).expect("File should exist and be readable");
		assert_eq!(response_contents, file_contents);
	}

	async fn get(path: &str) -> ServiceResponse<impl MessageBody> {
		let app = test::init_service(service()).await;
		let req = test::TestRequest::get().uri(path).to_request();
		test::call_service(&app, req).await
	}
	async fn get_with_user_agent(
		path: &str,
		user_agent: &str,
	) -> ServiceResponse<impl MessageBody> {
		let app = test::init_service(service()).await;
		let req = test::TestRequest::get()
			.uri(path)
			.insert_header((header::USER_AGENT, user_agent))
			.to_request();
		test::call_service(&app, req).await
	}
	async fn head(path: &str) -> ServiceResponse<impl MessageBody> {
		let app = test::init_service(service()).await;
		let req = test::TestRequest::default()
			.method(Method::HEAD)
			.uri(path)
			.to_request();
		test::call_service(&app, req).await
	}

	async fn response_bytes(res_body: impl MessageBody) -> web::Bytes {
		let hundred_mb = 100_000_000;
		body::to_bytes_limited(res_body, hundred_mb)
			.await
			.expect("Body should be under 100 MB")
			.ok()
			.expect("Body should download safely")
	}

	async fn response_body(res: ServiceResponse<impl MessageBody>) -> String {
		let body_bytes = response_bytes(res.into_body()).await;
		String::from_utf8(body_bytes.to_vec()).expect("Body should be valid UTF-8")
	}

	#[actix_web::test]
	async fn test_answers_favicon() {
		let res = get("/favicon.ico").await;
		assert_status(&res, StatusCode::NOT_FOUND);
		assert_headers(&res);
		let body = response_body(res).await;
		assert!(body.is_empty());
	}

	#[actix_web::test]
	async fn test_answers_pronouns() {
		let res = get("/.well-known/pronouns").await;
		assert_status(&res, StatusCode::OK);
		assert_headers(&res);
		let text = response_body(res).await;
		assert_eq!(text, "she/her")
	}

	#[actix_web::test]
	async fn test_serves_webfinger() {
		let res = get("/.well-known/webfinger").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
		assert_headers(&res);
	}

	#[actix_web::test]
	async fn test_serves_nodeinfo() {
		let res = get("/.well-known/nodeinfo").await;
		assert_status(&res, StatusCode::NOT_FOUND);
		assert_headers(&res);

		let res2 = get_with_user_agent("/.well-known/nodeinfo", "GitHub-NodeinfoQuery").await;
		assert_status(&res2, StatusCode::FOUND);
		assert_headers(&res2);
	}

	#[actix_web::test]
	async fn test_serves_on_demand_tls() {
		let res = get("/.well-known/domains").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
		assert_headers(&res);
	}

	#[actix_web::test]
	async fn test_serves_static_files() {
		let file_paths = vec![
			("/robots.txt", mime::TEXT_PLAIN_UTF_8),
			("/sitemap.html", mime::TEXT_HTML_UTF_8),
			("/sitemap-index.xml", mime::TEXT_XML),
			("/sitemap-0.xml", mime::TEXT_XML),
			("/.well-known/fursona.json", mime::APPLICATION_JSON),
			("/index.html", mime::TEXT_HTML_UTF_8),
			("/contact.html", mime::TEXT_HTML_UTF_8),
		];

		for (path, mime) in file_paths {
			let res = get(path).await;
			assert_status(&res, StatusCode::OK);
			assert_headers(&res);
			assert_mime(&res, mime.clone());
			let response_contents = response_body(res).await;
			let expected_file_path = format!("./dist{path}");
			assert_file_contents_match(response_contents, &expected_file_path);

			// TODO: *Is* normal traffic supposed to have a Content-Length? Or are we streaming it down and so don't need the length? Must investigate...
			let res = head(path).await;
			assert_status(&res, StatusCode::OK);
			assert_headers(&res);
			assert_mime(&res, mime.clone());
			// // let response_contents = response_body(res).await;
			// let expected_file_path = format!("./dist{path}");
			// assert_file_contents_match(response_contents, &expected_file_path);
			let content_length = res
				.headers()
				.get(header::CONTENT_LENGTH)
				.expect("Content-Length should be provided")
				.to_str()
				.expect("Content-Length should be a string")
				.parse::<u64>()
				.expect("Content-Length should be an integer");
			assert!(content_length > 0); // at least 1 byte

			if let BodySize::Sized(size) = res.into_body().size() {
				assert_eq!(size, content_length)
			} else {
				assert!(false, "Body should be sized")
			}
		}
	}

	#[actix_web::test]
	async fn test_serves_static_files_without_extension() {
		let file_paths = vec![
			("/sitemap", "sitemap.html"),
			("/", "index.html"),
			("/contact", "contact.html"),
		];

		for (path, file_name) in file_paths {
			let res = get(path).await;
			assert_status(&res, StatusCode::OK);
			assert_headers(&res);
			assert_mime(&res, mime::TEXT_HTML_UTF_8);
			let response_contents = response_body(res).await;
			let expected_file_path = format!("./dist/{file_name}");
			assert_file_contents_match(response_contents, &expected_file_path);
		}
	}

	#[actix_web::test]
	async fn test_serves_404_for_unknown_page() {
		let file_paths = vec![
			"/foo_bar_nothing_to_see_here", //
			"/contact/no-thanks",
		];

		for path in file_paths {
			let res = get(path).await;
			assert_status(&res, StatusCode::NOT_FOUND);
			assert_headers(&res);
			assert_mime(&res, mime::TEXT_HTML_UTF_8);
			let response_contents = response_body(res).await;
			assert_file_contents_match(response_contents, "./dist/404.html");
		}
	}

	async fn assert_would_serve_file(path: &'static str, mime_type: mime::Mime) {
		let res = head(path).await;
		assert_status(&res, StatusCode::OK);
		assert_headers(&res);
		assert_header(&res, header::CONTENT_TYPE, mime_type.essence_str());
		let content_length = res
			.headers()
			.get(header::CONTENT_LENGTH)
			.expect("Content-Length should be provided")
			.to_str()
			.expect("Content-Length should be a string")
			.parse::<u64>()
			.expect("Content-Length should be an integer");
		assert!(content_length > 1_000); // at least 1kb

		if let BodySize::Sized(size) = res.into_body().size() {
			assert_eq!(size, 0)
		} else {
			assert!(false, "Body should be sized")
		}
	}

	async fn assert_serves_file(path: &'static str, mime_type: mime::Mime) {
		let res = get(path).await;
		assert_status(&res, StatusCode::OK);
		assert_headers(&res);
		assert_header(&res, header::CONTENT_TYPE, mime_type.essence_str());
		let content_length = res
			.headers()
			.get(header::CONTENT_LENGTH)
			.expect("Content-Length should be provided")
			.to_str()
			.expect("Content-Length should be a string")
			.parse::<u64>()
			.expect("Content-Length should be an integer");
		assert!(content_length > 1_000); // at least 1kb

		let body = res.into_body();
		if let BodySize::Sized(size) = body.size() {
			assert_eq!(size, content_length)
		} else {
			assert!(false, "Body should be sized")
		}

		let response_contents = response_bytes(body).await;
		assert_file_bytes_match(response_contents, &format!("./dist{path}"));
	}

	#[actix_web::test]
	async fn test_serves_fursona_ref() {
		let path = "/images/refs/AverageHelper-avatar.png";
		assert_would_serve_file(path, mime::IMAGE_PNG).await;
		assert_serves_file("/images/refs/AverageHelper-avatar.png", mime::IMAGE_PNG).await;
	}

	#[actix_web::test]
	async fn test_serves_fursona_json() {
		assert_would_serve_file("/.well-known/fursona.json", mime::APPLICATION_JSON).await;
		assert_serves_file("/.well-known/fursona.json", mime::APPLICATION_JSON).await;
	}

	// TODO: Test that security and CORS headers are installed on appropriate routes (CORS disabled on certain routes, etc.)
	// TODO: Test that all internal links go where they're supposed to go

	#[actix_web::test]
	async fn test_redirects() {
		let redirects = vec![
			("/ip", "https://ip.average.name"),
			("/how", "/ways"),
			("/how.html", "/ways.html"),
			("/bookmarks", "/links"),
			("/bookmarks.html", "/links.html"),
			("/pronouns", "/.well-known/pronouns"),
			("/fursona.json", "/.well-known/fursona.json"),
			("/.well-known/fursona", "/.well-known/fursona.json"),
			("/@avg", "https://fosstodon.org/@avghelper"),
			("/@avghelper", "https://fosstodon.org/@avghelper"),
			("/@average", "https://fosstodon.org/@avghelper"),
		];

		for (from, to) in redirects {
			let res = get(from).await;
			assert_status(&res, StatusCode::FOUND);
			assert_headers(&res);
			assert_redir(&res, to);
		}
	}
}
