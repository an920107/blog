import type { User } from '$lib/auth/domain/entity/user';

export interface AuthRepository {
	getCurrentUser(): Promise<User | null>;
}
