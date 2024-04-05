import { createMiddleware } from "hono/helper.ts";
import { permissionsPolicy } from "./permissionsPolicy.ts";
import { secureHeaders } from "hono/middleware.ts";

/**
 * Sets security-related response headers.
 */
export const securityHeaders = [
	// TODO: Validate https://csp-evaluator.withgoogle.com/?csp=https://average.name
	// TODO: Validate https://observatory.mozilla.org/analyze/average.name
	permissionsPolicy(),
	createMiddleware(async (c, next) => {
		await next();

		// We need to set script-src-elem dynamically, because
		// (1) rss/styles.xsl somehow reads as a script,
		// (2) we don't know which origin the request comes from until here, and
		// (3) I don't want to permit localhost resources in production.
		const url = new URL(c.req.url);
		const origin = url.origin;
		const port = url.port; // TODO: Test that the port is correct
		const knownOrigins = [`http://localhost:${port}`, "https://average.name"];
		const rssStylesSrc = knownOrigins.includes(origin) ? `${origin}/rss/styles.xsl` : "'none'";

		secureHeaders({
			contentSecurityPolicy: {
				baseUri: ["'none'"],
				// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy/default-src
				defaultSrc: ["'none'"],
				formAction: ["'self'"],
				frameAncestors: ["'none'"],
				imgSrc: ["'self'", "https://*", "data:"],
				sandbox: ["allow-same-origin", "allow-downloads", "allow-forms", "allow-scripts"], // allow-scripts is only for rss/styles.xsl
				// TODO: Ditch unsafe-inline. See https://github.com/KindSpells/astro-shield
				styleSrc: ["'self'", "'unsafe-inline'"],
				scriptSrcElem: [rssStylesSrc], // Specifically enable XML stylesheet
				mediaSrc: ["'none'"],
				// mediaSrc: ["data:"], // Firefox wants this for some reason, but the error FF throws is benign, so leaving it for now.
				upgradeInsecureRequests: [],
			},
			crossOriginEmbedderPolicy: "require-corp",
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
			xXssProtection: "1; mode=block",
		})(c, () => Promise.resolve());
	}),
] as const;
