import type { MiddlewareHandler } from "hono";
import { factory } from "../factories/factory.ts";
import { randomName } from "../utils/memorials.ts";

/**
 * Sets the `X-Clacks-Overhead` response header.
 */
export function clacks(): MiddlewareHandler {
	return factory.createMiddleware(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		const name = randomName();
		res.headers.set("X-Clacks-Overhead", `GNU ${name}`);

		c.res = undefined;
		c.res = res;
	});
}
