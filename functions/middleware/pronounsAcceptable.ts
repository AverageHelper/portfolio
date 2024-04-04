import type { MiddlewareHandler } from "hono/mod.ts";
import { createMiddleware } from "hono/helper.ts";

export const PRONOUNS_EN = "she/her";

/**
 * Sets the `X-Pronouns-Acceptable` response header.
 */
export function pronounsAcceptable(): MiddlewareHandler {
	return createMiddleware(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		// See https://www.andrewyu.org/article/x-pronouns.html
		res.headers.set("X-Pronouns-Acceptable", `en:${PRONOUNS_EN}`);

		c.res = undefined;
		c.res = res;
	});
}
