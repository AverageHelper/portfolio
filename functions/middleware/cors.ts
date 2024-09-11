import type { MiddlewareHandler } from "hono";
import { cors as _cors } from "hono/cors";

export function cors(originOverride?: string): MiddlewareHandler {
	return _cors({
		origin: originOverride ?? "https://average.name",
	});
}
