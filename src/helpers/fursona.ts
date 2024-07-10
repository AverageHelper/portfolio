import type { Infer, Struct } from "superstruct";
import { array, define, min, number, optional, size, string, type } from "superstruct";
import { Temporal } from "temporal-polyfill";

// ** Validation logic based on https://git.sr.ht/~gmem/well-known-fursona/tree/5a0c47d9db323523a28cc5a39ff2f8e78de46aef/item/src/lib/Fursona.ts

/** A single fursona. */
export const sonaDescription = type({
	/** The name of this fursona. */
	name: optional(string()),

	/** This fursona's pronouns. */
	pronouns: optional(string()),

	/** This fursona's gender. */
	gender: optional(string()),

	/** This fursona's species. */
	species: optional(string()),

	/** Any additional notes or links for this fursona. (Up to 250 characters.) */
	description: optional(size(string(), 0, 250)), // Can this be Markdown?

	/** A link to a ref for this fursona. Should be an image file. */
	ref: optional(url()),

	/** Alt text to be added to the ref sheet image. Not required but highly encouraged. */
	refAlt: optional(string()),

	/** (Non-standard.) The artist of the ref and avatar. */
	refArtist: optional(
		type({
			name: string(),
			url: url(),
		}),
	),

	/** An image file to use as an avatar, if needed. */
	avatar: optional(url()),

	/** Alt text for the avatar. Like refAlt, not required but highly encouraged. */
	avatarAlt: optional(string()),

	/** This fursona's age. */
	age: optional(min(number(), 0)),

	/** The birthdate of the fursona. */
	birthdate: optional(iso8601()),

	/** The fursona's colors. Should be an array of 3 or 6-digit hex codes, which can be prefixed with a hashtag. */
	colors: optional(array(colorHex())),
});

/** A single fursona. */
export type SonaDescription = Infer<typeof sonaDescription>;

/** A description of someone's fursona(s). */
export const fursonaSchema = type({
	/** The list of fursonas. */
	sonas: optional(array(sonaDescription)),

	/** (Non-standard.) The canonical URL of this published fursona definition. */
	canonical: optional(string()),

	/** (Non-standard.) The date at which this fursona definition was last modified. */
	dateModified: optional(iso8601()),
});

/** A description of someone's fursona(s). */
export type FursonaSchema = Infer<typeof fursonaSchema>;

/**
 * @param date The date string to check.
 * @returns `true` if the given string can form a valid {@link Temporal.PlainDate}.
 */
export function isISO8601(date: string): boolean {
	try {
		Temporal.PlainDate.from(date);
		return true;
	} catch {
		return false;
	}
}

/**
 * @param url The URL string to check.
 * @returns `true` if the given string can form a valid {@link URL}.
 */
export function isURL(url: string): boolean {
	try {
		new URL(url);
		return true;
	} catch {
		return false;
	}
}

function url(): Struct<string, null> {
	return define("URL", value => {
		return (
			typeof value === "string" &&
			isURL(value) &&
			["http:", "https:", "gemini:", "ftp:"].includes(new URL(value).protocol)
		);
	});
}

function iso8601(): Struct<string, null> {
	return define("ISO8601 Date", value => {
		return typeof value === "string" && isISO8601(value);
	});
}

function colorHex(): Struct<string, null> {
	return define("color hex string", value => {
		return typeof value === "string" && /^#([0-9a-fA-F]{6}|[0-9a-fA-F]{3})$/iu.test(value);
	});
}
