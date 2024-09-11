import type { MiddlewareHandler } from "hono";
import { factory } from "../factories/factory.ts";

/**
 * Sets the `Permissions-Policy` response header.
 */
export function permissionsPolicy(): MiddlewareHandler {
	return factory.createMiddleware(async (c, next) => {
		await next();
		const res = new Response(c.res.body, c.res);

		// Security
		res.headers.set(
			"Permissions-Policy",
			"accelerometer=(), ambient-light-sensor=(), autoplay=(), battery=(), camera=(), clipboard-read=(), clipboard-write=(), cross-origin-isolated=(), display-capture=(), document-domain=(), encrypted-media=(), execution-while-not-rendered=(), execution-while-out-of-viewport=(), fullscreen=*, gamepad=(), geolocation=(), gyroscope=(), identity-credentials-get=(), idle-detection=(), interest-cohort=(), keyboard-map=(), local-fonts=(), magnetometer=(), microphone=(), midi=(), navigation-override=(), payment=(), picture-in-picture=*, publickey-credentials-create=(), publickey-credentials-get=(), screen-wake-lock=(), serial=(), speaker-selection=(), storage-access=(), sync-xhr=(), usb=(), web-share=*, xr-spatial-tracking=()",
		);

		c.res = undefined;
		c.res = res;
	});
}
