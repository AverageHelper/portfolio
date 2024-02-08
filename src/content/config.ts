import { z, defineCollection } from "astro:content";

/** Mathes `YYYY-MM-DD` strings. */
const dateString = /(\d{4})-(\d{2})-(\d{2})/gu;

// Define collection schemas here:
const ways = defineCollection({
	type: "content",
	schema: z.object({
		title: z.string(),
		description: z.string(),
		date: z.string().regex(dateString),
	}),
});

// Astro looks for this named import:
export const collections = {
	ways,
};
