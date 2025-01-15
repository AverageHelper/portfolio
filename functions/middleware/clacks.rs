use actix_http::header::{HeaderName, HeaderValue};
use actix_web::{
	body::MessageBody,
	dev::{ServiceRequest, ServiceResponse},
	middleware::Next,
};
use rand::seq::SliceRandom;

pub const X_CLACKS_OVERHEAD: HeaderName = HeaderName::from_static("x-clacks-overhead");

const NAMES: &'static [&'static str] = &[
	"Terry Pratchett", // 28 April 1948 - 12 March 2015
	"Nex Benedict",    // 11 January 2008 - February 8, 2024
];

/// Sets `X-Clacks-Overhead` on HTTP responses.
pub async fn clacks(
	req: ServiceRequest,
	next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
	// Prepare the outgoing response; we set these headers after other handlers finish
	let mut res = next.call(req).await?;

	let name = *NAMES
		.choose(&mut rand::thread_rng())
		.expect("Names array should not be empty");

	res.headers_mut().insert(
		X_CLACKS_OVERHEAD,
		HeaderValue::from_str(&format!("GNU {name}"))?,
	);
	Ok(res)
}
