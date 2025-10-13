import { UserViewModel, type DehydratedUserProps } from '$lib/auth/adapter/presenter/userViewModel';

export class AuthViewModel {
	readonly user: UserViewModel | null;

	constructor(params: { user: UserViewModel | null }) {
		this.user = params.user;
	}

	static fromEntity(user: UserViewModel | null): AuthViewModel {
		return new AuthViewModel({ user });
	}

	static rehydrate(props: DehydratedAuthProps): AuthViewModel {
		return new AuthViewModel({
			user: props.user ? UserViewModel.rehydrate(props.user) : null,
		});
	}

	dehydrate(): DehydratedAuthProps {
		return {
			user: this.user ? this.user.dehydrate() : null,
		};
	}

	get isAuthenticated(): boolean {
		return this.user !== null;
	}
}

export interface DehydratedAuthProps {
	user: DehydratedUserProps | null;
}
