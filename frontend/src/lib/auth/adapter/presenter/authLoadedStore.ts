import { AuthViewModel } from '$lib/auth/adapter/presenter/authViewModel';
import { UserViewModel } from '$lib/auth/adapter/presenter/userViewModel';
import type { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type AuthState = AsyncState<AuthViewModel>;

export class AuthLoadedStore implements BaseStore<AuthState> {
	private readonly state = writable<AuthState>(AsyncState.idle<AuthViewModel>(null));

	constructor(private readonly getCurrentUserUseCase: GetCurrentUserUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return () => this.loadCurrentUser();
	}

	private async loadCurrentUser(): Promise<AuthState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: AuthState;
		try {
			const user = await this.getCurrentUserUseCase.execute();
			const userViewModel = user ? UserViewModel.fromEntity(user) : null;
			const authViewModel = AuthViewModel.fromEntity(userViewModel);
			result = AsyncState.success(authViewModel);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
