export class HttpError extends Error {
	constructor(
		public readonly status: number,
		url: URL
	) {
		super(`HTTP ${status} at ${url.href}`);
	}
}
