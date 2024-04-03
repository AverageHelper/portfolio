export function notFound(): Response {
	return new Response("404 Not Found", { status: 404 });
}

export function badRequest(): Response {
	return new Response("400 Bad Request", { status: 400 });
}
