import { assertEquals, assertObjectMatch } from "assert/mod.ts";
import { assertHeaders } from "./testUtils/assertHeaders.ts";
import { beforeAll, describe, it as test } from "testing/bdd.ts";
import { stub } from "testing/mock.ts";

describe("Webfinger", () => {
	let app: typeof import("./index.ts").app;

	beforeAll(async () => {
		// Don't run webserver when testing
		stub(Deno, "serve", () => ({}) as Deno.HttpServer);

		// Import unit under test (with mocks)
		app = (await import("./index.ts")).app;
	});

	testRedirect("/@average", "https://fosstodon.org/@avghelper", "temporary");
	testRedirect("/@avghelper", "https://fosstodon.org/@avghelper", "temporary");
	testRedirect("/@avg", "https://fosstodon.org/@avghelper", "temporary");

	test("fails without `resource` param", async () => {
		const res = await app.request("/.well-known/webfinger");
		assertEquals(res.status, 400);
		assertHeaders(res);
	});

	test("fails if `resource` param is empty", async () => {
		const res = await app.request("/.well-known/webfinger?resource");
		assertEquals(res.status, 400);
		assertHeaders(res);
	});

	test("fails if `resource` param is not a URL", async () => {
		const res = await app.request("/.well-known/webfinger?resource=foo");
		assertEquals(res.status, 400);
		assertHeaders(res);
	});

	test("fails if `resource` param is only protocol", async () => {
		const res = await app.request("/.well-known/webfinger?resource=acct:");
		assertEquals(res.status, 400);
		assertHeaders(res);
	});

	test("fails if `resource` param protocol is not 'acct:'", async () => {
		const res = await app.request("/.well-known/webfinger?resource=https:foo.bar");
		assertEquals(res.status, 404);
		assertHeaders(res);
	});

	test("fails if `resource` URL param host is not known", async () => {
		const res = await app.request("/.well-known/webfinger?resource=acct:foo.bar");
		assertEquals(res.status, 404);
		assertHeaders(res);
	});

	test("fails if `resource` account param host is not known", async () => {
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

	test("succeeds with resource 'acct:average.name'", async () => {
		const res = await app.request("/.well-known/webfinger?resource=acct:average.name");
		assertEquals(res.status, 200);
		assertHeaders(res);
		await assertBaseFinger(res);
	});

	test("succeeds with resource 'acct:average@average.name'", async () => {
		const res = await app.request("/.well-known/webfinger?resource=acct:average@average.name");
		assertEquals(res.status, 200);
		assertHeaders(res);
		await assertBaseFinger(res);
	});

	test("succeeds with resource 'acct:fosstodon.org'", async () => {
		const res = await app.request("/.well-known/webfinger?resource=acct:fosstodon.org");
		assertEquals(res.status, 200);
		assertHeaders(res);
		await assertBaseFinger(res);
	});

	test("succeeds with resource 'acct:avghelper@fosstodon.org'", async () => {
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
		test(`redirects '${from}' -> '${to}' (${type})`, async () => {
			const res = await app.request(from);
			assertEquals(res.status, redirectType[type]);
			assertEquals(res.headers.get("Location"), to);
			assertHeaders(res);
		});
	}
});
