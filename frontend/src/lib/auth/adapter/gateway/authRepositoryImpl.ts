import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
import type { AuthRepository } from '$lib/auth/application/gateway/authRepository';
import type { User } from '$lib/auth/domain/entity/user';

export class AuthRepositoryImpl implements AuthRepository {
	constructor(private readonly authApiService: AuthApiService) {}

	async getCurrentUser(): Promise<User | null> {
		const result = await this.authApiService.getCurrentUser();
		return result?.toEntity() ?? null;
	}
}
