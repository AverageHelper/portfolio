use http::header;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::uri::{Absolute, Host};
use rocket::shield::{
	Allow, Feature, Frame, Hsts, NoSniff, Permission, Prefetch, Referrer, Shield,
};
use rocket::time::Duration;
use rocket::uri;
use rocket::{Request, Response};

// Standard headers that aren't in the http crate yet:
const CROSS_ORIGIN_EMBEDDER_POLICY: &'static str = "Cross-Origin-Embedder-Policy";
const CROSS_ORIGIN_OPENER_POLICY: &'static str = "Cross-Origin-Opener-Policy";
const CROSS_ORIGIN_RESOURCE_POLICY: &'static str = "Cross-Origin-Resource-Policy";
const X_DOWNLOAD_OPTIONS: &'static str = "X-Download-Options";
const X_PERMITTED_CROSS_DOMAIN_POLICIES: &'static str = "X-Permitted-Cross-Domain-Policies";

/// A Rocket [Fairing](https://rocket.rs/guide/v0.5/fairings/#fairings) that sets
/// security headers on every response.
pub fn shield() -> Shield {
	Shield::new()
		.enable(NoSniff::Enable) // X-Content-Type-Options: nosniff
		.enable(Frame::Deny) // X-Frame-Options: DENY
		.enable(Hsts::Preload(Duration::seconds(31536000))) // Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
		.enable(Referrer::NoReferrer) // Referrer-Policy: no-referrer
		.enable(Prefetch::Off) // X-DNS-Prefetch-Control: off
		.enable(
			Permission::blocked(Feature::Accelerometer) // accelerometer=()
				// .block(Feature::AllScreensCapture) // TODO: all-screens-capture=()
				.block(Feature::AmbientLightSensor) // ambient-light-sensor=()
				// .block(Feature::AttributionReporting) // TODO: attribution-reporting=()
				.block(Feature::Autoplay) // autoplay=()
				.block(Feature::Battery) // battery=()
				// .block(Feature::Bluetooth) // TODO: bluetooth=()
				// .block(Feature::BrowsingTopics) // TODO: browsing-topics=()
				.block(Feature::Camera) // camera=()
				// .block(Feature::CapturedSurfaceControl) // TODO: captured-surface-control=()
				.block(Feature::ClipboardRead) // clipboard-read=()
				.block(Feature::ClipboardWrite) // clipboard-write=()
				.block(Feature::CrossOriginIsolated) // cross-origin-isolated=()
				// .block(Feature::DigitalCredentialsGet) // TODO: digital-credentials-get=()
				// .block(Feature::DirectSockets) // TODO: direct-sockets=()
				.block(Feature::Displaycapture) // display-capture=()
				.block(Feature::EncryptedMedia) // encrypted-media=()
				.block(Feature::ExecutionWhileNotRendered) // execution-while-not-rendered=()
				.block(Feature::ExecutionWhileOutOfviewport) // execution-while-out-of-viewport=()
				.block(Feature::FocusWithoutUserActivation) // focus-without-user-activation=()
				.allow(Feature::Fullscreen, [Allow::Any]) // fullscreen=*
				.block(Feature::Gamepad) // gemepad=()
				.block(Feature::Geolocation) // geolocation=()
				.block(Feature::Gyroscope) // gyroscope=()
				.block(Feature::Hid) // hid=()
				// .block(Feature::IdentityCredentialsGet) // TODO: identity-credentials-get=()
				.block(Feature::IdleDetection) // idle-detection=()
				.block(Feature::InterestCohort) // interest-cohort=()
				// .block(Feature::JoinAdInterestGroup) // TODO: join-ad-interest-group=()
				// .block(Feature::KeyboardMap) // TODO: keyboard-map=()
				// .block(Feature::LocalFonts) // TODO: local-fonts=()
				.block(Feature::Magnetometer) // magnetometer=()
				.block(Feature::Microphone) // microphone=()
				.block(Feature::Midi) // midi=()
				.block(Feature::NavigationOverride) // navigation-override=()
				.block(Feature::Payment) // payment=()
				.allow(Feature::PictureInPicture, [Allow::Any]) // picture-in-picture=*
				.block(Feature::PublickeyCredentialsGet) // publickey-credentials-get=()
				// .block(Feature::RunAdAuction) // TODO: run-ad-auction=()
				.block(Feature::ScreenWakeLock) // screen-wake-lock=()
				.block(Feature::Serial) // serial=()
				// .block(Feature::SharedAutofill) // TODO: shared-autofill=()
				// .block(Feature::SmartCard) // TODO: smart-card=()
				.block(Feature::SpeakerSelection) // speaker-selection=()
				// .block(Feature::StorageAccess) // TODO: storage-access=()
				.block(Feature::SyncScript) // sync-script=()
				.block(Feature::SyncXhr) // sync-xhr=()
				.block(Feature::TrustTokenRedemption) // trust-token-redemption=()
				// .block(Feature::Unload) // TODO: unload=()
				.block(Feature::Usb) // usb=()
				.allow(Feature::VerticalScroll, [Allow::This]) // vertical-scroll=(self)
				.allow(Feature::WebShare, [Allow::Any]) // web-share=*
				// .block(Feature::WindowManagement) // TODO: window-management=()
				.block(Feature::XrSpatialTracking), // xr-spatial-tracking=()
		)
}

