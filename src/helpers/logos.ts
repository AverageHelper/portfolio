/**
 * The list of supported logo platforms and their logos.
 */
export const logos = {
	bsky: {
		name: "Bluesky",
		path: "/images/logo-bsky.svg",
	},
	codeberg: {
		name: "Codeberg",
		path: "/images/logo-codeberg.svg",
	},
	discord: {
		name: "Discord",
		path: "/images/logo-discord.svg",
	},
	fedi: {
		name: "Fediverse",
		path: "/images/logo-fediverse.svg",
	},
	forgejo: {
		name: "Forgejo",
		path: "/images/logo-forgejo.svg",
	},
	github: {
		name: "GitHub",
		path: "/images/logo-github.svg",
	},
	kofi: {
		name: "Ko-fi",
		path: "/images/logo-kofi.svg",
	},
	liberapay: {
		name: "Liberapay",
		path: "/images/logo-liberapay.svg",
	},
	matrix: {
		name: "Matrix",
		path: "/images/logo-matrix.svg",
	},
	paypal: {
		name: "PayPal",
		path: "/images/logo-paypal.svg",
	},
	phone: {
		name: "Telephone",
		path: "/images/logo-telephone.svg",
	},
	signal: {
		name: "Signal",
		path: "/images/logo-signal.svg",
	},
} as const;

/**
 * The name of a supported offline platform.
 */
export type PlatformName = keyof typeof logos;
