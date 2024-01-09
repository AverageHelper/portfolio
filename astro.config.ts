/* eslint-disable import/no-default-export */

import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
	site: "https://average.name", // Full URL
	root: ".", // Project root is the working directory
	outDir: "./pages", // Build to where Cloudflare Pages expects
	publicDir: "./public", // Static assets live in `/public`
	srcDir: "./src", // Component sources live in `/src`
	output: "static", // No SSR, and no client JS
	trailingSlash: "never", // Paths should not contain a trailing slash
	compressHTML: false, // Let Prettier do its thing
	build: {
		format: "file", // Build pages at root, not in subdirectories
	},
});
