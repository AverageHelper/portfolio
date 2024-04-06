import { assert, assertEquals } from "std/assert/mod.ts";

export function assertHeaders(res: Response): void {
	assert(res.headers.has("Content-Security-Policy"));
	assert(res.headers.get("Content-Security-Policy")?.includes("upgrade-insecure-requests"));
	assertEquals(res.headers.get("Referrer-Policy"), "no-referrer");
	assert(res.headers.has("Strict-Transport-Security"));
	assert(res.headers.has("X-Content-Type-Options"));
	assert(res.headers.has("X-Frame-Options"));
	assert(res.headers.has("X-XSS-Protection"));
	assert(res.headers.has("Vary"));
	assert(res.headers.has("X-Clacks-Overhead"));
	assert(res.headers.has("X-Pronouns-Acceptable"));
}
