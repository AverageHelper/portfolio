import { Hono } from "hono";

const app = new Hono() //
	.get("/", c => c.text("Hello, world!"));

// Cloudflare wants `export default` here:
// eslint-disable-next-line import/no-default-export
export default app;
