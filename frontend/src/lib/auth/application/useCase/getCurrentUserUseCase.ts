import type { AuthRepository } from '$lib/auth/application/gateway/authRepository';
import type { User } from '$lib/auth/domain/entity/user';

export class GetCurrentUserUseCase {
	constructor(private readonly authRepository: AuthRepository) {}

	async execute(): Promise<User | null> {
		return await this.authRepository.getCurrentUser();
	}
}
