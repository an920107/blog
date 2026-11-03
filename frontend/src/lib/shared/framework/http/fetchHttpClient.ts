import type { HttpClient } from '$lib/shared/adapter/port/httpClient';

/**
 * `HttpClient` backed by any `fetch`-compatible function. On the server this
 * receives SvelteKit's `event.fetch`, which forwards cookies; in the browser it
 * receives the global `fetch`.
 */
export class FetchHttpClient implements HttpClient {
	constructor(
		private readonly fetchImpl: typeof fetch,
		private readonly credentials: RequestCredentials = 'same-origin'
	) {}

	fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response> {
		return this.fetchImpl(input, { credentials: this.credentials, ...init });
	}
}
