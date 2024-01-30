/* eslint-disable no-console */

import { FgBlue, FgCyan, FgGreen, FgRed, Reset } from "./consoleColors.js";

const alreadyOk = new Set<string>();

/**
 * Resolves if the given URL is accessible. Throws a {@link TypeError} otherwise.
 *
 * Only runs when the `MODE` environment variable is `production`.
 *
 * @param src The URL to check.
 */
export async function checkUrl(src: URL): Promise<void> {
	if (import.meta.env["MODE"] !== "production") {
		// Only run in when building
		return;
	}

	// Check cache, make sure we haven't already checked this URL
	if (alreadyOk.has(src.href)) {
		console.info(`href ${FgBlue}'${src.href}'${FgGreen} OK (skipped)${Reset}`);
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
			console.info(`href ${FgBlue}'${src.href}'${FgCyan} OK (redirected)${Reset}`);
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
