import type { UserResponseDto } from '$lib/auth/adapter/gateway/dto/userResponseDto';

export interface AuthApiService {
	getCurrentUser(): Promise<UserResponseDto | null>;
}
