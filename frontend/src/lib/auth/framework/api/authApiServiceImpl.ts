import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
import { UserResponseDto } from '$lib/auth/adapter/gateway/userResponseDto';
import { HttpError } from '$lib/common/framework/web/httpError';
import { HttpStatusCode } from '$lib/common/framework/web/httpStatusCode';
import { Environment } from '$lib/environment';

export class AuthApiServiceImpl implements AuthApiService {
	constructor(private readonly fetchFn: typeof fetch) {}

	async getCurrentUser(): Promise<UserResponseDto | null> {
		const url = new URL('me', Environment.API_BASE_URL);

		const response = await this.fetchFn(url);

		if (response.status === HttpStatusCode.UNAUTHORIZED) {
			return null;
		}

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const json = await response.json();
		return UserResponseDto.fromJson(json);
	}
}
