import { secureHeaders } from "hono/middleware.ts";
import { permissionsPolicy } from "./permissionsPolicy.ts";

/**
 * Sets security-related response headers.
 */
export const securityHeaders = [
	// TODO: Validate https://csp-evaluator.withgoogle.com/?csp=https://average.name
	// TODO: Validate https://observatory.mozilla.org/analyze/average.name
	permissionsPolicy(),
	secureHeaders({
		contentSecurityPolicy: {
			baseUri: ["'none'"],
			// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/default-src
			defaultSrc: ["'none'"],
			formAction: ["'self'"],
			frameAncestors: ["'none'"],
			imgSrc: ["'self'", "https://*", "data:"],
			sandbox: ["allow-downloads", "allow-forms"],
			// TODO: Ditch unsafe-inline. See https://github.com/KindSpells/astro-shield
			styleSrc: ["'self'", "'unsafe-inline'"],
			upgradeInsecureRequests: [],
		},
		crossOriginEmbedderPolicy: "require-corp", // FIXME: This and CORP makes CSS assets not load at all unless inlined, with even stricter reqs in Safari
		crossOriginResourcePolicy: "same-origin",
		crossOriginOpenerPolicy: "same-origin",
		originAgentCluster: "?1",
		referrerPolicy: "no-referrer",
		strictTransportSecurity: "max-age=31536000; includeSubDomains; preload",
		xContentTypeOptions: "nosniff",
		xDnsPrefetchControl: "off",
		xDownloadOptions: "noopen",
		xFrameOptions: "DENY",
		xPermittedCrossDomainPolicies: "none",
		xXssProtection: "0",
	}),
] as const;
