use crate::middleware::CorsAllowAll;
use core::{borrow::Borrow, convert::Infallible};
use rocket::http::{ContentType, Status};
use rocket::request::{self, FromRequest};
use rocket::response::{Redirect, Responder};
use rocket::{Request, Response, uri};
use serde::{Deserialize, Serialize};
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
			rel: "http://webfinger.net/rel/profile-page",
			r#type: Some("text/html"),
			href: Some(url.into()),
			template: None,
		}
	}

	fn with_self<U>(url: U) -> Self
	where
		U: Into<&'r str>,
	{
		AvailableLink {
			rel: "self",
			r#type: Some("application/activity+json"),
			href: Some(url.into()),
			template: None,
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

/// Answers Webfinger requests. See <https://www.rfc-editor.org/rfc/rfc7033.html>.
pub fn webfinger<'r>(
	resource: &'r str,
	rel: Option<Vec<&'r str>>,
) -> Result<WebFinger<'r>, Status> {
	// "If the "resource" parameter is absent or malformed, [...] indicate that the request is bad"
	let resource_uri = match Url::parse(resource) {
		Err(err) => {
			eprintln!("bad resource URI: {err}");
			return Err(Status::BadRequest);
		}
		Ok(value) => value,
	};

	// Fail if URI is only a protocol
	if resource_uri.as_str().ends_with(':') {
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
	let host = resource.split('@').next_back().unwrap_or("");
	if !host.ends_with("average.name") && host != "fosstodon.org" {
		return Err(Status::NotFound);
	}

	let rel_queries: Vec<&str> = rel.unwrap_or_default();

	let links: Vec<AvailableLink> = {
		let mut available_links = vec![
			AvailableLink::with_profile_page("https://gts.average.name/@avghelper"),
			AvailableLink::with_self("https://gts.average.name/users/avghelper"),
		];

		// "When the "rel" parameter is used and accepted, only the link relation types that match the link relation type provided via the "rel" parameter are included."
		if !rel_queries.is_empty() {
			available_links.retain(|link| rel_queries.contains(&link.rel));
		}

		available_links
	};

	Ok(WebFinger {
		subject: "acct:avghelper@gts.average.name",
		aliases: vec![
			"https://average.name/@average",
			"https://average.name/@avg",
			"https://average.name/@avghelper",
			"https://gts.average.name/@avghelper",
			"https://gts.average.name/users/avghelper",
		],
		links,
	})
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

/// Returns my Fedi's nodeinfo if the requester is GitHub's noneinfo query bot.
pub fn nodeinfo<'r, U: Borrow<UserAgent<'r>>>(user_agent: U) -> Result<Redirect, Status> {
	// Who's asking?
	let user_agent = user_agent.borrow();
	if user_agent.0.is_none()
		|| user_agent
			.0
			.is_some_and(|ua| !ua.starts_with("GitHub-NodeinfoQuery"))
	{
		// Non-GitHub User-Agent provided, hide:
		return Err(Status::NotFound);
	}

	// GitHub is asking. Point to Fedi:
	let fedi = uri!("https://gts.average.name/.well-known/nodeinfo");
	Ok(Redirect::found(fedi))
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn nodeinfo_returns_not_found_without_user_agent() {
		let no_user_agent = UserAgent(None);
		match nodeinfo(no_user_agent) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn nodeinfo_returns_not_found_with_regular_user_agent() {
		let user_agent = UserAgent(Some("foo"));
		match nodeinfo(user_agent) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn nodeinfo_returns_redirect_with_github_user_agent() {
		let user_agent = UserAgent(Some("GitHub-NodeinfoQuery"));
		nodeinfo(user_agent).expect("Expected 302");
	}

	#[test]
	fn webfinger_fails_if_resource_param_is_empty() {
		match webfinger("", None) {
			Ok(_) => panic!("Expected 400"),
			Err(status) => assert_eq!(status, Status::BadRequest),
		}
	}

	#[test]
	fn webfinger_fails_if_resource_param_is_not_a_url() {
		match webfinger("foo", None) {
			Ok(_) => panic!("Expected 400"),
			Err(status) => assert_eq!(status, Status::BadRequest),
		}
	}

	#[test]
	fn webfinger_fails_if_resource_param_is_only_protocol() {
		match webfinger("acct:", None) {
			Ok(_) => panic!("Expected 400"),
			Err(status) => assert_eq!(status, Status::BadRequest),
		}
	}

	#[test]
	fn webfinger_fails_if_resource_param_is_not_acct() {
		match webfinger("https:foo.bar", None) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn webfinger_fails_if_resource_url_param_host_is_not_known() {
		match webfinger("acct:foo.bar", None) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn webfinger_fails_if_resource_account_param_host_is_not_known() {
		match webfinger("acct:foo@foo.bar", None) {
			Ok(_) => panic!("Expected 404"),
			Err(status) => assert_eq!(status, Status::NotFound),
		}
	}

	#[test]
	fn webfinger_serializes_links_without_extra_keys() {
		let link = AvailableLink {
			rel: "foo",
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
		let Ok(data) = result else {
			panic!("Expected 200");
		};

		assert_eq!(data.subject, "acct:avghelper@gts.average.name");
		assert_eq!(
			data.aliases,
			vec![
				"https://average.name/@average",
				"https://average.name/@avg",
				"https://average.name/@avghelper",
				"https://gts.average.name/@avghelper",
				"https://gts.average.name/users/avghelper",
			]
		);

		let [ref link0, ref link1] = data.links[..] else {
			panic!("expected exactly 2 links");
		};

		assert_eq!(link0.rel, "http://webfinger.net/rel/profile-page");
		assert_eq!(link0.r#type.expect("kind should be present"), "text/html");
		assert_eq!(
			link0.href.expect("href should be present"),
			"https://gts.average.name/@avghelper"
		);

		assert_eq!(link1.rel, "self");
		assert_eq!(
			link1.r#type.expect("kind should be present"),
			"application/activity+json"
		);
		assert_eq!(
			link1.href.expect("href should be present"),
			"https://gts.average.name/users/avghelper"
		);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_average_name() {
		let res = webfinger("acct:average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_average_average_name() {
		let res = webfinger("acct:average@average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_gts_average_average_name() {
		let res = webfinger("acct:average@gts.average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_social_average_average_name() {
		let res = webfinger("acct:average@social.average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_any_subdomain_average_average_name() {
		let res = webfinger("acct:average@thissubdomaindoesnotexist.average.name", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_fosstodon_org() {
		let res = webfinger("acct:fosstodon.org", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_succeeds_with_resource_acct_avghelper_fosstodon_org() {
		let res = webfinger("acct:avghelper@fosstodon.org", None);
		assert_base_finger(res);
	}

	#[test]
	fn webfinger_responds_with_rel_self() {
		let Ok(data) = webfinger("acct:avghelper@fosstodon.org", Some(vec!["self"])) else {
			panic!("Expected 200");
		};
		assert_eq!(data.subject, "acct:avghelper@gts.average.name");
		assert!(!data.aliases.is_empty(), "Aliases should be nonempty");

		let [ref link] = data.links[..] else {
			panic!("expected exactly 1 link");
		};

		assert_eq!(link.rel, "self");
		assert_eq!(
			link.r#type.expect("kind should be present"),
			"application/activity+json"
		);
		assert_eq!(
			link.href.expect("href should be present"),
			"https://gts.average.name/users/avghelper"
		);
	}
}
