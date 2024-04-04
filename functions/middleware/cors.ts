import type { MiddlewareHandler } from "hono/mod.ts";
import { cors as _cors } from "hono/middleware.ts";

export function cors(): MiddlewareHandler {
	return _cors({
		origin: "https://average.name",
	});
}
