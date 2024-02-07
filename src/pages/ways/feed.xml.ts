import { getCollection } from "astro:content";
// import { unified } from "unified"; // from "astro"
// import rehypeExternalLinks from "rehype-external-links";
// import remarkParse from "remark-parse"; // from "astro"
// import remarkRehype from "remark-rehype"; // from "astro"
// import rehypeStringify from "rehype-stringify"; // from "astro"
import rss from "@astrojs/rss";
import MarkdownIt from "markdown-it";
import sanitizeHtml from "sanitize-html";

const parser = new MarkdownIt();

// Astro looks for this function to generate the feed:
export async function GET(context: { site: URL }): Promise<Response> {
	const ways = await getCollection("ways");

	// const parser = unified() //
	// 	.use(remarkParse)
	// 	.use(remarkRehype)
	// 	.use(rehypeExternalLinks, { target: "_blank", rel: ["noopener", "noreferrer"] }) // should match Astro config
	// 	.use(rehypeStringify);

	return rss({
		// stylesheet: "/rss/styles.xsl", // From "public/rss/styles.xsl"
		title: "Average Helper | Ways",
		description: "Average Helper's Ways Folder",
		customData: "<language>en-us</language>",
		site: new URL("ways", context.site), // origin+"/ways/"
		items: ways.map(way => ({
			link: `/ways/${way.slug}/`,
			title: way.data.title,
			pubDate: way.data.date,
			description: way.data.description,
			// content: sanitizeHtml(parser.processSync(way.body).toString()),
			content: sanitizeHtml(parser.render(way.body)),
		})),
	});
}
