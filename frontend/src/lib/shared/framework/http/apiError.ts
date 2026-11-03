export class ApiError extends Error {
	constructor(
		public readonly status: number,
		message: string,
		public readonly data?: unknown
	) {
		super(message);
		this.name = 'ApiError';
	}

	/**
	 * Builds an `ApiError` from a non-OK response. Backend error payloads are
	 * optional, so the status code is used as the fallback message.
	 */
	static async fromResponse(response: Response): Promise<ApiError> {
		let data: unknown;
		try {
			data = await response.json();
		} catch {
			return new ApiError(response.status, `HTTP ${response.status}`);
		}

		if (isRecord(data)) {
			if (typeof data.error === 'string') {
				return new ApiError(response.status, data.error, data);
			}
			if (typeof data.message === 'string') {
				return new ApiError(response.status, data.message, data);
			}
		}

		return new ApiError(response.status, `HTTP ${response.status}`, data);
	}
}

function isRecord(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null;
}
