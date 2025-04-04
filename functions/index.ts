import { cacheControl } from "./middleware/cacheControl.ts";
import { clacks } from "./middleware/clacks.ts";
import { compress } from "hono/compress";
import { config } from "./config.ts";
import { cors } from "./middleware/cors.ts";
import { factory } from "./factories/factory.ts";
import { nodeinfo, webfinger } from "./factories/webfinger.ts";
import { onDemandTls } from "./factories/onDemandTls.ts";
import { pronounsAcceptable, PRONOUNS_EN } from "./middleware/pronounsAcceptable.ts";
import { securityHeaders } from "./middleware/securityHeaders.ts";
import { serveStatic } from "hono/deno";
import { trimTrailingSlash } from "hono/trailing-slash";

// All requests to the average.name domain route here first.
// The last handler falls back to serving the contents of /dist.
// Static content should be served there primarily, built using Astro.
// Dynamic content is served here using Deno.

export const app = factory
	.createApp()
	.use(compress())
	.use(trimTrailingSlash())
	.use(securityHeaders())
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
	.get("/.well-known/pronouns", cors("*"), c => c.text(`${PRONOUNS_EN}\n`))

	// ** Fursona
	.get("/fursona.json", c => c.redirect("/.well-known/fursona.json", 302))
	.get("/.well-known/fursona", c => c.redirect("/.well-known/fursona.json", 302))
	.get(
		"/images/refs/AverageHelper-avatar.png",
		cors("*"),
		serveStatic({ path: "./dist/images/refs/AverageHelper-avatar.png" }),
	)
	.get(
		"/.well-known/fursona.json",
		cors("*"),
		serveStatic({ path: "./dist/.well-known/fursona.json" }),
	)

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
		if (c.req.path.endsWith("favicon.ico")) {
			return c.text("Not found", 404);
		}

		// TODO: Use import.meta.dirname to resolve dist/
		const file = await Deno.readTextFile("./dist/404.html"); // relative to working directory, I think
		return c.html(file, 404);
	});

const port = config.port;
try {
	Deno.serve({ port }, app.fetch);
} catch (error) {
	if (!(error instanceof Deno.errors.AddrInUse)) {
		// Unknown error
		throw error;
	}

	// If our port is in use, say something, rather than throwing a nebulous error:
	console.error(`%cPort ${config.port} is already in use!`, "color: red; font-weight: bold");
	Deno.exit(1);
}
