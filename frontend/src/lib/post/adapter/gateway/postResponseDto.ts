import {
	PostInfoResponseDto,
	postInfoResponseSchema,
} from '$lib/post/adapter/gateway/postInfoResponseDto';
import { Post } from '$lib/post/domain/entity/post';
import z from 'zod';

export const postResponseSchema = z.object({
	id: z.int32(),
	info: postInfoResponseSchema,
	content: z.string(),
});

export class PostResponseDto {
	readonly id: number;
	readonly info: PostInfoResponseDto;
	readonly content: string;

	private constructor(props: { id: number; info: PostInfoResponseDto; content: string }) {
		this.id = props.id;
		this.info = props.info;
		this.content = props.content;
	}

	static fromJson(json: unknown): PostResponseDto {
		const parsedJson = postResponseSchema.parse(json);
		return new PostResponseDto({
			id: parsedJson.id,
			info: PostInfoResponseDto.fromJson(parsedJson.info),
			content: parsedJson.content,
		});
	}

	toEntity(): Post {
		return new Post({
			id: this.id,
			info: this.info.toEntity(),
			content: this.content,
		});
	}
}
