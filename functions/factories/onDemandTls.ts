import { badRequest } from "../utils/responses.ts";
import { factory } from "./factory.ts";

const SUBDOMAINS = [
	// Subdomains that I want to give a *.avg.name alias:
	"blog",
	"dotfiles",
	"flashcards",
	"git",
	"ip",
	"ipv4",
	"jsonresume",
	"status",
	"www",
] as const;

const aliasDomains: ReadonlySet<string> = new Set(SUBDOMAINS.map(s => `${s}.avg.name`));

const AT_PROTO = [
	// Subdomains that I want to give an AT Protocol handle, i.e. @test.average.name
	"avgtest",
];

const atProtoDomains: ReadonlySet<string> = new Set(AT_PROTO.map(s => `${s}.average.name`));

/**
 * Answers HTTP 204 if the given `domain` is valid. HTTP 404 otherwise.
 */
export const onDemandTls = factory.createHandlers(c => {
	// See https://caddyserver.com/docs/automatic-https#on-demand-tls
	const domain = c.req.query("domain");
	if (!domain) return badRequest();

	if (domain === "avg.name") return new Response(null, { status: 204 });
	if (atProtoDomains.has(domain)) return new Response(null, { status: 204 });
	if (aliasDomains.has(domain)) return new Response(null, { status: 204 });

	return new Response(null, { status: 404 }); // not found
});
