use actix_http::header::HeaderName;
use actix_web::middleware;

pub const X_PRONOUNS_ACCEPTABLE: HeaderName = HeaderName::from_static("x-pronouns-acceptable");
pub const PRONOUNS_EN: &'static str = "she/her";

/// Sets [`X-Pronouns-Acceptable`](https://www.andrewyu.org/article/x-pronouns.html) on HTTP responses.
pub fn pronouns_acceptable() -> middleware::DefaultHeaders {
	middleware::DefaultHeaders::new().add((X_PRONOUNS_ACCEPTABLE, format!("en:{PRONOUNS_EN}")))
}
