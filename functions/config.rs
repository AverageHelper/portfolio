use std::{
	net::{IpAddr, Ipv4Addr},
	path::PathBuf,
};

#[derive(Clone)]
pub struct Config {
	/// The port on which the Gemini webserver should listen.
	pub gemini_port: u16,

	/// The hostname at which the Gemini webserver should listen.
	/// Certificates are not generated automatically.
	pub gemini_hostname: String,

	/// The local directory in which to look for TLS certificates for the Gemini webserver.
	pub gemini_certs_dir: Option<PathBuf>,

	/// The port on which the HTTP webserver should listen.
	pub http_port: u16,

	/// The address at which both the HTTP webserver should listen.
	pub http_hostname: IpAddr,
}

impl Default for Config {
	fn default() -> Self {
		let gemini_hostname =
			std::env::var("GEMINI_HOSTNAME").unwrap_or_else(|_| "average.name".to_owned());
		let gemini_certs_dir = std::env::var("GEMINI_CERTS_DIR")
			.ok()
			.or_else(|| Some(".certs".to_owned()))
			.map(PathBuf::from);

		Self {
			gemini_port: 1965,
			gemini_hostname,
			gemini_certs_dir,
			http_port: 8787,
			http_hostname: IpAddr::V4(Ipv4Addr::UNSPECIFIED), // 0.0.0.0
		}
	}
}

impl Config {
	pub fn rocket_config(&self) -> rocket::Config {
		#[cfg(debug_assertions)]
		let defaults = rocket::Config::debug_default();
		#[cfg(not(debug_assertions))]
		let defaults = rocket::Config::release_default();

		rocket::Config {
			port: self.http_port,
			address: self.http_hostname,
			..defaults
		}
	}
}
