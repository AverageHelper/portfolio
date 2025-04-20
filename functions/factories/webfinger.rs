use crate::middleware::CorsAllowAll;
use rocket::http::{ContentType, Status};
use rocket::request::{self, FromRequest};
use rocket::response::{Redirect, Responder};
use rocket::{uri, Request, Response};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::io::Cursor;
use url::Url;

#[derive(Serialize, Deserialize, Clone)]
pub struct AvailableLink<'r> {
	rel: &'r str,

	#[serde(skip_serializing_if = "Option::is_none")]
	r#type: Option<&'r str>,

	#[serde(skip_serializing_if = "Option::is_none")]
	href: Option<&'r str>,

	#[serde(skip_serializing_if = "Option::is_none")]
	template: Option<&'r str>,
}
impl<'r> AvailableLink<'r> {
	fn with_profile_page<U>(url: U) -> Self
	where
		U: Into<&'r str>,
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
		U: Into<&'r str>,
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
		T: Into<&'r str>,
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
pub struct WebFinger<'r> {
	pub subject: &'r str,
	pub aliases: Vec<&'r str>,
	pub links: Vec<AvailableLink<'r>>,
}
impl<'r> Responder<'r, 'static> for WebFinger<'r> {
	fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		let body = serde_json::to_vec(&self).map_err(|err| {
			eprintln!("[portfolio] Failed to construct WebFinger: {err}");
			Status::InternalServerError
		})?;
		let jrd_json =
			ContentType::new("application", "jrd+json").with_params(("charset", "UTF-8"));
		Response::build()
			.header(jrd_json)
			.header(CorsAllowAll) // Allow all origins. See https://www.rfc-editor.org/rfc/rfc7033#section-5.
			.sized_body(body.len(), Cursor::new(body))
			.ok()
	}
}

/// Answers Webfinger requests. See https://www.rfc-editor.org/rfc/rfc7033.html.
pub fn webfinger<'r>(
	resource: &'r str,
	rel: Option<Vec<&'r str>>,
) -> Result<WebFinger<'r>, Status> {
	// "If the "resource" parameter is absent or malformed, [...] indicate that the request is bad"
	let resource_uri = match Url::parse(resource) {
		Err(_) => {
			return Err(Status::BadRequest);
		}
		Ok(value) => value,
	};

	// Fail if URI is only a protocol
	if resource_uri.as_str().ends_with(":") {
		return Err(Status::BadRequest);
	}

	// We only know 'acct:' resources
	if resource_uri.scheme() != "acct" {
		return Err(Status::NotFound);
	}

	// Something like 'acct:average@average.name' or 'acct:average.name'
	let resource = resource_uri.path();
	if resource.is_empty() {
		return Err(Status::BadRequest);
	}

	// "If the "resource" parameter is a value for which the server has no information, the server MUST indicate [not found]"
	let host = resource.split("@").last().unwrap_or("");
	if host != "average.name" && host != "fosstodon.org" {
		return Err(Status::NotFound);
	}

	let rel_queries: Vec<&str> = match rel {
		None => vec![],
		Some(vec) => vec,
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

	return Ok(WebFinger {
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

/// A request guard that provides the request's User-Agent string, if any.
pub struct UserAgent<'r>(Option<&'r str>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
	type Error = Infallible;

	async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		let ua = req.headers().get_one("User-Agent");
		request::Outcome::Success(Self(ua))
	}
}

/// Returns Fosstodon's nodeinfo if the requester is GitHub's noneinfo query bot.
pub fn nodeinfo(user_agent: UserAgent<'_>) -> Result<Redirect, Status> {
	// Who's asking?
	if user_agent.0.is_none()
		|| user_agent
			.0
			.is_some_and(|ua| !ua.starts_with("GitHub-NodeinfoQuery"))
	{
		// Non-GitHub User-Agent provided, hide:
		return Err(Status::NotFound);
	}

	// GitHub is asking. Point to Mastodon:
	let mastodon = uri!("https://fosstodon.org/.well-known/nodeinfo");
	return Ok(Redirect::found(mastodon));
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_nodeinfo_returns_not_found_without_user_agent() {
		let no_user_agent = UserAgent(None);
		match nodeinfo(no_user_agent) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn test_nodeinfo_returns_not_found_with_regular_user_agent() {
		let user_agent = UserAgent(Some("foo"));
		match nodeinfo(user_agent) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn test_nodeinfo_returns_redirect_with_github_user_agent() {
		let user_agent = UserAgent(Some("GitHub-NodeinfoQuery"));
		match nodeinfo(user_agent) {
			Err(_) => panic!("Expected 302"),
			Ok(_) => {} // yay
		}
	}

	#[test]
	fn test_webfinger_fails_if_resource_param_is_empty() {
		match webfinger("", None) {
			Ok(_) => panic!("Expected 400"),
			Err(status) => assert_eq!(status, Status::BadRequest),
		}
	}

	#[test]
	fn test_webfinger_fails_if_resource_param_is_not_a_url() {
		match webfinger("foo", None) {
			Ok(_) => panic!("Expected 400"),
			Err(status) => assert_eq!(status, Status::BadRequest),
		}
	}

	#[test]
	fn test_webfinger_fails_if_resource_param_is_only_protocol() {
		match webfinger("acct:", None) {
			Ok(_) => panic!("Expected 400"),
			Err(status) => assert_eq!(status, Status::BadRequest),
		}
	}

	#[test]
	fn test_webfinger_fails_if_resource_param_is_not_acct() {
		match webfinger("https:foo.bar", None) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn test_webfinger_fails_if_resource_url_param_host_is_not_known() {
		match webfinger("acct:foo.bar", None) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn test_webfinger_fails_if_resource_account_param_host_is_not_known() {
		match webfinger("acct:foo@foo.bar", None) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn test_webfinger_serializes_links_without_extra_keys() {
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

	fn assert_base_finger(result: Result<WebFinger<'_>, Status>) {
		let data = match result {
			Ok(wf) => wf,
			Err(_) => panic!("Expected 200"),
		};

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

	#[test]
	fn test_webfinger_succeeds_with_resource_acct_average_name() {
		let res = webfinger("acct:average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn test_webfinger_succeeds_with_resource_acct_average_average_name() {
		let res = webfinger("acct:average@average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn test_webfinger_succeeds_with_resource_acct_fosstodon_org() {
		let res = webfinger("acct:fosstodon.org", None);
		assert_base_finger(res);
	}

	#[test]
	fn test_webfinger_succeeds_with_resource_acct_avghelper_fosstodon_org() {
		let res = webfinger("acct:avghelper@fosstodon.org", None);
		assert_base_finger(res);
	}

	#[test]
	fn test_webfinger_responds_with_rel_self() {
		let data = match webfinger("acct:avghelper@fosstodon.org", Some(vec!["self"])) {
			Ok(wf) => wf,
			Err(_) => panic!("Expected 200"),
		};
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
