import { AuthRepositoryImpl } from '$lib/auth/adapter/gateway/authRepositoryImpl';
import { AuthLoadedStore } from '$lib/auth/adapter/presenter/store/authLoadedStore';
import type { AuthRepository } from '$lib/auth/application/port/authRepository';
import { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
import { AuthApiServiceImpl } from '$lib/auth/framework/api/authApiServiceImpl';
import type { ApiClient } from '$lib/shared/framework/http/apiClient';

export function createAuthModule(apiClient: ApiClient) {
	const apiService = new AuthApiServiceImpl(apiClient);
	const repository: AuthRepository = new AuthRepositoryImpl(apiService);
	const getCurrentUserUseCase = new GetCurrentUserUseCase(repository);

	return {
		getCurrentUserUseCase,
		createAuthLoadedStore: () => new AuthLoadedStore(getCurrentUserUseCase),
	};
}

export type AuthModule = ReturnType<typeof createAuthModule>;
