import { AuthViewModel } from '$lib/auth/adapter/presenter/viewModel/authViewModel';
import { UserViewModel } from '$lib/auth/adapter/presenter/viewModel/userViewModel';
import type { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class AuthLoadedStore extends BaseActionStore<AuthViewModel> {
	constructor(private readonly getCurrentUserUseCase: GetCurrentUserUseCase) {
		super();
	}

	protected async execute(): Promise<AuthViewModel> {
		const user = await this.getCurrentUserUseCase.execute();
		const userViewModel = user ? UserViewModel.fromEntity(user) : null;

		return AuthViewModel.fromEntity(userViewModel);
	}
}
