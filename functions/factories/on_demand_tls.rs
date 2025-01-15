use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::LazyLock;

const ALIAS_DOMAINS: LazyLock<[String; 9]> = LazyLock::new(|| {
	[
		// Subdomains that I want to give a *.avg.name alias:
		"blog",
		"dotfiles",
		"flashcards",
		"git",
		"ip",
		"ipv4",
		"jsonresume",
		"status",
		"www",
	]
	.map(|s| format!("{s}.avg.name"))
});

const AT_PROTO_DOMAINS: LazyLock<[String; 2]> = LazyLock::new(|| {
	[
		// Subdomains that I want to give an AT Protocol handle, i.e. @avgtest.average.name
		// "test", // DO NOT USE: This name is reserved internally
		"avgtest", //
		"avg",
	]
	.map(|s| format!("{s}.average.name"))
});

#[derive(Deserialize)]
pub struct Info {
	domain: String,
}

/// Handles Caddy's on-demand TLS requests.
pub async fn on_demand_tls(info: web::Query<Info>) -> impl Responder {
	let domain = &info.domain;
	if domain == "avg.name"
		|| (&*ALIAS_DOMAINS).contains(domain)
		|| (&*AT_PROTO_DOMAINS).contains(domain)
	{
		return HttpResponse::NoContent();
	}

	return HttpResponse::NotFound();
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;
	use actix_http::StatusCode;
	use actix_web::{body::MessageBody, dev::ServiceResponse, test, App};

	fn assert_status(res: &ServiceResponse<impl MessageBody>, status_code: StatusCode) {
		assert_eq!(res.status(), status_code, "Status should be {status_code}")
	}

	async fn get(path: &str) -> ServiceResponse<impl MessageBody> {
		let app = test::init_service(
			App::new().route("/.well-known/domains", web::get().to(on_demand_tls)),
		)
		.await;
		let req = test::TestRequest::get().uri(path).to_request();
		test::call_service(&app, req).await
	}

	#[actix_web::test]
	async fn test_answers_bad_request_without_query() {
		let res = get("/.well-known/domains").await;
		assert_status(&res, StatusCode::BAD_REQUEST);
	}

	#[actix_web::test]
	async fn test_answers_not_found_for_unknown_domain() {
		let domains = vec!["example.com", "foo.avg.name", "nobodyhere.average.name"];

		for domain in domains {
			let res = get(&format!("/.well-known/domains?domain={domain}")).await;
			assert_status(&res, StatusCode::NOT_FOUND);
		}
	}

	#[actix_web::test]
	async fn test_answers_no_content_for_known_domain() {
		let domains = vec!["avg.name", "dotfiles.avg.name", "avg.average.name"];

		for domain in domains {
			let res = get(&format!("/.well-known/domains?domain={domain}")).await;
			assert_status(&res, StatusCode::NO_CONTENT);
		}
	}
}
