import { AuthViewModel } from '$lib/auth/adapter/presenter/authViewModel';
import { UserViewModel } from '$lib/auth/adapter/presenter/userViewModel';
import type { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
import { StatusType, type AsyncState } from '$lib/common/adapter/presenter/asyncState';
import { get, writable } from 'svelte/store';

export type AuthState = AsyncState<AuthViewModel>;
export type AuthEvent = CurrentUserLoadedEvent;

export class AuthBloc {
	private readonly state = writable<AuthState>({
		status: StatusType.Idle,
	});

	constructor(
		private readonly getCurrentUserUseCase: GetCurrentUserUseCase,
		initialData?: AuthViewModel
	) {
		this.state.set({
			status: StatusType.Idle,
			data: initialData,
		});
	}

	get subscribe() {
		return this.state.subscribe;
	}

	async dispatch(event: AuthEvent): Promise<AuthState> {
		switch (event.event) {
			case AuthEventType.CurrentUserLoadedEvent:
				return this.loadCurrentUser();
		}
	}

	private async loadCurrentUser(): Promise<AuthState> {
		this.state.set({ status: StatusType.Loading, data: get(this.state).data });

		const user = await this.getCurrentUserUseCase.execute();

		const userViewModel = user ? UserViewModel.fromEntity(user) : null;
		const authViewModel = AuthViewModel.fromEntity(userViewModel);
		const result: AuthState = {
			status: StatusType.Success,
			data: authViewModel,
		};

		this.state.set(result);
		return result;
	}
}

export enum AuthEventType {
	CurrentUserLoadedEvent,
}

interface CurrentUserLoadedEvent {
	event: AuthEventType.CurrentUserLoadedEvent;
}
