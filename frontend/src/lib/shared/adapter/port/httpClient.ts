/**
 * A `fetch`-compatible HTTP transport. Implementations only deal with sending
 * requests; URL resolution, response parsing, and error mapping live in
 * `ApiClient`.
 */
export interface HttpClient {
	fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
}
