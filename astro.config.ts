/* eslint-disable import/no-default-export */

import { defineConfig } from "astro/config";
import rehypeExternalLinks from "rehype-external-links";

// https://astro.build/config
export default defineConfig({
	site: "https://average.name", // Full URL
	root: ".", // Project root is the working directory
	outDir: "./dist", // Build to `/dist`
	publicDir: "./public", // Static assets live in `/public`
	srcDir: "./src", // Component sources live in `/src`
	output: "static", // No SSR, and no client JS
	trailingSlash: "never", // Paths should not contain a trailing slash
	compressHTML: false, // Let Prettier do its thing
	build: {
		format: "file", // Build HTML pages at root, not in subdirectories
		assets: "assets", // Call the build assets folder "assets" instead of "_astro"
	},
	devToolbar: {
		enabled: false, // Don't show dev controls in the webpage
	},
	markdown: {
		// Applies to .md and .mdx files
		rehypePlugins: [
			// Better anchor tags
			[rehypeExternalLinks, { target: "_blank", rel: ["noopener", "noreferrer"] }],
		],
	},
});
