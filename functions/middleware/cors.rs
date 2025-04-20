use http::header;
use rocket::{
	http::{uri::Absolute, Header},
	response::Responder,
	uri,
};

/// A response header that allows all origins.
pub struct CorsAllowAll;
impl Into<Header<'static>> for CorsAllowAll {
	fn into(self) -> Header<'static> {
		Header::new(header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), "*")
	}
}

/// A responder that wraps the given responder and applies CORS headers
/// that allow all request origins.
///
/// Effectively sets `access-control-allow-origin: *` on the response.
pub struct CorsAllowAllResponse<T: for<'r> Responder<'r, 'static>>(pub T);
impl<'r, T: for<'s> Responder<'s, 'static>> Responder<'r, 'static> for CorsAllowAllResponse<T> {
	fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		let mut res = self.0.respond_to(request)?;
		res.set_header(CorsAllowAll);
		Ok(res)
	}
}

const PROD_URI: Absolute<'static> = uri!("https://average.name");

/// A response header that allows only our deployment origin.
struct CorsOnlyProd;
impl Into<Header<'static>> for CorsOnlyProd {
	fn into(self) -> Header<'static> {
		Header::new(
			header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str(),
			PROD_URI.to_string(),
		)
	}
}

/// A responder that wraps the given responder and applies CORS headers
/// that allow only our deployment origin. If the request's `Origin` header
/// is not present or does not match our expected origin, then no new CORS
/// header is set.
///
/// Effectively sets `access-control-allow-origin: https://average.name` on applicable responses.
pub struct CorsOnlyProdResponse<T: for<'r> Responder<'r, 'static>>(pub T);
impl<'r, T: for<'s> Responder<'s, 'static>> Responder<'r, 'static> for CorsOnlyProdResponse<T> {
	fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
		let mut res = self.0.respond_to(request)?;

		if let Some(user_provided_origin) = request.headers().get_one(header::ORIGIN.as_str()) {
			if user_provided_origin == PROD_URI.to_string() {
				res.set_header(CorsOnlyProd);
			}
		}

		Ok(res)
	}
}
