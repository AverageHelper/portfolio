use actix_http::header;
use actix_web::{
	body::BoxBody, http::header::ContentType, web, Either, HttpRequest, HttpResponse,
	HttpResponseBuilder, Responder,
};
use mime_infer::mime;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
struct AvailableLink {
	rel: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	r#type: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	href: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	template: Option<String>,
}
impl AvailableLink {
	fn with_profile_page<U>(url: U) -> Self
	where
		U: Into<String>,
	{
		AvailableLink {
			rel: "http://webfinger.net/rel/profile-page".into(),
			r#type: Some("text/html".into()),
			href: Some(url.into()),
			template: None,
		}
	}

	fn with_self<U>(url: U) -> Self
	where
		U: Into<String>,
	{
		AvailableLink {
			rel: "self".into(),
			r#type: Some("application/activity+json".into()),
			href: Some(url.into()),
			template: None,
		}
	}

	fn with_subscribe_template<T>(template: T) -> Self
	where
		T: Into<String>,
	{
		AvailableLink {
			rel: "http://ostatus.org/schema/1.0/subscribe".into(), // Seems ostatus.org is no more, but Mastodon's docs still reference it
			r#type: None,
			href: None,
			template: Some(template.into()),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct WebFinger {
	subject: String,
	aliases: Vec<String>,
	links: Vec<AvailableLink>,
}
impl Responder for WebFinger {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let body: String = serde_json::to_string(&self).expect("WebFinger should form valid JSON");

		HttpResponse::Ok()
			.content_type(ContentType(
				"application/jrd+json; charset=UTF-8"
					.parse::<mime::Mime>()
					.expect("Static MIME type should parse"),
			))
			.body(body)
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrStringVec {
	String(String),
	Vec(Vec<String>),
}

#[derive(Deserialize)]
pub struct Info {
	resource: String,
	rel: Option<StringOrStringVec>,
}

/// Answers Webfinger requests. See https://www.rfc-editor.org/rfc/rfc7033.html.
pub async fn webfinger(info: web::Query<Info>) -> Either<WebFinger, HttpResponseBuilder> {
	// "If the "resource" parameter is absent or malformed, [...] indicate that the request is bad"
	let resource_query = &info.resource;
	let resource_uri = match Url::parse(resource_query) {
		Err(_) => {
			return Either::Right(HttpResponse::BadRequest());
		}
		Ok(value) => value,
	};

	// Fail if URI is only a protocol
	if resource_uri.as_str().ends_with(":") {
		return Either::Right(HttpResponse::BadRequest());
	}

	// We only know 'acct:' resources
	if resource_uri.scheme() != "acct" {
		return Either::Right(HttpResponse::NotFound());
	}

	// Something like 'acct:average@average.name' or 'acct:average.name'
	let resource = resource_uri.path();
	if resource.is_empty() {
		return Either::Right(HttpResponse::BadRequest());
	}

	// "If the "resource" parameter is a value for which the server has no information, the server MUST indicate [not found]"
	let host = resource.split("@").last().unwrap_or("");
	if host != "average.name" && host != "fosstodon.org" {
		return Either::Right(HttpResponse::NotFound());
	}

	let rel_queries: Vec<String> = match info.rel.clone() {
		None => vec![],
		Some(StringOrStringVec::String(value)) => vec![value],
		Some(StringOrStringVec::Vec(values)) => values,
	};

	let links: Vec<AvailableLink> = {
		let mut available_links = vec![
			AvailableLink::with_profile_page("https://fosstodon.org/@avghelper"),
			AvailableLink::with_self("https://fosstodon.org/users/avghelper"),
			AvailableLink::with_subscribe_template(
				"https://fosstodon.org/authorize_interaction?uri={uri}",
			),
		];

		// "When the "rel" parameter is used and accepted, only the link relation types that match the link relation type provided via the "rel" parameter are included."
		if !rel_queries.is_empty() {
			available_links.retain(|link| rel_queries.contains(&link.rel));
		}

		available_links
	};

	return Either::Left(WebFinger {
		// subject: "acct:average@average.name".into(),
		subject: "acct:avghelper@fosstodon.org".into(),
		aliases: vec![
			"https://average.name/@average".into(),
			"https://average.name/@avg".into(),
			"https://average.name/@avghelper".into(),
			"https://fosstodon.org/@avghelper".into(),
			"https://fosstodon.org/users/avghelper".into(),
		],
		links,
	});
}

/// Returns Fosstodon's nodeinfo if the requester is GitHub's noneinfo query bot.
pub async fn nodeinfo(req: HttpRequest) -> impl Responder {
	// Who's asking?
	let user_agent = req.headers().get(header::USER_AGENT);

	if user_agent.is_none()
		|| user_agent.is_some_and(|ua| {
			!ua.to_str()
				.is_ok_and(|ua| ua.starts_with("GitHub-NodeinfoQuery"))
		}) {
		// Non-GitHub User-Agent provided, hide:
		return HttpResponse::NotFound().finish();
	}

	// GitHub is asking. Point to Mastodon:
	return HttpResponse::Found()
		.insert_header((
			header::LOCATION,
			"https://fosstodon.org/.well-known/nodeinfo",
		))
		.finish();
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::*;
	use actix_http::StatusCode;
	use actix_service::ServiceFactory;
	use actix_web::{
		body::{self, MessageBody},
		dev::{ServiceRequest, ServiceResponse},
		test, web, App,
	};
	use mime_infer::mime;

	fn assert_status(res: &ServiceResponse<impl MessageBody>, status_code: StatusCode) {
		assert_eq!(res.status(), status_code, "Status should be {status_code}");
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
			.route("/.well-known/webfinger", web::get().to(webfinger))
			.route("/.well-known/nodeinfo", web::get().to(nodeinfo))
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

	#[actix_web::test]
	async fn test_nodeinfo_returns_not_found_without_user_agent() {
		let res = get("/.well-known/nodeinfo").await;
		assert_status(&res, StatusCode::NOT_FOUND);
	}

	#[actix_web::test]
	async fn test_nodeinfo_returns_not_found_with_regular_user_agent() {
		let res = get_with_user_agent("/.well-known/nodeinfo", "foo").await;
		assert_status(&res, StatusCode::NOT_FOUND);
	}

	#[actix_web::test]
	async fn test_nodeinfo_returns_redirect_with_github_user_agent() {
		let res = get_with_user_agent("/.well-known/nodeinfo", "GitHub-NodeinfoQuery").await;
		assert_status(&res, StatusCode::FOUND);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_without_resource_param() {
		let res = get("/.well-known/webfinger").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_if_resource_param_is_empty() {
		let res = get("/.well-known/webfinger?resource").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_if_resource_param_is_not_a_url() {
		let res = get("/.well-known/webfinger?resource=foo").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_if_resource_param_is_only_protocol() {
		let res = get("/.well-known/webfinger?resource=acct:").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_if_resource_param_is_not_acct() {
		let res = get("/.well-known/webfinger?resource=https:foo.bar").await;
		assert_status(&res, StatusCode::NOT_FOUND);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_if_resource_url_param_host_is_not_known() {
		let res = get("/.well-known/webfinger?resource=acct:foo.bar").await;
		assert_status(&res, StatusCode::NOT_FOUND);
	}

	#[actix_web::test]
	async fn test_webfinger_fails_if_resource_account_param_host_is_not_known() {
		let res = get("/.well-known/webfinger?resource=acct:foo@foo.bar").await;
		assert_status(&res, StatusCode::NOT_FOUND);
	}

	#[test]
	async fn test_webfinger_serializes_links_without_extra_keys() {
		let link = AvailableLink {
			rel: "foo".into(),
			r#type: None,
			href: None,
			template: None,
		};
		let serialized = serde_json::to_string(&link);
		assert_eq!(
			serialized.expect("Link should serialize"),
			"{\"rel\":\"foo\"}"
		);
	}

	async fn response_body(res: ServiceResponse<impl MessageBody>) -> String {
		let hundred_mb = 100_000_000;
		let body_bytes: web::Bytes = body::to_bytes_limited(res.into_body(), hundred_mb)
			.await
			.expect("Body should be under 100 MB")
			.ok()
			.expect("Body should download safely");
		String::from_utf8(body_bytes.to_vec()).expect("Body should be valid UTF-8")
	}

	async fn assert_base_finger(res: ServiceResponse<impl MessageBody>) {
		assert_status(&res, StatusCode::OK);

		let mime = "application/jrd+json; charset=UTF-8"
			.parse::<mime::Mime>()
			.expect("MIME string should be valid");
		assert_mime(&res, mime.clone());

		let text = response_body(res).await;
		let data: WebFinger = serde_json::from_str(&text).expect("Response should be valid JSON");
		assert_eq!(data.subject, "acct:avghelper@fosstodon.org");
		assert_eq!(
			data.aliases,
			vec![
				"https://average.name/@average",
				"https://average.name/@avg",
				"https://average.name/@avghelper",
				"https://fosstodon.org/@avghelper",
				"https://fosstodon.org/users/avghelper",
			]
		);

		let links = data.links;
		assert_eq!(links.len(), 3);

		let link0 = links[0].clone();
		assert_eq!(link0.rel, "http://webfinger.net/rel/profile-page");
		assert_eq!(link0.r#type.expect("kind should be present"), "text/html");
		assert_eq!(
			link0.href.expect("href should be present"),
			"https://fosstodon.org/@avghelper"
		);

		let link1 = links[1].clone();
		assert_eq!(link1.rel, "self");
		assert_eq!(
			link1.r#type.expect("kind should be present"),
			"application/activity+json"
		);
		assert_eq!(
			link1.href.expect("href should be present"),
			"https://fosstodon.org/users/avghelper"
		);

		let link2 = links[2].clone();
		assert_eq!(link2.rel, "http://ostatus.org/schema/1.0/subscribe");
		assert_eq!(
			link2.template.expect("template should be present"),
			"https://fosstodon.org/authorize_interaction?uri={uri}"
		);
	}

	#[actix_web::test]
	async fn test_webfinger_succeeds_with_resource_acct_average_name() {
		let res = get("/.well-known/webfinger?resource=acct:average.name").await;
		assert_base_finger(res).await;
	}

	#[actix_web::test]
	async fn test_webfinger_succeeds_with_resource_acct_average_average_name() {
		let res = get("/.well-known/webfinger?resource=acct:average@average.name").await;
		assert_base_finger(res).await;
	}

	#[actix_web::test]
	async fn test_webfinger_succeeds_with_resource_acct_fosstodon_org() {
		let res = get("/.well-known/webfinger?resource=acct:fosstodon.org").await;
		assert_base_finger(res).await;
	}

	#[actix_web::test]
	async fn test_webfinger_succeeds_with_resource_acct_avghelper_fosstodon_org() {
		let res = get("/.well-known/webfinger?resource=acct:avghelper@fosstodon.org").await;
		assert_base_finger(res).await;
	}

	#[actix_web::test]
	async fn test_webfinger_responds_with_rel_self() {
		let res =
			get("/.well-known/webfinger?resource=acct:avghelper@fosstodon.org&rel=self").await;
		assert_status(&res, StatusCode::OK);
		let text = response_body(res).await;
		let data: WebFinger = serde_json::from_str(&text).expect("Response should be valid JSON");
		assert_eq!(data.subject, "acct:avghelper@fosstodon.org");
		assert!(!data.aliases.is_empty(), "Aliases should be nonempty");

		let links = data.links;
		assert_eq!(links.len(), 1);

		let link0 = links[0].clone();
		assert_eq!(link0.rel, "self");
		assert_eq!(
			link0.r#type.expect("kind should be present"),
			"application/activity+json"
		);
		assert_eq!(
			link0.href.expect("href should be present"),
			"https://fosstodon.org/users/avghelper"
		);
	}
}
