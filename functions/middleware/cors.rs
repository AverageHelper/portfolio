use actix_cors::Cors;
// use actix_http::Uri;

/// Sets CORS headers on HTTP responses that permit only the default origin.
pub fn cors() -> Cors {
	Cors::default().allowed_origin("https://average.name")
}

/// Sets CORS headers on HTTP responses that permit only the given origin.
// pub fn cors_origin(allowed_origin: Uri) -> Cors {
// 	Cors::default().allowed_origin(&allowed_origin.to_string())
// }

/// Sets CORS headers on HTTP responses that permit any origin.
pub fn cors_any_origin() -> Cors {
	Cors::default().allow_any_origin()
}

// pub fn cors(origin_override: Option<&'static str>) -> CorsHandler {
// 	Cors::new()
// 		.allow_origin(origin_override.unwrap_or("https://average.name"))
// 		.into_handler()
// }
