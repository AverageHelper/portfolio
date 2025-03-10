import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

/** Mathes `YYYY-MM-DD` strings. */
const dateString = /(\d{4})-(\d{2})-(\d{2})/gu;

// Define collection schemas here:
const ways = defineCollection({
	loader: glob({ pattern: "**/[^_]*.{md,mdx}", base: "./src/content/ways" }),
	schema: z.object({
		/** A descriptive title of the document's content. */
		title: z.string(),

		/** A text description of the document's content, usually the first sentence or two of the contents. */
		description: z.string(),

		/** The `YYYY-MM-DD` when this document should show as published. */
		date: z.string().regex(dateString),
	}),
});

// Astro looks for this named import:
export const collections = {
	ways,
};
