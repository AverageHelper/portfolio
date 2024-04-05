import { cacheControl } from "./middleware/cacheControl.ts";
import { clacks } from "./middleware/clacks.ts";
import { compress, serveStatic, trimTrailingSlash } from "hono/middleware.ts";
import { config } from "./config.ts";
import { cors } from "./middleware/cors.ts";
import { Hono } from "hono/mod.ts";
import { ifNotTesting } from "./utils/ifNotTesting.ts";
import { nodeinfo, webfinger } from "./factories/webfinger.ts";
import { onDemandTls } from "./factories/onDemandTls.ts";
import { pronounsAcceptable, PRONOUNS_EN } from "./middleware/pronounsAcceptable.ts";
import { securityHeaders } from "./middleware/securityHeaders.ts";

// All requests to the average.name domain route here first.
// The last handler falls back to serving the contents of /dist.
// Static content should be served there primarily, built using Astro.
// Dynamic content is served here using Deno.

export const app = new Hono({ strict: true })
	.use(compress())
	.use(trimTrailingSlash())
	.use(...securityHeaders)
	.use(cacheControl())
	.use(clacks())
	.use(pronounsAcceptable())

	.get("/ip", c => c.redirect("https://ip.average.name", 302))

	.get("/how", c => c.redirect("/ways", 302))
	.get("/how.html", c => c.redirect("/ways.html", 302))

	.get("/bookmarks", c => c.redirect("/links", 302))
	.get("/bookmarks.html", c => c.redirect("/links.html", 302))

	// ** Pronouns
	.get("/pronouns", c => c.redirect("/.well-known/pronouns", 302))
	.get("/.well-known/pronouns", cors(), c => c.text(`${PRONOUNS_EN}\n`))

	// ** Fediverse aliases
	.get("/@avg", c => c.redirect("https://fosstodon.org/@avghelper", 302))
	.get("/@avghelper", c => c.redirect("https://fosstodon.org/@avghelper", 302))
	.get("/@average", c => c.redirect("https://fosstodon.org/@avghelper", 302))
	.get("/.well-known/webfinger", ...webfinger)
	.get("/.well-known/nodeinfo", ...nodeinfo)

	// ** Caddy On-Demand TLS
	.get(".well-known/domains", ...onDemandTls)

	// ** Serve the /dist dir
	.get(
		"/*",
		cors(),
		serveStatic({
			// TODO: Use import.meta.dirname to resolve dist/
			root: "./dist", // relative to working directory, I think
			rewriteRequestPath(path) {
				// Hono by default expects the path to be either a directory or a file, but won't check for .html
				if (path === "/") return path;
				if (path.includes(".")) return path;
				return `${path}.html`;
			},
		}),
	)

	.notFound(async c => {
		// TODO: Use import.meta.dirname to resolve dist/
		const file = await Deno.readTextFile("./dist/404.html"); // relative to working directory, I think
		return c.html(file, 404);
	});

await ifNotTesting(() => {
	const hostname = config.hostname;
	const port = config.port;
	Deno.serve({ hostname, port }, app.fetch);
});
