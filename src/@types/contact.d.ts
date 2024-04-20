// These types must be explicitly exported and imported, not just
// defined on global scope, because of weirdness with Astro and
// type parsing.

import type { PlatformName } from "@/helpers/logos.js";

export interface ContactInfo {
	/**
	 * The name of the contact platform where I may be found.
	 */
	platform: PlatformName;

	/**
	 * My unique identifier on the platform, or an explanatory note.
	 */
	handle: string;

	/**
	 *  A URL by which I can be reached on the platform.
	 */
	href?: string;

	/**
	 * Whether to omit `rel="me"` from the link.
	 */
	notMe?: true;

	/**
	 * Whether the `handle` is an explanatory note (and therefore should be rendered in a mixed-width font).
	 */
	note?: true;
}

export type NonEmptyArray<T> = [T, ...Array<T>];

export type ReadonlyNonEmptyArray<T> = readonly [T, ...ReadonlyArray<T>];
