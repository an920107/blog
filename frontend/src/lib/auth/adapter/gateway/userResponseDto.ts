import { User } from '$lib/auth/domain/entity/user';
import z from 'zod';

export const userResponseSchema = z.object({
	id: z.int32(),
	displayed_name: z.string(),
	email: z.email(),
});

export class UserResponseDto {
	readonly id: number;
	readonly displayedName: string;
	readonly email: string;

	private constructor(props: { id: number; displayedName: string; email: string }) {
		this.id = props.id;
		this.displayedName = props.displayedName;
		this.email = props.email;
	}

	static fromJson(json: unknown): UserResponseDto {
		const parsedJson = userResponseSchema.parse(json);
		return new UserResponseDto({
			id: parsedJson.id,
			displayedName: parsedJson.displayed_name,
			email: parsedJson.email,
		});
	}

	toEntity(): User {
		return new User({
			id: this.id,
			name: this.displayedName,
			email: this.email,
		});
	}
}
