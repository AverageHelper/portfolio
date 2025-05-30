use rocket::http::Status;

const ALIAS_DOMAINS: &[&'static str] = &[
	// Subdomains that I want to give a *.avg.name alias:
	"blog.avg.name",
	"dotfiles.avg.name",
	"flashcards.avg.name",
	"git.avg.name",
	"ip.avg.name",
	"ipv4.avg.name",
	"jsonresume.avg.name",
	"redir.avg.name",
	"status.avg.name",
	"www.avg.name",
];

const AT_PROTO_DOMAINS: &[&'static str] = &[
	// Subdomains that I want to give an AT Protocol handle, i.e. @avgtest.average.name
	// "test.average.name", // DO NOT USE: This name is reserved internally in bsky
	"avgtest.average.name",
	"avg.average.name",
];

/// Handles Caddy's on-demand TLS requests.
// #[get("/.well-known/domains?<domain>")]
pub fn on_demand_tls(domain: &str) -> (Status, ()) {
	if domain == "avg.name"
		|| (ALIAS_DOMAINS).contains(&domain)
		|| (AT_PROTO_DOMAINS).contains(&domain)
	{
		return (Status::NoContent, ());
	}

	return (Status::NotFound, ());
}

// MARK: - Tests

#[cfg(test)]
mod tests {
	use super::*;
	use rocket::http::Status;

	fn assert_status(res: &(Status, ()), status_code: Status) {
		assert_eq!(res.0, status_code, "Status should be {status_code}")
	}

	#[test]
	fn test_answers_not_found_for_unknown_domain() {
		let domains = vec!["example.com", "foo.avg.name", "nobodyhere.average.name"];

		for domain in domains {
			let res = on_demand_tls(domain);
			assert_status(&res, Status::NotFound);
		}
	}

	#[test]
	fn test_answers_no_content_for_known_domain() {
		let domains = vec!["avg.name", "dotfiles.avg.name", "avg.average.name"];

		for domain in domains {
			let res = on_demand_tls(domain);
			assert_status(&res, Status::NoContent);
		}
	}
}
