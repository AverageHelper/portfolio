import { z, defineCollection } from "astro:content";

/**
 * Returns a {@link Date} that follows the given date by one day.
 */
function oneDayAfter(date: Date): Date {
	const result = new Date(date);
	result.setDate(date.getDate() + 1);
	return result;
}

// Define collection schemas here:
const ways = defineCollection({
	type: "content",
	schema: z.object({
		title: z.string(),
		description: z.string(),
		date: z.coerce.date().transform(oneDayAfter), // `Date()` assumes UTC
	}),
});

// Astro looks for this named import:
export const collections = {
	ways,
};
