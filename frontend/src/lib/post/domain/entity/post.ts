import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class Post {
	readonly id: number;
	readonly info: PostInfo;
	readonly content: string;

	constructor(props: { id: number; info: PostInfo; content: string }) {
		this.id = props.id;
		this.info = props.info;
		this.content = props.content;
	}
}
