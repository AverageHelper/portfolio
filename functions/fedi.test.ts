import { app } from "./index.ts";
import { assertEquals } from "assert/mod.ts";

const redirectType = {
	permanent: 301,
	temporary: 302,
} as const;

type RedirectType = keyof typeof redirectType;

function testRedirect(from: string, to: string, type: RedirectType): void {
	Deno.test(`${from} -> ${to} (${type})`, async () => {
		const res = await app.request(from);
		assertEquals(res.status, redirectType[type]);
		assertEquals(res.headers.get("Location"), to);
	});
}

testRedirect("/@average", "https://fosstodon.org/@avghelper", "temporary");
testRedirect("/@avghelper", "https://fosstodon.org/@avghelper", "temporary");
testRedirect("/@avg", "https://fosstodon.org/@avghelper", "temporary");

Deno.test("Webfinger fails without `resource` param", async () => {
	const res = await app.request("/.well-known/webfinger");
	assertEquals(res.status, 400);
});

// TODO: More tests, and code coverage
