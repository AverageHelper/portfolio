import { app } from "./index.ts";
import { assertEquals, assertObjectMatch } from "assert/mod.ts";
import { assertHeaders } from "./testUtils/assertHeaders.ts";

testRedirect("/@average", "https://fosstodon.org/@avghelper", "temporary");
testRedirect("/@avghelper", "https://fosstodon.org/@avghelper", "temporary");
testRedirect("/@avg", "https://fosstodon.org/@avghelper", "temporary");

Deno.test("Webfinger fails without `resource` param", async () => {
	const res = await app.request("/.well-known/webfinger");
	assertEquals(res.status, 400);
	assertHeaders(res);
});

Deno.test("Webfinger fails if `resource` param is empty", async () => {
	const res = await app.request("/.well-known/webfinger?resource");
	assertEquals(res.status, 400);
	assertHeaders(res);
});

Deno.test("Webfinger fails if `resource` param is not a URL", async () => {
	const res = await app.request("/.well-known/webfinger?resource=foo");
	assertEquals(res.status, 400);
	assertHeaders(res);
});

Deno.test("Webfinger fails if `resource` param protocol is not 'acct:'", async () => {
	const res = await app.request("/.well-known/webfinger?resource=https:foo.bar");
	assertEquals(res.status, 400);
	assertHeaders(res);
});

Deno.test("Webfinger fails if `resource` URL param host is not known", async () => {
	const res = await app.request("/.well-known/webfinger?resource=acct:foo.bar");
	assertEquals(res.status, 404);
	assertHeaders(res);
});

Deno.test("Webfinger fails if `resource` account param host is not known", async () => {
	const res = await app.request("/.well-known/webfinger?resource=acct:foo@foo.bar");
	assertEquals(res.status, 404);
	assertHeaders(res);
});

async function assertBaseFinger(res: Response): Promise<void> {
	const data = await res.json();
	assertObjectMatch(data, {
		subject: "acct:avghelper@fosstodon.org",
		aliases: [
			"https://average.name/@average",
			"https://average.name/@avg",
			"https://average.name/@avghelper",
			"https://fosstodon.org/@avghelper",
			"https://fosstodon.org/users/avghelper",
		],
	});
}

Deno.test("Webfinger succeeds with resource 'acct:average.name'", async () => {
	const res = await app.request("/.well-known/webfinger?resource=acct:average.name");
	assertEquals(res.status, 200);
	assertHeaders(res);
	await assertBaseFinger(res);
});

Deno.test("Webfinger succeeds with resource 'acct:average@average.name'", async () => {
	const res = await app.request("/.well-known/webfinger?resource=acct:average@average.name");
	assertEquals(res.status, 200);
	assertHeaders(res);
	await assertBaseFinger(res);
});

Deno.test("Webfinger succeeds with resource 'acct:fosstodon.org'", async () => {
	const res = await app.request("/.well-known/webfinger?resource=acct:fosstodon.org");
	assertEquals(res.status, 200);
	assertHeaders(res);
	await assertBaseFinger(res);
});

Deno.test("Webfinger succeeds with resource 'acct:avghelper@fosstodon.org'", async () => {
	const res = await app.request("/.well-known/webfinger?resource=acct:avghelper@fosstodon.org");
	assertEquals(res.status, 200);
	assertHeaders(res);
	await assertBaseFinger(res);
});

// TODO: Test `rel` query and `"links"` result

const redirectType = {
	permanent: 301,
	temporary: 302,
} as const;

type RedirectType = keyof typeof redirectType;

function testRedirect(from: string, to: string, type: RedirectType): void {
	Deno.test(`Redirect '${from}' -> '${to}' (${type})`, async () => {
		const res = await app.request(from);
		assertEquals(res.status, redirectType[type]);
		assertEquals(res.headers.get("Location"), to);
		assertHeaders(res);
	});
}
