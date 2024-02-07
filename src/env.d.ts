/* eslint-disable @typescript-eslint/triple-slash-reference */
/// <reference path="../.astro/types.d.ts" />
/// <reference types="astro/client" />

// See https://docs.astro.build/en/guides/typescript/#built-in-html-attributes
// on adding additional HTML attributes.
declare namespace astroHTML.JSX {
	interface HTMLAttributes {
		"xmlns:cc"?: string;
		"xmlns:dct"?: string;
	}
}
