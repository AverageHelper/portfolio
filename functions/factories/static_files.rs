use actix_files::NamedFile;
use actix_http::{
	// header::{self, HeaderValue},
	uri::PathAndQuery,
	Uri,
};
use actix_service::{fn_service, Service, ServiceFactory};
use actix_web::{
	// body::MessageBody,
	dev::{self, HttpServiceFactory, ResourceDef, ServiceRequest, ServiceResponse},
	http::StatusCode,
};
use futures_core::future::LocalBoxFuture; // TODO: Can we do this without Futures Core?
use std::{path::PathBuf, str::FromStr};

// TODO: Handle HEAD
#[derive(Clone)]
pub struct ServeFiles {
	service: actix_files::Files,
	mount_path: String,
}

impl ServeFiles {
	/// Create new `ServeFiles` instance for a specified base directory, wrapping a
	/// [Files](actix_files::Files) instance and setting a fallback on `404.html`.
	///
	/// # Argument Order
	/// The first argument (`mount_path`) is the root URL at which the static files are served.
	/// For example, `/assets` will serve files at `example.com/assets/...`.
	///
	/// The second argument (`serve_from`) is the location on disk at which files are loaded.
	/// This can be a relative path. For example, `./` would serve files from the current
	/// working directory.
	///
	/// # Implementation Notes
	/// If the mount path is set as the root path `/`, services registered after this one will
	/// be inaccessible. Register more specific handlers and services first.
	///
	/// `Files` utilizes the existing Tokio thread-pool for blocking filesystem operations.
	/// The number of running threads is adjusted over time as needed, up to a maximum of 512 times
	/// the number of server [workers](actix_web::HttpServer::workers), by default.
	pub fn new<T: Into<PathBuf> + Clone>(mount_path: &str, serve_from: T) -> Self {
		let service = actix_files::Files::new(mount_path, serve_from)
			.default_handler(fn_service(not_found_fallback))
			.index_file("index.html")
			.prefer_utf8(true);
		ServeFiles {
			service,
			mount_path: mount_path.trim_end_matches('/').to_owned(),
		}
	}
}

impl HttpServiceFactory for ServeFiles {
	fn register(self, config: &mut dev::AppService) {
		let rdef = if config.is_root() {
			ResourceDef::root_prefix(&self.mount_path)
		} else {
			ResourceDef::prefix(&self.mount_path)
		};
		config.register_service(rdef, None, self, None)
	}
}

impl ServiceFactory<ServiceRequest> for ServeFiles {
	type Response = ServiceResponse;
	type Error = actix_web::Error;
	type Config = ();
	type Service = Self;
	type InitError = ();
	type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

	fn new_service(&self, _cfg: Self::Config) -> Self::Future {
		let val = self.clone();
		Box::pin(async move { Ok(val) })
	}
}

impl Service<ServiceRequest> for ServeFiles {
	type Response = ServiceResponse;
	type Error = actix_web::Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	dev::always_ready!();

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let service = self.service.clone();
		Box::pin(async move {
			let rewritten_req = rewrite_request_path(req);
			let res = service
				.new_service(())
				.await
				.unwrap() // TODO
				.call(rewritten_req)
				.await?;
			// let file_size = res.into_body().size();
			// let content_length = HeaderValue::from_str(&file_size.to_string())?;
			// res.headers_mut()
			// 	.insert(header::CONTENT_LENGTH, content_length);
			Ok(res)
		})
	}
}

/// Serves `dist/404.html`
async fn not_found_fallback(req: ServiceRequest) -> Result<ServiceResponse, actix_web::Error> {
	let not_found_fallback = NamedFile::open("./dist/404.html")
		.expect("404.html should exist")
		.prefer_utf8(true);

	let (req, _) = req.into_parts();
	let mut res = not_found_fallback.into_response(&req);
	*res.status_mut() = StatusCode::NOT_FOUND;
	Ok(ServiceResponse::new(req, res))
}

/// Returns a new request that appends a file extension to the path of the given request.
fn rewrite_request_path(req: ServiceRequest) -> ServiceRequest {
	// Actix by default expects the path to be either a directory or a file, but won't check for .html
	let new_path = {
		let orig_path = req.path();
		if orig_path == "/" || orig_path.contains(".") {
			// Root, or already has extension, or .well-known
			String::from(orig_path)
		} else {
			// Append .html
			format!("{orig_path}.html")
		}
	};
	let mut new_req = ServiceRequest::from(req);
	let mut parts = new_req.head_mut().uri.clone().into_parts();
	parts.path_and_query = Some(PathAndQuery::from_str(&new_path).unwrap()); // TODO
	let new_uri = Uri::from_parts(parts).unwrap(); // TODO
	new_req.match_info_mut().get_mut().update(&new_uri);
	new_req
}