/// A Rocket [Fairing](https://rocket.rs/guide/v0.5/fairings/#fairings) that sets
/// additional security headers on every response. Use in conjunction with [`shield`].
pub struct ExtraSecurityHeaders;

#[rocket::async_trait]
impl Fairing for ExtraSecurityHeaders {
	fn info(&self) -> Info {
		Info {
			name: "Security Headers",
			kind: Kind::Response,
		}
	}

	async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
		let user_provided_host = req.host();

		// We need to set script-src-elem dynamically, because
		// (1) XML styles somehow read as a script,
		// (2) we don't know which origin the request comes from until this spot, and
		// (3) I don't want to permit localhost resources in production.
		let rss_styles_src = format!("{}/rss/styles.xsl", resource_origin(user_provided_host));
		let sitemap_styles_src =
			format!("{}/sitemap/styles.xsl", resource_origin(user_provided_host));

		response.set_raw_header(header::CONTENT_SECURITY_POLICY.to_string(), format!("base-uri 'none'; default-src 'none'; form-action 'self'; frame-ancestors 'none'; img-src 'self' https://* data:; sandbox allow-same-origin allow-downloads allow-forms allow-scripts; style-src 'self' 'unsafe-inline'; media-src 'none'; script-src-elem {rss_styles_src} {sitemap_styles_src}; upgrade-insecure-requests"));
		response.set_raw_header(CROSS_ORIGIN_EMBEDDER_POLICY, "require-corp");
		response.set_raw_header(CROSS_ORIGIN_OPENER_POLICY, "same-origin");
		response.set_raw_header(CROSS_ORIGIN_RESOURCE_POLICY, "same-origin");
		response.set_raw_header(X_DOWNLOAD_OPTIONS, "noopen");
		response.set_raw_header(X_PERMITTED_CROSS_DOMAIN_POLICIES, "none");
	}
}

/// Returns the origin from the given user-provided host name. If the given host is
/// some flavor of `localhost`, then the host is returned as-is with an `http` scheme.
/// If the host is unknown, or no host is given, then our production domain name
/// is returned.
fn resource_origin<'a>(user_provided_host: Option<&'a Host<'_>>) -> Absolute<'static> {
	match user_provided_host {
		Some(untrustred) if untrustred.domain() == "localhost" => uri!("http://localhost"), // dev
		Some(untrustred) if untrustred.domain() == "127.0.0.1" => uri!("http://127.0.0.1"), // dev
		Some(untrustred) if untrustred.domain() == "[::1]" => uri!("http://[::1]"),         // dev
		Some(_) | None => return uri!("https://average.name"),                              // prod
	}
}
