import { Hono } from "hono";
import { serveStatic } from "hono/cloudflare-pages";

const app = new Hono()
	.get("/", serveStatic()) // Serves the /pages dir
	.get("/.well-known/webfinger", c =>
		c.json({
			subject: "acct:avg@average.name",
			aliases: [
				"https://average.name",
				"https://fosstodon.org/@avghelper",
				"https://fosstodon.org/users/avghelper",
			],
			links: [
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
					rel: "http://ostatus.org/schema/1.0/subscribe",
					template: "https://fosstodon.org/authorize_interaction?uri={uri}",
				},
			],
		}),
	);

// The filename [[foo]].ts means we accept any path here.
// See https://developers.cloudflare.com/pages/platform/functions/get-started/
export const onRequest: PagesFunction = async c => {
	return await app.fetch(c.request, c.env, c);
};
