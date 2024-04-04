import { badRequest, notFound } from "../utils/responses.ts";
import { createFactory } from "hono/helper.ts";
import { cors } from "../middleware/cors.ts";

const factory = createFactory();

/**
 * Answers Webfinger requests.
 * @see https://www.rfc-editor.org/rfc/rfc7033.html
 */
export const webfinger = factory.createHandlers(cors(), c => {
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
});

/**
 * Tells GitHub about our Fedi redirect.
 */
export const nodeinfo = factory.createHandlers(cors(), c => {
	// Who's asking?
	const userAgent = c.req.header("User-Agent");
	if (!userAgent || !userAgent.startsWith("GitHub-NodeinfoQuery")) return c.notFound();

	// GitHub is asking. Pretend we're Mastodon:
	return c.redirect("https://fosstodon.org/.well-known/nodeinfo", 302);
});

function url(str: string): URL | null {
	try {
		return new URL(str);
	} catch {
		return null;
	}
}
