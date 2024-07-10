import type { MiddlewareHandler } from "hono/mod.ts";
import { cors as _cors } from "hono/middleware.ts";

export function cors(originOverride?: string): MiddlewareHandler {
	return _cors({
		origin: originOverride ?? "https://average.name",
	});
}
