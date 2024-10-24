import type { MiddlewareHandler } from "hono";
import { config } from "../config.ts";
import { factory } from "../factories/factory.ts";
import { secureHeaders } from "hono/secure-headers";

/**
 * Sets security-related response headers.
 */
export function securityHeaders(): MiddlewareHandler {
	// TODO: Validate https://csp-evaluator.withgoogle.com/?csp=https://average.name
	// TODO: Validate https://observatory.mozilla.org/analyze/average.name
	return factory.createMiddleware(async (c, next) => {
		await next();

		// We need to set script-src-elem dynamically, because
		// (1) XML styles somehow read as a script,
		// (2) we don't know which origin the request comes from until this spot, and
		// (3) I don't want to permit localhost resources in production.
		const url = new URL(c.req.url);
		const rssStylesSrc = `${resourceOrigin(url)}/rss/styles.xsl`;
		const sitemapStylesSrc = `${resourceOrigin(url)}/sitemap/styles.xsl`;

		secureHeaders({
			// Hono follows https://github.com/w3c/webappsec-permissions-policy/blob/main/features.md
			permissionsPolicy: {
				accelerometer: [],
				allScreensCapture: [],
				ambientLightSensor: [],
				attributionReporting: [],
				autoplay: [],
				battery: [],
				bluetooth: [],
				browsingTopics: [],
				camera: [],
				capturedSurfaceControl: [],
				clipboardRead: [],
				clipboardWrite: [],
				crossOriginIsolated: [],
				digitalCredentialsGet: [],
				directSockets: [],
				displayCapture: [],
				// documentDomain: [], // TODO: Do we need this one?
				encryptedMedia: [],
				executionWhileNotRendered: [],
				executionWhileOutOfViewport: [],
				focusWithoutUserActivation: [],
				fullscreen: true,
				gemepad: [],
				geolocation: [],
				gyroscope: [],
				hid: [],
				identityCredentialsGet: [],
				idleDetection: [],
				// interestCohort: [], // TODO: Does this one exist?
				joinAdInterestGroup: [],
				keyboardMap: [],
				localFonts: [],
				magnetometer: [],
				microphone: [],
				midi: [],
				navigationOverride: [],
				payment: [],
				pictureInPicture: true,
				// publickeyCredentialsCreate: [], // TODO: Does this one exist?
				publickeyCredentialsGet: [],
				runAdAuction: [],
				screenWakeLock: [],
				serial: [],
				sharedAutofill: [],
				smartCard: [],
				speakerSelection: [],
				storageAccess: [],
				syncScript: [],
				syncXhr: [],
				trustTokenRedemption: [],
				unload: [],
				usb: [],
				verticalScroll: ["self"],
				webShare: true,
				windowManagement: [],
				xrSpatialTracking: [],
			},
			contentSecurityPolicy: {
				baseUri: ["'none'"],
				// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/default-src
				defaultSrc: ["'none'"],
				formAction: ["'self'"],
				frameAncestors: ["'none'"],
				imgSrc: ["'self'", "https://*", "data:"],
				sandbox: ["allow-same-origin", "allow-downloads", "allow-forms", "allow-scripts"], // allow-scripts is only for rss/styles.xsl
				// TODO: Ditch unsafe-inline. See https://astro-shield.kindspells.dev/guides/subresource-integrity/static-sites/
				styleSrc: ["'self'", "'unsafe-inline'"],
				mediaSrc: ["'none'"],
				// mediaSrc: ["data:"], // Firefox wants this for some reason, but the error FF throws is benign, so leaving it for now.
				scriptSrcElem: [rssStylesSrc, sitemapStylesSrc], // Specifically enable XML stylesheets
				upgradeInsecureRequests: [],
			},
			crossOriginEmbedderPolicy: "require-corp",
			crossOriginResourcePolicy:
				// Specifically allow other domains to access the fursona avatar
				url.pathname === "/images/refs/AverageHelper-avatar.png" ? "cross-origin" : "same-origin",
			crossOriginOpenerPolicy: "same-origin",
			originAgentCluster: "?1",
			referrerPolicy: "no-referrer",
			strictTransportSecurity: "max-age=31536000; includeSubDomains; preload",
			xContentTypeOptions: "nosniff",
			xDnsPrefetchControl: "off",
			xDownloadOptions: "noopen",
			xFrameOptions: "DENY",
			xPermittedCrossDomainPolicies: "none",
			xXssProtection: "1; mode=block",
		})(c, () => Promise.resolve());
	});
}

function isLocalhost(url: URL): boolean {
	return [
		`http://localhost:${config.port}`,
		`http://127.0.0.1:${config.port}`,
		`http://[::1]:${config.port}`,
	].includes(url.origin);
}

function resourceOrigin(url: URL): string {
	return isLocalhost(url)
		? url.origin // dev
		: "https://average.name"; // prod
}
