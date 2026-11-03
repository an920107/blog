import { UserResponseDto } from '$lib/auth/adapter/gateway/dto/userResponseDto';
import type { AuthApiService } from '$lib/auth/adapter/port/authApiService';
import { ApiClient } from '$lib/shared/framework/http/apiClient';
import { HttpStatusCode } from '$lib/shared/framework/http/httpStatusCode';

export class AuthApiServiceImpl implements AuthApiService {
	constructor(private readonly apiClient: ApiClient) {}

	getCurrentUser(): Promise<UserResponseDto | null> {
		return this.apiClient.getJsonOrNull('me', UserResponseDto.fromJson, [
			HttpStatusCode.UNAUTHORIZED,
		]);
	}
}
