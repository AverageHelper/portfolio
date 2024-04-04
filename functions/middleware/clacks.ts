import type { MiddlewareHandler } from "hono/mod.ts";
import { createMiddleware } from "hono/helper.ts";

const NAMES = ["Terry Pratchett", "Nex Benedict"] as const;

/**
 * Sets the `X-Clacks-Overhead` response header.
 */
export function clacks(): MiddlewareHandler {
	return createMiddleware(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		res.headers.set("X-Clacks-Overhead", randomClacks());

		c.res = undefined;
		c.res = res;
	});
}

function randomElementOfArray<T>(array: readonly [T, ...ReadonlyArray<T>]): T {
	const index = Math.floor(Math.random() * array.length);
	return array[index] ?? array[0];
}

function randomClacks(): `GNU ${string}` {
	const name = randomElementOfArray(NAMES);
	return `GNU ${name}`;
}
