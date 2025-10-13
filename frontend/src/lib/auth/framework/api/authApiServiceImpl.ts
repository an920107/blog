import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
import { UserResponseDto } from '$lib/auth/adapter/gateway/userResponseDto';
import { Environment } from '$lib/environment';

export class AuthApiServiceImpl implements AuthApiService {
	constructor(private readonly fetchFn: typeof fetch) {}

	async getCurrentUser(): Promise<UserResponseDto | null> {
		const url = new URL('me', Environment.API_BASE_URL);

		const response = await this.fetchFn(url);

		if (!response.ok) {
			return null;
		}

		const json = await response.json();
		return UserResponseDto.fromJson(json);
	}
}
