import { cors } from "hono/cors";
import { Hono } from "hono";
import { serveStatic } from "hono/cloudflare-pages";

// All requests to the average.name domain route here first.
// The last handler falls back to serving the contents of /dist.
// Static content should be served there primarily.
// Dynamic content is served here.

const PRONOUNS_EN = "she/her";

const clacks = ["Terry Pratchett", "Nex Benedict"] as const;

function randomElementOfArray<T>(array: readonly [T, ...ReadonlyArray<T>]): T {
	const index = Math.floor(Math.random() * array.length);
	return array[index] ?? array[0];
}

function randomClacks(): `GNU ${string}` {
	const name = randomElementOfArray(clacks);
	return `GNU ${name}`;
}

const app = new Hono()
	// ** Additional headers
	.use(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		// See https://www.andrewyu.org/article/x-pronouns.html
		res.headers.set("X-Pronouns-Acceptable", `en:${PRONOUNS_EN}`);
		res.headers.set("X-Clacks-Overhead", randomClacks());

		c.res = undefined;
		c.res = res;
	})

	// ** Pronouns
	.get("/.well-known/pronouns", cors(), c => c.text(`${PRONOUNS_EN}\n`))

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
				// subject: "acct:avg@average.name",
				subject: "acct:avghelper@fosstodon.org",
				aliases: [
					"https://average.name/@avg",
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

	// ** Serve the /dist dir
	.get("*", serveStatic())

	.notFound(c => c.redirect("/404.html"));

//#region Utilities
function url(str: string): URL | null {
	try {
		return new URL(str);
	} catch {
		return null;
	}
}

function notFound(): Response {
	return new Response("404 Not Found", { status: 404 });
}

function badRequest(): Response {
	return new Response("400 Bad Request", { status: 400 });
}
//#endregion

// The filename [[foo]].ts means we accept any path here.
// See https://developers.cloudflare.com/pages/platform/functions/get-started/
export const onRequest: PagesFunction = async c => {
	return await app.fetch(c.request, c.env, c);
};
