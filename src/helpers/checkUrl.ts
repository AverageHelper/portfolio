/* eslint-disable no-console */

import { FgBlue, FgCyan, FgGreen, FgRed, Reset } from "./consoleColors.js";

const alreadyOk = new Set<string>([
	"https://ko-fi.com/avghelper", // answers 403 to the CI, so check this manually
	"https://ko-fi.com/decemberbreezee", // answers 403 to the CI
	"https://bsky.app/profile/did:plc:zxthjxcmxpjl372uwgrm6dxi", // answers 404 to the CI
]);
const ownDomain = "https://average.name";

/**
 * Resolves if the given URL is accessible. Throws a {@link TypeError} otherwise.
 *
 * Only runs when the `MODE` environment variable is `production`.
 *
 * @param url The URL to check.
 */
export async function checkUrl(url: URL): Promise<void> {
	if (import.meta.env["MODE"] !== "production") {
		// Only run when building
		return;
	}

	// Check cache, make sure we haven't already checked this URL
	if (alreadyOk.has(url.href) || url.href.startsWith(ownDomain)) {
		console.info(`\nhref ${FgBlue}'${url.href}'${FgGreen} OK ${FgCyan}(skipped)${Reset}`);
		return;
	}

	try {
		// Try fetch with HEAD method
		const result = await fetch(url, { method: "HEAD" });
		let didRedirect = result.redirected;
		if (result.status === 405) {
			// HEAD isn't allowed for some reason. Try GET
			const result2 = await fetch(url, { method: "GET" });
			didRedirect = result2.redirected;
			if (!result2.ok) throw result2.status; // eslint-disable-line @typescript-eslint/only-throw-error
		} else if (!result.ok) {
			throw result.status; // eslint-disable-line @typescript-eslint/only-throw-error
		}

		// Success! Remember this URL for later
		alreadyOk.add(url.href);
		if (didRedirect) {
			console.info(`\nhref ${FgBlue}'${url.href}'${FgGreen} OK ${FgCyan}(redirected)${Reset}`);
		} else {
			console.info(`\nhref ${FgBlue}'${url.href}'${FgGreen} OK${Reset}`);
		}
	} catch (error) {
		if (typeof error === "number") {
			// Got non-OK HTTP response
			console.info(`\nhref ${FgBlue}'${url.href}'${FgRed} HTTP ${error}${Reset}`);
			throw new TypeError(`${FgRed}Link href '${url.href}' is broken: ${error}${Reset}`);
		} else if (error instanceof Error) {
			// Got network error
			console.info(`\nhref ${FgBlue}'${url.href}'${FgRed} Network Error: ${error.message}${Reset}`);
			throw new TypeError(`${FgRed}Link href '${url.href}' is broken: ${error.message}${Reset}`);
		} else {
			// Got unknown error
			const message = JSON.stringify(error);
			console.info(`\nhref ${FgBlue}'${url.href}'${FgRed} Unknown Error: ${message}${Reset}`);
			throw new TypeError(`${FgRed}Link href '${url.href}' is broken: ${message}${Reset}`);
		}
	}
}
