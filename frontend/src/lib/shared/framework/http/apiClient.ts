import type { HttpClient } from '$lib/shared/adapter/port/httpClient';
import { ApiError } from '$lib/shared/framework/http/apiError';

type JsonParser<T> = (json: unknown) => T;

interface RequestOptions {
	method: string;
	json?: unknown;
	form?: FormData;
}

/**
 * HTTP client for the backend API. It owns URL resolution, JSON handling, and
 * error mapping; feature `*ApiService` implementations only choose endpoints
 * and parse their DTOs.
 */
export class ApiClient {
	constructor(
		private readonly httpClient: HttpClient,
		private readonly baseUrl: string
	) {}

	url(path: string): URL {
		return new URL(path.replace(/^\/+/, ''), this.baseUrl);
	}

	getJson<T>(path: string, parse: JsonParser<T>): Promise<T> {
		return this.requestJson(path, { method: 'GET' }, parse);
	}

	getJsonArray<T>(path: string, parseItem: JsonParser<T>): Promise<T[]> {
		return this.getJson(path, (json) => {
			if (!Array.isArray(json)) {
				throw new Error(`Expected an array response from ${path}`);
			}
			return json.map(parseItem);
		});
	}

	async getJsonOrNull<T>(
		path: string,
		parse: JsonParser<T>,
		nullStatuses: readonly number[] = []
	): Promise<T | null> {
		try {
			return await this.getJson(path, parse);
		} catch (error) {
			if (error instanceof ApiError && nullStatuses.includes(error.status)) {
				return null;
			}
			throw error;
		}
	}

	postJson<T>(path: string, body: unknown, parse: JsonParser<T>): Promise<T> {
		return this.requestJson(path, { method: 'POST', json: body }, parse);
	}

	putJson<T>(path: string, body: unknown, parse: JsonParser<T>): Promise<T> {
		return this.requestJson(path, { method: 'PUT', json: body }, parse);
	}

	postForm<T>(path: string, form: FormData, parse: JsonParser<T>): Promise<T> {
		return this.requestJson(path, { method: 'POST', form }, parse);
	}

	async delete(path: string): Promise<void> {
		await this.request(path, { method: 'DELETE' });
	}

	private async requestJson<T>(
		path: string,
		options: RequestOptions,
		parse: JsonParser<T>
	): Promise<T> {
		const response = await this.request(path, options);
		return this.parseJson(response, parse);
	}

	private async request(path: string, options: RequestOptions): Promise<Response> {
		const init: RequestInit = { method: options.method };

		if (options.json !== undefined) {
			init.headers = { 'Content-Type': 'application/json' };
			init.body = JSON.stringify(options.json);
		} else if (options.form !== undefined) {
			init.body = options.form;
		}

		const response = await this.httpClient.fetch(this.url(path), init);
		if (!response.ok) {
			throw await ApiError.fromResponse(response);
		}

		return response;
	}

	private async parseJson<T>(response: Response, parse: JsonParser<T>): Promise<T> {
		const text = await response.text();
		if (text.length === 0) {
			throw new Error(`Expected a JSON response from ${response.url}`);
		}

		return parse(JSON.parse(text));
	}
}
