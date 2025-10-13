import type { UserResponseDto } from '$lib/auth/adapter/gateway/userResponseDto';

export interface AuthApiService {
	getCurrentUser(): Promise<UserResponseDto | null>;
}
