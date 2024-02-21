/* eslint-disable unicorn/filename-case, import/no-default-export */

declare module "@fec/remark-a11y-emoji" {
	import type { RemarkPlugin } from "@astrojs/markdown-remark";

	const a11yEmoji: RemarkPlugin;
	export default a11yEmoji;
}

// Canon from package, but doesn't work with Astro's types for some reason:
// declare module "@fec/remark-a11y-emoji" {
// 	import type { Nodes } from "mdast";

// 	export default function a11yEmoji(): (markdownAST: Nodes) => Nodes;
// }
