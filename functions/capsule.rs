use crate::config::Config;
use fluffer::Status;
use rust_embed::Embed;
use std::path::PathBuf;
use url::Host;

/// Launches a Gemini capsule with the given configuration.
pub async fn gemini_service<'a>(config: &Config) -> Result<(), fluffer::AppErr> {
	let address = format!("[::]:{}", config.gemini_port);
	let Certs { key, cert } = Certs::from_config(config);

	println!("Gemini: Serving on port {}", config.gemini_port);

	fluffer::App::default()
		.state(config.clone())
		.address(address)
		.path_to_key(key)
		.path_to_cert(cert)
		.route("/", |client| route(client, |_| root()))
		.route("/robots.txt", static_txt) // See gemini://geminiprotocol.net/docs/companion/robots.gmi
		.route("/humans.txt", static_txt)
		.route("/contact", static_gmi)
		.route("/support", static_gmi)
		.route("/ways", |client| route(client, |_| ways()))
		.route("/ways/:slug", ways_content)
		.run()
		.await
}

// MARK: Static files

// Generated in build.rs:
include!(concat!(env!("OUT_DIR"), "/ways.rs"));

/// Serves the capsule index file.
async fn root() -> &'static str {
	include_str!("../src/content/gemtext/index.gmi")
}

/// Serves the Ways index file.
async fn ways() -> &'static str {
	include_str!(concat!(env!("OUT_DIR"), "/ways.gmi"))
}

/// Serves a static file from `$OUT_DIR/ways/{slug}.gmi` if the current path matches.
async fn ways_content(client: fluffer::Client<Config>) -> Result<Vec<u8>, RequestError> {
	route(client, async |client: fluffer::Client<Config>| {
		let slug = client.url.path();
		ways_from_slug(slug).ok_or(RequestError::NotFound)
	})
	.await
}

/// Serves a static file from `src/content/gemtext/{slug}.gmi` if the current path matches.
async fn static_gmi(client: fluffer::Client<Config>) -> Result<Vec<u8>, RequestError> {
	route(client, async |client: fluffer::Client<Config>| {
		let file_path = {
			let mut str = client.url.path().to_owned();
			str.push_str(".gmi");
			str
		};
		let asset = GemtextAsset::get(&file_path).ok_or(RequestError::NotFound)?;
		let data = match String::from_utf8(asset.data.to_vec()) {
			Err(_) => return Err(RequestError::TemporaryFailure),
			Ok(content) => content,
		};

		Ok(data) // text/gemini
	})
	.await
}

#[derive(Embed)]
#[folder = "src/content/gemtext/"]
#[prefix = "/"]
#[include = "*.gmi"]
struct GemtextAsset;

/// Serves a static file from `public/{slug}.txt` if the current path matches.
async fn static_txt(client: fluffer::Client<Config>) -> Result<Vec<u8>, RequestError> {
	route(client, async |client: fluffer::Client<Config>| {
		let file_path = client.url.path();
		let asset = PublicAsset::get(&file_path).ok_or(RequestError::NotFound)?;
		let data = match String::from_utf8(asset.data.to_vec()) {
			Err(_) => return Err(RequestError::TemporaryFailure),
			Ok(content) => content,
		};

		Ok((Status::Success, "text/plain", data))
	})
	.await
}

#[derive(Embed)]
#[folder = "public/"]
#[prefix = "/"]
#[include = "*.txt"]
struct PublicAsset;

// MARK: Certs

struct Certs {
	/// The path to key.pem
	key: PathBuf,

	/// The path to cert.pem
	cert: PathBuf,
}

impl Certs {
	fn from_config(config: &Config) -> Self {
		let certs_dir = config
			.gemini_certs_dir
			.as_ref()
			.expect("Missing Gemini certs directory config");
		if !certs_dir.is_dir() {
			panic!("Gemini certs directory not found: {}", certs_dir.display());
		}
		let key = certs_dir.join("key.pem");
		let cert = certs_dir.join("cert.pem");
		if !key.exists() || !cert.exists() {
			panic!("Gemini certs files not found in {}", certs_dir.display());
		}

		Self { key, cert }
	}
}

// MARK: Hostname

enum RequestError {
	/// The request was malformed.
	BadRequest,

	/// The requested resource was not found.
	NotFound,

	/// Something went wrong
	TemporaryFailure,

	/// The request was for a hostname we do not serve.
	WrongHost,
}

#[fluffer::async_trait]
impl fluffer::GemBytes for RequestError {
	async fn gem_bytes(self) -> Vec<u8> {
		match self {
			Self::BadRequest => (Status::BadRequest, "Bad request.").gem_bytes(),
			Self::NotFound => (Status::NotFound, "Page not found.").gem_bytes(),
			Self::TemporaryFailure => (Status::TemporaryFailure, "Temporary failure.").gem_bytes(),
			Self::WrongHost => (Status::ProxyRequestRefused, "Wrong host.").gem_bytes(),
		}
		.await
	}
}

async fn route(
	client: fluffer::Client<Config>,
	handler: impl fluffer::GemCall<Config> + 'static + Sync + Send,
) -> Result<Vec<u8>, RequestError> {
	let config = &client.state;
	let hostname = &config.gemini_hostname;

	// Make sure the URL is the correct size
	const URL_MAX_SIZE: usize = 1024;
	if client.url.as_str().len() > URL_MAX_SIZE {
		return Err(RequestError::BadRequest);
	}

	// Make sure the caller is using the correct scheme
	if client.url.scheme() != "gemini" {
		return Err(RequestError::WrongHost);
	}

	// Make sure the port number is correct, if given
	if let Some(port) = client.url.port() {
		if port != config.gemini_port {
			return Err(RequestError::WrongHost);
		}
	}

	// Make sure the caller found us through the correct domain!
	match client.url.host() {
		Some(url::Host::Domain(req_host)) if req_host == hostname => {
			// Known host
			Ok(handler.gem_call(client).await)
		}
		Some(host) if host.is_loopback() => {
			// Localhost
			Ok(handler.gem_call(client).await)
		}
		Some(url::Host::Domain(domain)) => {
			eprintln!("Caller requested an unknown domain {domain}");
			return Err(RequestError::WrongHost);
		}
		Some(url::Host::Ipv4(_)) | Some(url::Host::Ipv6(_)) | None => {
			eprintln!("Caller requested an unknown domain");
			return Err(RequestError::WrongHost);
		}
	}
}

trait LoopbackTest {
	fn is_loopback(&self) -> bool;
}

impl LoopbackTest for url::Host<&str> {
	fn is_loopback(&self) -> bool {
		match self {
			Host::Ipv4(ip) if ip.is_loopback() => true,
			Host::Ipv6(ip) if ip.is_loopback() => true,
			Host::Domain("localhost") => true,
			_ => false,
		}
	}
}
