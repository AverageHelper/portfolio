/* eslint-disable import/no-default-export */

import { defineConfig } from "astro/config";
import a11yEmoji from "@fec/remark-a11y-emoji";
import rehypeExternalLinks from "rehype-external-links";
import { rehypeGithubAlerts } from "rehype-github-alerts";
import sitemap from "@astrojs/sitemap";

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
	// Applies to .md and .mdx files
	markdown: {
		remarkPlugins: [
			// Wrap emoji characters in `span` with accessible labels
			a11yEmoji,
		],
		rehypePlugins: [
			// Better anchor tags
			[rehypeExternalLinks, { target: "_blank", rel: ["external", "noopener", "noreferrer"] }],
			// Emulate GitHub's fancy Blockquote Alerts
			rehypeGithubAlerts,
		],
		syntaxHighlight: "prism", // Use Prism instead of Shiki to render code blocks
	},
	integrations: [
		sitemap({
			changefreq: "weekly",
			lastmod: new Date(), // Last modified on [today]
		}),
	],
});
