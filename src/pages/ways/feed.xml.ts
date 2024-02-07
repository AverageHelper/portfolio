import { getCollection } from "astro:content";
import MarkdownIt from "markdown-it";
import rss from "@astrojs/rss";
import sanitizeHtml from "sanitize-html";

const parser = new MarkdownIt();

// Astro looks for this function to generate the feed:
export async function GET(context: { site: URL }): Promise<Response> {
	const ways = await getCollection("ways");

	return rss({
		stylesheet: "/rss/styles.xsl", // From "public/rss/styles.xsl"
		title: "Average Helper | Ways",
		description: "Average Helper's Ways Folder",
		customData: "<language>en-us</language>",
		site: new URL("ways", context.site), // origin+"/ways/"
		items: ways.map(way => ({
			link: `/ways/${way.slug}/`,
			title: way.data.title,
			pubDate: way.data.date,
			description: way.data.description,
			content: sanitizeHtml(parser.render(way.body)),
		})),
	});
}
