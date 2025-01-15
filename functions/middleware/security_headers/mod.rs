// pub mod permissions_policy;

use actix_http::{
	header::{self, HeaderName, HeaderValue},
	Uri,
};
use actix_web::{
	body::MessageBody,
	dev::{ServiceRequest, ServiceResponse},
	middleware::Next,
};
use std::collections::HashSet;

// Standard headers that aren't in actix yet:
const ORIGIN_AGENT_CLUSTER: HeaderName = HeaderName::from_static("origin-agent-cluster");
const X_DOWNLOAD_OPTIONS: HeaderName = HeaderName::from_static("x-download-options");
const X_PERMITTED_CROSS_DOMAIN_POLICIES: HeaderName =
	HeaderName::from_static("x-permitted-cross-domain-policies");

/// Sets common security headers on HTTP responses.
pub async fn security_headers(
	req: ServiceRequest,
	next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
	let url = req.uri().clone();

	// Prepare the outgoing response; we set these headers after other handlers finish
	let mut res = next.call(req).await?;

	// We need to set script-src-elem dynamically, because
	// (1) XML styles somehow read as a script,
	// (2) we don't know which origin the request comes from until this spot, and
	// (3) I don't want to permit localhost resources in production.
	let rss_styles_src = format!("{}/rss/styles.xsl", resource_origin(&url));
	let sitemap_styles_src = format!("{}/sitemap/styles.xsl", resource_origin(&url));

	res
		.headers_mut().insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_str(&format!("base-uri 'none'; default-src 'none'; form-action 'self'; frame-ancestors 'none'; img-src 'self' https://* data:; sandbox allow-same-origin allow-downloads allow-forms allow-scripts; style-src 'self' 'unsafe-inline'; media-src 'none'; script-src-elem {rss_styles_src} {sitemap_styles_src}; upgrade-insecure-requests"))?);
	res.headers_mut().insert(
		header::CROSS_ORIGIN_EMBEDDER_POLICY,
		HeaderValue::from_static("require-corp"),
	);
	res.headers_mut().insert(
		header::CROSS_ORIGIN_OPENER_POLICY,
		HeaderValue::from_static("same-origin"),
	);
	res.headers_mut().insert(
		header::CROSS_ORIGIN_RESOURCE_POLICY,
		HeaderValue::from_static("same-origin"),
	);
	res.headers_mut()
		.insert(ORIGIN_AGENT_CLUSTER, HeaderValue::from_static("?1"));
	res.headers_mut().insert(header::PERMISSIONS_POLICY, HeaderValue::from_static("accelerometer=(), all-screens-capture=(), ambient-light-sensor=(), attribution-reporting=(), autoplay=(), battery=(), bluetooth=(), browsing-topics=(), camera=(), captured-surface-control=(), clipboard-read=(), clipboard-write=(), cross-origin-isolated=(), digital-credentials-get=(), direct-sockets=(), display-capture=(), encrypted-media=(), execution-while-not-rendered=(), execution-while-out-of-viewport=(), focus-without-user-activation=(), fullscreen=*, gemepad=(), geolocation=(), gyroscope=(), hid=(), identity-credentials-get=(), idle-detection=(), join-ad-interest-group=(), keyboard-map=(), local-fonts=(), magnetometer=(), microphone=(), midi=(), navigation-override=(), payment=(), picture-in-picture=*, publickey-credentials-get=(), run-ad-auction=(), screen-wake-lock=(), serial=(), shared-autofill=(), smart-card=(), speaker-selection=(), storage-access=(), sync-script=(), sync-xhr=(), trust-token-redemption=(), unload=(), usb=(), vertical-scroll=(self), web-share=*, window-management=(), xr-spatial-tracking=()"));
	res.headers_mut().insert(
		header::REFERRER_POLICY,
		HeaderValue::from_static("no-referrer"),
	);
	res.headers_mut().insert(
		header::STRICT_TRANSPORT_SECURITY,
		HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
	);
	res.headers_mut().insert(
		header::X_CONTENT_TYPE_OPTIONS,
		HeaderValue::from_static("nosniff"),
	);
	res.headers_mut().insert(
		header::X_DNS_PREFETCH_CONTROL,
		HeaderValue::from_static("off"),
	);
	res.headers_mut()
		.insert(X_DOWNLOAD_OPTIONS, HeaderValue::from_static("noopen"));
	res.headers_mut()
		.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
	res.headers_mut().insert(
		X_PERMITTED_CROSS_DOMAIN_POLICIES,
		HeaderValue::from_static("none"),
	);
	res.headers_mut().insert(
		header::X_XSS_PROTECTION,
		HeaderValue::from_static("1; mode=block"),
	);

	Ok(res)

	// set_secure_headers(
	// 	res,
	// 	SecureHeadersOptions {
	// 		// We follow https://github.com/w3c/webappsec-permissions-policy/blob/main/features.md
	// 		permissions_policy: Some(BTreeMap::from_iter(vec![
	// 			(Directive::Accelerometer, vec![]),
	// 			(Directive::AllScreensCapture, vec![]),
	// 			(Directive::AmbientLightSensor, vec![]),
	// 			(Directive::AttributionReporting, vec![]),
	// 			(Directive::Autoplay, vec![]),
	// 			(Directive::Battery, vec![]),
	// 			(Directive::Bluetooth, vec![]),
	// 			(Directive::BrowsingTopics, vec![]),
	// 			(Directive::Camera, vec![]),
	// 			(Directive::CapturedSurfaceControl, vec![]),
	// 			(Directive::ClipboardRead, vec![]),
	// 			(Directive::ClipboardWrite, vec![]),
	// 			(Directive::CrossOriginIsolated, vec![]),
	// 			(Directive::DigitalCredentialsGet, vec![]),
	// 			(Directive::DirectSockets, vec![]),
	// 			(Directive::DisplayCapture, vec![]),
	// 			// (Directive::DocumentDomain, vec![]), // TODO, Do we need this one?
	// 			(Directive::EncryptedMedia, vec![]),
	// 			(Directive::ExecutionWhileNotRendered, vec![]),
	// 			(Directive::ExecutionWhileOutOfViewport, vec![]),
	// 			(Directive::FocusWithoutUserActivation, vec![]),
	// 			(Directive::Fullscreen, vec![PermissionsPolicyValue::default()]),
	// 			(Directive::Gemepad, vec![]),
	// 			(Directive::Geolocation, vec![]),
	// 			(Directive::Gyroscope, vec![]),
	// 			(Directive::Hid, vec![]),
	// 			(Directive::IdentityCredentialsGet, vec![]),
	// 			(Directive::IdleDetection, vec![]),
	// 			// (Directive::InterestCohort, vec![]), // TODO, Does this one exist?
	// 			(Directive::JoinAdInterestGroup, vec![]),
	// 			(Directive::KeyboardMap, vec![]),
	// 			(Directive::LocalFonts, vec![]),
	// 			(Directive::Magnetometer, vec![]),
	// 			(Directive::Microphone, vec![]),
	// 			(Directive::Midi, vec![]),
	// 			(Directive::NavigationOverride, vec![]),
	// 			(Directive::Payment, vec![]),
	// 			(Directive::PictureInPicture, vec![PermissionsPolicyValue::default()]),
	// 			// (Directive::PublickeyCredentialsCreate, vec![]), // TODO, Does this one exist?
	// 			(Directive::PublickeyCredentialsGet, vec![]),
	// 			(Directive::RunAdAuction, vec![]),
	// 			(Directive::ScreenWakeLock, vec![]),
	// 			(Directive::Serial, vec![]),
	// 			(Directive::SharedAutofill, vec![]),
	// 			(Directive::SmartCard, vec![]),
	// 			(Directive::SpeakerSelection, vec![]),
	// 			(Directive::StorageAccess, vec![]),
	// 			(Directive::SyncScript, vec![]),
	// 			(Directive::SyncXhr, vec![]),
	// 			(Directive::TrustTokenRedemption, vec![]),
	// 			(Directive::Unload, vec![]),
	// 			(Directive::Usb, vec![]),
	// 			(Directive::VerticalScroll, vec![PermissionsPolicyValue::SelfValue]),
	// 			(Directive::WebShare, vec![PermissionsPolicyValue::default()]),
	// 			(Directive::WindowManagement, vec![]),
	// 			(Directive::XrSpatialTracking, vec![]),
	// 		].into_iter())),
	// 		content_security_policy: Some(ContentSecurityPolicyOptions {
	// 			base_uri: Some(vec![ContentSecurityPolicyOptionValue::None]),
	// 			// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/default-src
	// 			default_src: Some(vec![ContentSecurityPolicyOptionValue::None]),
	// 			form_action: Some(vec![ContentSecurityPolicyOptionValue::SelfValue]),
	// 			frame_ancestors: Some(vec![ContentSecurityPolicyOptionValue::None]),
	// 			img_src: Some(vec![ContentSecurityPolicyOptionValue::SelfValue, "https://*", "data:"]),
	// 			sandbox: Some(vec!["allow-same-origin", "allow-downloads", "allow-forms", "allow-scripts"]), // allow-scripts is only for rss/styles.xsl
	// 			// TODO: Ditch unsafe-inline. See https://astro-shield.kindspells.dev/guides/subresource-integrity/static-sites/
	// 			style_src: Some(vec![ContentSecurityPolicyOptionValue::SelfValue, ContentSecurityPolicyOptionValue::UnsafeInline]),
	// 			media_src: Some(vec![ContentSecurityPolicyOptionValue::None]),
	// 			// media_src: ["data:"], // Firefox wants this for some reason, but the error FF throws is benign, so leaving it for now.
	// 			script_src_elem: Some(vec![&rss_styles_src, &sitemap_styles_src]), // Specifically enable XML stylesheets
	// 			upgrade_insecure_requests: Some(vec![]),
	// 			..ContentSecurityPolicyOptions::default()
	// 		}),
	// 		// cross_origin_embedder_policy: Some("require-corp"),
	// 		cross_origin_resource_policy:
	// 			// Specifically allow other domains to access the fursona avatar
	// 			if url.path() == "/images/refs/AverageHelper-avatar.png" { Some("cross-origin") } else { Some("same-origin") },
	// 		// cross_origin_opener_policy: Some("same-origin"),
	// 		// origin_agent_cluster: Some("?1"),
	// 		// referrer_policy: Some("no-referrer"),
	// 		strict_transport_security: Some("max-age=31536000; includeSubDomains; preload"),
	// 		// x_content_type_options: Some("nosniff"),
	// 		// x_dns_prefetch_control: Some("off"),
	// 		// x_download_options: Some("noopen"),
	// 		x_frame_options: Some("DENY"),
	// 		// x_permitted_cross_domain_policies: Some("none"),
	// 		x_xss_protection: Some("1; mode=block"),
	// 		..SecureHeadersOptions::default()
	// 	},
	// );
}

