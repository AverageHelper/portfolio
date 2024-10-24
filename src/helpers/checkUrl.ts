/* eslint-disable no-console */

import { FgBlue, FgCyan, FgGreen, FgRed, Reset } from "./consoleColors.js";

const alreadyOk = new Set<string>([
	"https://ko-fi.com/avghelper", // answers 403 to the CI, so check this manually
	"https://ko-fi.com/decemberbreezee", // answers 403 to the CI
	"https://bsky.app/profile/did:plc:zxthjxcmxpjl372uwgrm6dxi", // answers 404 to the CI
]);

/**
 * Resolves if the given URL is accessible. Throws a {@link TypeError} otherwise.
 *
 * Only runs when the `MODE` environment variable is `production`.
 *
 * @param src The URL to check.
 */
export async function checkUrl(src: URL): Promise<void> {
	if (import.meta.env["MODE"] !== "production") {
		// Only run when building
		return;
	}

	// Check cache, make sure we haven't already checked this URL
	if (alreadyOk.has(src.href)) {
		console.info(`href ${FgBlue}'${src.href}'${FgGreen} OK ${FgCyan}(skipped)${Reset}`);
		return;
	}

	try {
		// Try fetch with HEAD method
		const result = await fetch(src, { method: "HEAD" });
		let didRedirect = result.redirected;
		if (result.status === 405) {
			// HEAD isn't allowed for some reason. Try GET
			const result2 = await fetch(src, { method: "GET" });
			didRedirect = result2.redirected;
			if (!result2.ok) throw result2.status;
		} else if (!result.ok) {
			throw result.status;
		}

		// Success! Remember this URL for later
		alreadyOk.add(src.href);
		if (didRedirect) {
			console.info(`href ${FgBlue}'${src.href}'${FgGreen} OK ${FgCyan}(redirected)${Reset}`);
		} else {
			console.info(`href ${FgBlue}'${src.href}'${FgGreen} OK${Reset}`);
		}
	} catch (error) {
		if (typeof error === "number") {
			// Got non-OK HTTP response
			console.info(`href ${FgBlue}'${src.href}'${FgRed} HTTP ${error}${Reset}`);
			throw new TypeError(`${FgRed}Link href '${src.href}' is broken: ${error}${Reset}`);
		} else if (error instanceof Error) {
			// Got network error
			console.info(`href ${FgBlue}'${src.href}'${FgRed} Network Error: ${error.message}${Reset}`);
			throw new TypeError(`${FgRed}Link href '${src.href}' is broken: ${error.message}${Reset}`);
		} else {
			// Got unknown error
			const message = JSON.stringify(error);
			console.info(`href ${FgBlue}'${src.href}'${FgRed} Unknown Error: ${message}${Reset}`);
			throw new TypeError(`${FgRed}Link href '${src.href}' is broken: ${message}${Reset}`);
		}
	}
}
