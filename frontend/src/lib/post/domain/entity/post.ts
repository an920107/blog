import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class Post {
	id: number;
	info: PostInfo;
	content: string;

	constructor(props: { id: number; info: PostInfo; content: string }) {
		this.id = props.id;
		this.info = props.info;
		this.content = props.content;
	}
}
