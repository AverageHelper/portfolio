import type { MiddlewareHandler } from "hono/mod.ts";
import { badRequest, notFound } from "./utils/responses.ts";
import {
	compress,
	cors as _cors,
	secureHeaders,
	serveStatic,
	trimTrailingSlash,
} from "hono/middleware.ts";
import { Hono } from "hono/mod.ts";

// All requests to the average.name domain route here first.
// The last handler falls back to serving the contents of /dist.
// Static content should be served there primarily.
// Dynamic content is served here.

const PRONOUNS_EN = "she/her";

const clacks = ["Terry Pratchett", "Nex Benedict"] as const;

const wellKnownSubdomains = [
	// Subdomains that I want to give a *.avg.name alias:
	"blog",
	"flashcards",
	"git",
	"ip",
	"ipv4",
	"www",
] as const;

const wellKnownAliasDomains: ReadonlySet<string> = new Set(
	wellKnownSubdomains.map(s => `${s}.avg.name`),
);

function randomElementOfArray<T>(array: readonly [T, ...ReadonlyArray<T>]): T {
	const index = Math.floor(Math.random() * array.length);
	return array[index] ?? array[0];
}

function randomClacks(): `GNU ${string}` {
	const name = randomElementOfArray(clacks);
	return `GNU ${name}`;
}

const PORT = 8787;

function cors(): MiddlewareHandler {
	return _cors({
		origin: [`http://localhost:${PORT}`, "https://average.name"],
	});
}

const app = new Hono({ strict: true })
	.use(compress())
	.use(trimTrailingSlash())

	// ** Additional headers
	.use(
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
	)
	.use(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		// See https://www.andrewyu.org/article/x-pronouns.html
		res.headers.set("X-Pronouns-Acceptable", `en:${PRONOUNS_EN}`);
		res.headers.set("X-Clacks-Overhead", randomClacks());

		// Disable cache (for now)
		res.headers.set("Vary", "*");

		// Security
		res.headers.set(
			"Permissions-Policy",
			"accelerometer=(), ambient-light-sensor=(), autoplay=(), battery=(), camera=(), clipboard-read=(), clipboard-write=(), cross-origin-isolated=(), display-capture=(), document-domain=(), encrypted-media=(), execution-while-not-rendered=(), execution-while-out-of-viewport=(), fullscreen=*, gamepad=(), geolocation=(), gyroscope=(), identity-credentials-get=(), idle-detection=(), interest-cohort=(), keyboard-map=(), local-fonts=(), magnetometer=(), microphone=(), midi=(), navigation-override=(), payment=(), picture-in-picture=*, publickey-credentials-create=(), publickey-credentials-get=(), screen-wake-lock=(), serial=(), speaker-selection=(), storage-access=(), sync-xhr=(), usb=(), web-share=*, xr-spatial-tracking=()",
		);

		c.res = undefined;
		c.res = res;
	})

	// ** Redirects
	.get("/ip", c => c.redirect("https://ip.average.name", 302))

	.get("/@avg", c => c.redirect("https://fosstodon.org/@avghelper", 302))
	.get("/@avghelper", c => c.redirect("https://fosstodon.org/@avghelper", 302))
	.get("/@average", c => c.redirect("https://fosstodon.org/@avghelper", 302))

	.get("/how", c => c.redirect("/ways", 302))
	.get("/how.html", c => c.redirect("/ways.html", 302))

	.get("/bookmarks", c => c.redirect("/links", 302))
	.get("/bookmarks.html", c => c.redirect("/links.html", 302))

	// ** Pronouns
	.get("/pronouns", c => c.redirect("/.well-known/pronouns", 302))
	.get("/.well-known/pronouns", cors(), c => c.text(`${PRONOUNS_EN}\n`))

	// ** Tell GitHub about our WebFinger proxy coolness
	.get("/.well-known/nodeinfo", cors(), c => {
		// Who's asking?
		const userAgent = c.req.header("User-Agent");
		if (!userAgent || !userAgent.startsWith("GitHub-NodeinfoQuery")) return c.notFound();

		// GitHub is asking. Pretend we're Mastodon:
		return c.redirect("https://fosstodon.org/.well-known/nodeinfo", 302);
	})

	// ** Webfinger
	// See https://www.rfc-editor.org/rfc/rfc7033.html
	.get("/.well-known/webfinger", cors(), c => {
		// "If the "resource" parameter is absent or malformed, [...] indicate that the request is bad"
		const resourceQuery = c.req.query("resource");
		if (!resourceQuery) return badRequest();
		const resourceUri = url(resourceQuery);
		if (!resourceUri) return badRequest();

		const resource =
			resourceUri.protocol === "acct:" ? url(resourceUri.pathname) ?? resourceUri.pathname : null;
		if (!resource) return badRequest();

		// "If the "resource" parameter is a value for which the server has no information, the server MUST indicate [not found]"
		const host = typeof resource === "string" ? resource.split("@").at(-1) : resource.host;
		if (host !== "average.name") return notFound();

		const relQueries = c.req.queries("rel") ?? [];

		const availableLinks = [
			{
				rel: "http://webfinger.net/rel/profile-page",
				type: "text/html",
				href: "https://fosstodon.org/@avghelper",
			},
			{
				rel: "self",
				type: "application/activity+json",
				href: "https://fosstodon.org/users/avghelper",
			},
			{
				rel: "http://ostatus.org/schema/1.0/subscribe", // Seems ostatus.org is no more, but Mastodon's docs still reference it
				template: "https://fosstodon.org/authorize_interaction?uri={uri}",
			},
		] as const;

		// "When the "rel" parameter is used and accepted, only the link relation types that match the link relation type provided via the "rel" parameter are included."
		const links =
			relQueries.length === 0
				? availableLinks
				: availableLinks.filter(link => relQueries.includes(link.rel));

		return c.json(
			{
				// subject: "acct:average@average.name",
				subject: "acct:avghelper@fosstodon.org",
				aliases: [
					"https://average.name/@average",
					"https://average.name/@avg",
					"https://average.name/@avghelper",
					"https://fosstodon.org/@avghelper",
					"https://fosstodon.org/users/avghelper",
				],
				links,
			},
			200,

			// "The media type used for the JSON Resource Descriptor (JRD) is `application/jrd+json`"
			{ "Content-Type": "application/jrd+json; charset=UTF=8" },
		);
	})

	// ** Caddy On-Demand TLS
	.get(".well-known/domains", c => {
		// See https://caddyserver.com/docs/automatic-https#on-demand-tls
		const domain = c.req.query("domain");
		if (!domain) return badRequest();

		if (domain === "avg.name") return new Response(null, { status: 204 });
		if (wellKnownAliasDomains.has(domain)) return new Response(null, { status: 204 });
		return new Response(null, { status: 404 }); // not found
	})

	// ** Serve the /dist dir
	.get(
		"/*",
		cors(),
		serveStatic({
			root: "./dist",
			rewriteRequestPath(path) {
				// Hono by default expects the path to be either a directory or a file, but won't check for .html
				if (path === "/") return path;
				if (path.includes(".")) return path;
				return `${path}.html`;
			},
		}),
	)

	.notFound(c => c.redirect("/404.html"));

function url(str: string): URL | null {
	try {
		return new URL(str);
	} catch {
		return null;
	}
}

Deno.serve({ port: PORT, hostname: "localhost" }, app.fetch);
