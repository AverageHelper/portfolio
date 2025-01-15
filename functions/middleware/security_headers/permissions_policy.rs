// See https://github.com/w3c/webappsec-permissions-policy/blob/main/features.md
// Based on https://github.com/honojs/hono/blob/v4.6.8/src/middleware/secure-headers/permissions-policy.ts

#[derive(Eq, PartialEq)]
pub enum Directive {
	/**
	 * Standardized features:
	 * These features have been declared in a published version of the respective specification.
	 */
	Accelerometer,
	AmbientLightSensor,
	AttributionReporting,
	Autoplay,
	Battery,
	Bluetooth,
	Camera,
	ChUa,
	ChUaArch,
	ChUaBitness,
	ChUaFullVersion,
	ChUaFullVersionList,
	ChUaMobile,
	ChUaModel,
	ChUaPlatform,
	ChUaPlatformVersion,
	ChUaWow64,
	ComputePressure,
	CrossOriginIsolated,
	DirectSockets,
	DisplayCapture,
	EncryptedMedia,
	ExecutionWhileNotRendered,
	ExecutionWhileOutOfViewport,
	Fullscreen,
	Geolocation,
	Gyroscope,
	Hid,
	IdentityCredentialsGet,
	IdleDetection,
	KeyboardMap,
	Magnetometer,
	Microphone,
	Midi,
	NavigationOverride,
	Payment,
	PictureInPicture,
	PublickeyCredentialsGet,
	ScreenWakeLock,
	Serial,
	StorageAccess,
	SyncXhr,
	Usb,
	WebShare,
	WindowManagement,
	XrSpatialTracking,

	/**
	 * Proposed features:
	 * These features have been proposed, but the definitions have not yet been integrated into their respective specs.
	 */
	ClipboardRead,
	ClipboardWrite,
	Gemepad,
	SharedAutofill,
	SpeakerSelection,

	/**
	 * Experimental features:
	 * These features generally have an explainer only, but may be available for experimentation by web developers.
	 */
	AllScreensCapture,
	BrowsingTopics,
	CapturedSurfaceControl,
	ConversionMeasurement,
	DigitalCredentialsGet,
	FocusWithoutUserActivation,
	JoinAdInterestGroup,
	LocalFonts,
	RunAdAuction,
	SmartCard,
	SyncScript,
	TrustTokenRedemption,
	Unload,
	VerticalScroll,
}

impl Directive {
	const fn to_str(&self) -> &'static str {
		match self {
			Self::Accelerometer => "accelerometer",
			Self::AmbientLightSensor => "ambientLightSensor",
			Self::AttributionReporting => "attributionReporting",
			Self::Autoplay => "autoplay",
			Self::Battery => "battery",
			Self::Bluetooth => "bluetooth",
			Self::Camera => "camera",
			Self::ChUa => "chUa",
			Self::ChUaArch => "chUaArch",
			Self::ChUaBitness => "chUaBitness",
			Self::ChUaFullVersion => "chUaFullVersion",
			Self::ChUaFullVersionList => "chUaFullVersionList",
			Self::ChUaMobile => "chUaMobile",
			Self::ChUaModel => "chUaModel",
			Self::ChUaPlatform => "chUaPlatform",
			Self::ChUaPlatformVersion => "chUaPlatformVersion",
			Self::ChUaWow64 => "chUaWow64",
			Self::ComputePressure => "computePressure",
			Self::CrossOriginIsolated => "crossOriginIsolated",
			Self::DirectSockets => "directSockets",
			Self::DisplayCapture => "displayCapture",
			Self::EncryptedMedia => "encryptedMedia",
			Self::ExecutionWhileNotRendered => "executionWhileNotRendered",
			Self::ExecutionWhileOutOfViewport => "executionWhileOutOfViewport",
			Self::Fullscreen => "fullscreen",
			Self::Geolocation => "geolocation",
			Self::Gyroscope => "gyroscope",
			Self::Hid => "hid",
			Self::IdentityCredentialsGet => "identityCredentialsGet",
			Self::IdleDetection => "idleDetection",
			Self::KeyboardMap => "keyboardMap",
			Self::Magnetometer => "magnetometer",
			Self::Microphone => "microphone",
			Self::Midi => "midi",
			Self::NavigationOverride => "navigationOverride",
			Self::Payment => "payment",
			Self::PictureInPicture => "pictureInPicture",
			Self::PublickeyCredentialsGet => "publickeyCredentialsGet",
			Self::ScreenWakeLock => "screenWakeLock",
			Self::Serial => "serial",
			Self::StorageAccess => "storageAccess",
			Self::SyncXhr => "syncXhr",
			Self::Usb => "usb",
			Self::WebShare => "webShare",
			Self::WindowManagement => "windowManagement",
			Self::XrSpatialTracking => "xrSpatialTracking",

			Self::ClipboardRead => "clipboardRead",
			Self::ClipboardWrite => "clipboardWrite",
			Self::Gemepad => "gemepad",
			Self::SharedAutofill => "sharedAutofill",
			Self::SpeakerSelection => "speakerSelection",

			Self::AllScreensCapture => "allScreensCapture",
			Self::BrowsingTopics => "browsingTopics",
			Self::CapturedSurfaceControl => "capturedSurfaceControl",
			Self::ConversionMeasurement => "conversionMeasurement",
			Self::DigitalCredentialsGet => "digitalCredentialsGet",
			Self::FocusWithoutUserActivation => "focusWithoutUserActivation",
			Self::JoinAdInterestGroup => "joinAdInterestGroup",
			Self::LocalFonts => "localFonts",
			Self::RunAdAuction => "runAdAuction",
			Self::SmartCard => "smartCard",
			Self::SyncScript => "syncScript",
			Self::TrustTokenRedemption => "trustTokenRedemption",
			Self::Unload => "unload",
			Self::VerticalScroll => "verticalScroll",
		}
	}
}

impl PartialOrd for Directive {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		return self.to_str().partial_cmp(other.to_str());
	}
}

impl Ord for Directive {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		return self.to_str().cmp(other.to_str());
	}
}
