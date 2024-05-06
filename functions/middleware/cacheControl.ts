import type { MiddlewareHandler } from "hono/mod.ts";
import { factory } from "../factories/factory.ts";

/**
 * Sets cache control headers.
 */
export function cacheControl(): MiddlewareHandler {
	return factory.createMiddleware(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		// Disable cache (for now)
		res.headers.set("Vary", "*");

		c.res = undefined;
		c.res = res;
	});
}