/* Type-safe headers config inspired by Hono (https://github.com/honojs/hono/blob/v4.6.8/src/middleware/secure-headers/secure-headers.ts) */
/*
pub type ContentSecurityPolicyOptionValues = Vec<ContentSecurityPolicyOptionValue>;

pub enum ContentSecurityPolicyOptionValue {
	None,
	SelfValue,
	StrictDynamic,
	ReportSample,
	InlineSpeculationRules,
	UnsafeInline,
	UnsafeEval,
	UnsafeHashes,
	WasmUnsafeEval,
}
impl ToString for ContentSecurityPolicyOptionValue {
	fn to_string(&self) -> String {
		match self {
			Self::None => "'none'",
			Self::SelfValue => "'self'",
			Self::StrictDynamic => "'strict-dynamic'",
			Self::ReportSample => "'report-sample'",
			Self::InlineSpeculationRules => "'inline-speculation-rules'",
			Self::UnsafeInline => "'unsafe-inline'",
			Self::UnsafeEval => "'unsafe-eval'",
			Self::UnsafeHashes => "'unsafe-hashes'",
			Self::WasmUnsafeEval => "'wasm-unsafe-eval'",
		}
		.into()
	}
}

pub struct ContentSecurityPolicyOptions {
	default_src: Option<ContentSecurityPolicyOptionValues>,
	base_uri: Option<ContentSecurityPolicyOptionValues>,
	child_src: Option<ContentSecurityPolicyOptionValues>,
	connect_src: Option<ContentSecurityPolicyOptionValues>,
	font_src: Option<ContentSecurityPolicyOptionValues>,
	form_action: Option<ContentSecurityPolicyOptionValues>,
	frame_ancestors: Option<ContentSecurityPolicyOptionValues>,
	frame_src: Option<ContentSecurityPolicyOptionValues>,
	img_src: Option<ContentSecurityPolicyOptionValues>,
	manifest_src: Option<ContentSecurityPolicyOptionValues>,
	media_src: Option<ContentSecurityPolicyOptionValues>,
	object_src: Option<ContentSecurityPolicyOptionValues>,
	report_to: Option<&'static str>,
	sandbox: Option<ContentSecurityPolicyOptionValues>,
	script_src: Option<ContentSecurityPolicyOptionValues>,
	script_src_attr: Option<ContentSecurityPolicyOptionValues>,
	script_src_elem: Option<ContentSecurityPolicyOptionValues>,
	style_src: Option<ContentSecurityPolicyOptionValues>,
	style_src_attr: Option<ContentSecurityPolicyOptionValues>,
	style_src_elem: Option<ContentSecurityPolicyOptionValues>,
	upgrade_insecure_requests: Option<ContentSecurityPolicyOptionValues>,
	worker_src: Option<ContentSecurityPolicyOptionValues>,
}

impl Default for ContentSecurityPolicyOptions {
	fn default() -> Self {
		Self {
			default_src: None,
			base_uri: None,
			child_src: None,
			connect_src: None,
			font_src: None,
			form_action: None,
			frame_ancestors: None,
			frame_src: None,
			img_src: None,
			manifest_src: None,
			media_src: None,
			object_src: None,
			report_to: None,
			sandbox: None,
			script_src: None,
			script_src_attr: None,
			script_src_elem: None,
			style_src: None,
			style_src_attr: None,
			style_src_elem: None,
			upgrade_insecure_requests: None,
			worker_src: None,
		}
	}
}

fn to_directive(name: &str, value: Vec<PermissionsPolicyValue>) -> String {
	format!("{name} {}", value.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(" "))
}

impl ToString for ContentSecurityPolicyOptions {
	fn to_string(&self) -> String {
		let mut result = String::from("");
		if let Some(value) = self.base_uri {
			result.push_str(&to_directive("base-uri", value));
		}
		if let Some(value) = self.child_src {
			result.push_str(&to_directive("child-src", value));
		}
		if let Some(value) = self.connect_src {
			result.push_str(&to_directive("connect-src", value));
		}
		if let Some(value) = self.default_src {
			result.push_str(&to_directive("default-src", value));
		}
		if let Some(value) = self.font_src {
			result.push_str(&to_directive("font-src", value));
		}
		if let Some(value) = self.form_action {
			result.push_str(&to_directive("form-action", value));
		}
		if let Some(value) = self.frame_ancestors {
			result.push_str(&to_directive("frame-ancestors", value));
		}
		if let Some(value) = self.frame_src {
			result.push_str(&to_directive("frame-src", value));
		}
		if let Some(value) = self.img_src {
			result.push_str(&to_directive("img-src", value));
		}
		if let Some(value) = self.manifest_src {
			result.push_str(&to_directive("manifest-src", value));
		}
		if let Some(value) = self.media_src {
			result.push_str(&to_directive("media-src", value));
		}
		if let Some(value) = self.object_src {
			result.push_str(&to_directive("object-src", value));
		}
		if let Some(value) = self.report_to {
			result.push_str(value);
		}
		sandbox: Option<ContentSecurityPolicyOptionValues>;
		script_src: Option<ContentSecurityPolicyOptionValues>;
		script_src_attr: Option<ContentSecurityPolicyOptionValues>;
		script_src_elem: Option<ContentSecurityPolicyOptionValues>;
		style_src: Option<ContentSecurityPolicyOptionValues>;
		style_src_attr: Option<ContentSecurityPolicyOptionValues>;
		style_src_elem: Option<ContentSecurityPolicyOptionValues>;
		upgrade_insecure_requests: Option<ContentSecurityPolicyOptionValues>;
		worker_src: Option<ContentSecurityPolicyOptionValues>;
		return result;
	}
}

pub struct ReportingEndpointOptions {
	name: String,
	url: String,
}

pub struct ReportToEndpoint {
	url: String,
}

pub struct ReportToOptions {
	group: String,
	max_age: u64, // TODO: ??
	endpoints: Vec<ReportToEndpoint>,
}

pub enum PermissionsPolicyValue {
	Star,
	SelfValue,
	Src,
	Other(String),
}
impl Default for PermissionsPolicyValue {
	fn default() -> Self {
		Self::Star
	}
}
impl ToString for PermissionsPolicyValue {
	fn to_string(&self) -> String {
		match self {
			Self::Star => "*",
			Self::SelfValue => "self",
			Self::Src => "src",
			Self::Other(val) => val,
		}
		.into()
	}
}

pub type PermissionsPolicyAllowList = Vec<PermissionsPolicyValue>;
// pub enum PermissionsPolicyAllowList {
// 	Values(Vec<PermissionsPolicyValue>),
// 	Include(bool), true == Star, false == "None"
// }

pub type PermissionsPolicyOptions = BTreeMap<Directive, PermissionsPolicyAllowList>;

pub struct SecureHeadersOptions<'a> {
	content_security_policy: Option<ContentSecurityPolicyOptions>,
	content_security_policy_report_only: Option<ContentSecurityPolicyOptions>,
	cross_origin_embedder_policy: Option<&'a str>,
	cross_origin_resource_policy: Option<&'a str>,
	cross_origin_opener_policy: Option<&'a str>,
	origin_agent_cluster: Option<&'a str>,
	referrer_policy: Option<&'a str>,
	reporting_endpoints: Option<Vec<ReportingEndpointOptions>>,
	report_to: Option<Vec<ReportToOptions>>,
	strict_transport_security: Option<&'a str>,
	x_content_type_options: Option<&'a str>,
	x_dns_prefetch_control: Option<&'a str>,
	x_download_options: Option<&'a str>,
	x_frame_options: Option<&'a str>,
	x_permitted_cross_domain_policies: Option<&'a str>,
	x_xss_protection: Option<&'a str>,
	permissions_policy: Option<PermissionsPolicyOptions>,
	remove_powered_by: bool,
}

impl Default for SecureHeadersOptions<'_> {
	fn default() -> Self {
		Self {
			content_security_policy: None, // Matches Header::ContentSecurityPolicy.default_value(),
			content_security_policy_report_only: None, // Matches Header::ContentSecurityPolicyReportOnly.default_value(),
			cross_origin_embedder_policy: Header::CrossOriginEmbedderPolicy.default_value(),
			cross_origin_resource_policy: Header::CrossOriginResourcePolicy.default_value(),
			cross_origin_opener_policy: Header::CrossOriginOpenerPolicy.default_value(),
			origin_agent_cluster: Header::OriginAgentCluster.default_value(),
			referrer_policy: Header::ReferrerPolicy.default_value(),
			reporting_endpoints: None, // Matches Header::ReportingEndpoints.default_value(),
			report_to: None,           // Matches Header::ReportTo.default_value(),
			strict_transport_security: Header::StrictTransportSecurity.default_value(),
			x_content_type_options: Header::XContentTypeOptions.default_value(),
			x_dns_prefetch_control: Header::XDNSPrefetchControl.default_value(),
			x_download_options: Header::XDownloadOptions.default_value(),
			x_frame_options: Header::XFrameOptions.default_value(),
			x_permitted_cross_domain_policies: Header::XPermittedCrossDomainPolicies
				.default_value(),
			x_xss_protection: Header::XXSSProtection.default_value(),
			permissions_policy: None, // Matches Header::PermissionsPolicy.default_value(),
			remove_powered_by: true,
		}
	}
}

/// Security headers.
enum Header {
	ContentSecurityPolicy,
	ContentSecurityPolicyReportOnly,
	CrossOriginEmbedderPolicy,
	CrossOriginResourcePolicy,
	CrossOriginOpenerPolicy,
	OriginAgentCluster,
	ReferrerPolicy,
	ReportingEndpoints,
	ReportTo,
	StrictTransportSecurity,
	XContentTypeOptions,
	XDNSPrefetchControl,
	XDownloadOptions,
	XFrameOptions,
	XPermittedCrossDomainPolicies,
	XXSSProtection,
	PermissionsPolicy,
}

impl Header {
	/// Returns the name of the header.
	const fn to_str(&self) -> &'static str {
		match self {
			Self::ContentSecurityPolicy => "Content-Security-Policy",
			Self::ContentSecurityPolicyReportOnly => "Content-Security-Policy-Report-Only",
			Self::CrossOriginEmbedderPolicy => "Cross-Origin-Embedder-Policy",
			Self::CrossOriginResourcePolicy => "Cross-Origin-Resource-Policy",
			Self::CrossOriginOpenerPolicy => "Cross-Origin-Opener-Policy",
			Self::OriginAgentCluster => "Origin-Agent-Cluster",
			Self::ReferrerPolicy => "Referrer-Policy",
			Self::ReportingEndpoints => "Reporting-Endpoints",
			Self::ReportTo => "Report-To",
			Self::StrictTransportSecurity => "Strict-Transport-Security",
			Self::XContentTypeOptions => "X-Content-Type-Options",
			Self::XDNSPrefetchControl => "X-DNS-Prefetch-Control",
			Self::XDownloadOptions => "X-Download-Options",
			Self::XFrameOptions => "X-Frame-Options",
			Self::XPermittedCrossDomainPolicies => "X-Permitted-Cross-Domain-Policies",
			Self::XXSSProtection => "X-XSS-Protection",
			Self::PermissionsPolicy => "Permissions-Policy",
		}
	}

	/// Returns the default value for the header, if any.
	const fn default_value(&self) -> Option<&'static str> {
		match self {
			Self::ContentSecurityPolicy => None,
			Self::ContentSecurityPolicyReportOnly => None,
			Self::CrossOriginEmbedderPolicy => Some("require-corp"),
			Self::CrossOriginResourcePolicy => Some("same-origin"),
			Self::CrossOriginOpenerPolicy => Some("same-origin"),
			Self::OriginAgentCluster => Some("?1"),
			Self::ReferrerPolicy => Some("no-referrer"),
			Self::ReportingEndpoints => None,
			Self::ReportTo => None,
			Self::StrictTransportSecurity => Some("max-age=15552000; includeSubDomains"),
			Self::XContentTypeOptions => Some("nosniff"),
			Self::XDNSPrefetchControl => Some("off"),
			Self::XDownloadOptions => Some("noopen"),
			Self::XFrameOptions => Some("SAMEORIGIN"),
			Self::XPermittedCrossDomainPolicies => Some("none"),
			Self::XXSSProtection => Some("0"),
			Self::PermissionsPolicy => None,
		}
	}
}
*/
/* TODO: To make invalid states unrepresentable (as opposed to the current stringly-typed layout), try something like this instead for each security header: */
/*
/// Represents the [Cross-Origin-Opener-Policy](https://developer.mozilla.org/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy) header.
pub struct CrossOriginOpenerPolicy {
	directive: CrossOriginOpenerPolicyDirective,
}
impl CrossOriginOpenerPolicy {
	pub const fn new(directive: CrossOriginOpenerPolicyDirective) -> Self {
		Self { directive }
	}
	pub const fn header_name() -> &'static str {
		return "Cross-Origin-Opener-Policy";
	}
	pub const fn header_value(&self) -> &'static str {
		match self.directive {
			CrossOriginOpenerPolicyDirective::UnsafeNone => "unsafe-none",
			CrossOriginOpenerPolicyDirective::SameOriginAllowPopups => "same-origin-allow-popups",
			CrossOriginOpenerPolicyDirective::SameOrigin => "same-origin",
		}
	}
}
/// Directive values for the [Cross-Origin-Opener-Policy](https://developer.mozilla.org/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy) header.
///
/// We avoid implementing [`Default`] for this type to encourage implementers to decide which
/// directive best fits their project.
pub enum CrossOriginOpenerPolicyDirective {
	/// From [MDN](https://developer.mozilla.org/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy#unsafe-none):
	/// This is the default value. Allows the document to be added to its opener's browsing
	/// context group unless the opener itself has a COOP of `same-origin` or
	/// `same-origin-allow-popups`.
	UnsafeNone,

	/// From [MDN](https://developer.mozilla.org/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy#same-origin-allow-popups):
	/// Retains references to newly opened windows or tabs that either don't set COOP or
	/// that opt out of isolation by setting a COOP of `unsafe-none`.
	SameOriginAllowPopups,

	/// From [MDN](https://developer.mozilla.org/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy#same-origin):
	/// Isolates the browsing context exclusively to same-origin documents. Cross-origin
	/// documents are not loaded in the same browsing context.
	SameOrigin,
}
*/
/*
fn set_header<V: TryInto<HeaderValue>>(res: &mut Response, header: Header, value: V) {
	res.add_header(header.to_str(), value, true);
}

/// Sets security headers on the given response, based on the given options.
///
/// To use the default optimal settings:
/// ```
/// set_secure_headers(res, SecureHeadersOptions::default());
/// ```
///
/// To omit unnecessary headers:
/// ```
/// set_secure_headers(res, SecureHeadersOptions {
/// 	x_frame_options: Some(SecureHeaderValue::Exclude),
/// 	x_xss_protection: Some(SecureHeaderValue::Exclude),
/// 	..SecureHeadersOptions::default()
/// });
/// ```
///
/// To override default header values:
/// ```
/// set_secure_headers(res, SecureHeadersOptions {
/// 	strict_transport_security: Some(SecureheaderValue::Override(
/// 		"max-age=63072000; includeSubDomains; preload".into()
/// 	)),
/// 	x_frame_options: Some(SecureheaderValue::Override("DENY".into())),
/// 	x_xss_protection: Some(SecureheaderValue::Override("1".into())),
/// 	..SecureHeadersOptions::default()
/// });
/// ```
fn set_secure_headers(res: &mut Response, options: SecureHeadersOptions) {
	if let Some(value) = options.content_security_policy {
		set_header(res, Header::ContentSecurityPolicy, value.to_string());
	}
	if let Some(value) = options.content_security_policy_report_only {
		set_header(res, Header::ContentSecurityPolicyReportOnly, value.to_string());
	}
	if let Some(value) = options.cross_origin_embedder_policy {
		set_header(res, Header::CrossOriginEmbedderPolicy, value);
	}
	if let Some(value) = options.cross_origin_resource_policy {
		set_header(res, Header::CrossOriginResourcePolicy, value);
	}
	if let Some(value) = options.cross_origin_opener_policy {
		set_header(res, Header::CrossOriginOpenerPolicy, value);
	}
	if let Some(value) = options.origin_agent_cluster {
		set_header(res, Header::OriginAgentCluster, value);
	}
	if let Some(value) = options.referrer_policy {
		set_header(res, Header::ReferrerPolicy, value);
	}
	if let Some(value) = options.reporting_endpoints {
		set_header(res, Header::ReportingEndpoints, value);
	}
	if let Some(value) = options.report_to {
		set_header(res, Header::ReportTo, value);
	}
	if let Some(value) = options.strict_transport_security {
		set_header(res, Header::StrictTransportSecurity, value);
	}
	if let Some(value) = options.x_content_type_options {
		set_header(res, Header::XContentTypeOptions, value);
	}
	if let Some(value) = options.x_dns_prefetch_control {
		set_header(res, Header::XDNSPrefetchControl, value);
	}
	if let Some(value) = options.x_download_options {
		set_header(res, Header::XDownloadOptions, value);
	}
	if let Some(value) = options.x_frame_options {
		set_header(res, Header::XFrameOptions, value);
	}
	if let Some(value) = options.x_permitted_cross_domain_policies {
		set_header(res, Header::XPermittedCrossDomainPolicies, value);
	}
	if let Some(value) = options.x_xss_protection {
		set_header(res, Header::XXSSProtection, value);
	}
	if let Some(value) = options.permissions_policy {
		set_header(res, Header::PermissionsPolicy, value);
	}
	if options.remove_powered_by {
		res.headers_mut().remove("Powered-By");
	}
}

*/
fn is_localhost(url: &Uri) -> bool {
	let localhosts: HashSet<&'static str> = HashSet::from_iter(["localhost", "127.0.0.1", "[::1]"]);
	if let Some(host) = url.host() {
		return localhosts.contains(host);
	}
	return false;
}

fn resource_origin(url: &Uri) -> String {
	if is_localhost(url) {
		if let Some(scheme) = url.scheme() {
			if let Some(host) = url.host() {
				return format!("{scheme}://{host}"); // dev
			}
		}
	}
	return String::from("https://average.name"); // prod
}
