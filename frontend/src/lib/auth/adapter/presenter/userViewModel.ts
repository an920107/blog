import type { User } from '$lib/auth/domain/entity/user';

export class UserViewModel {
	readonly id: number;
	readonly name: string;
	readonly email: string;

	private constructor(props: { id: number; name: string; email: string }) {
		this.id = props.id;
		this.name = props.name;
		this.email = props.email;
	}

	static fromEntity(user: User): UserViewModel {
		return new UserViewModel({
			id: user.id,
			name: user.name,
			email: user.email,
		});
	}

	static rehydrate(props: DehydratedUserProps): UserViewModel {
		return new UserViewModel({
			id: props.id,
			name: props.name,
			email: props.email,
		});
	}

	dehydrate(): DehydratedUserProps {
		return {
			id: this.id,
			name: this.name,
			email: this.email,
		};
	}
}

export interface DehydratedUserProps {
	id: number;
	name: string;
	email: string;
}
