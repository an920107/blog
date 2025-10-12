import {
	PostInfoViewModel,
	type DehydratedPostInfoProps,
} from '$lib/post/adapter/presenter/postInfoViewModel';
import type { Post } from '$lib/post/domain/entity/post';

export class PostViewModel {
	id: number;
	info: PostInfoViewModel;
	content: string;

	private constructor(props: { id: number; info: PostInfoViewModel; content: string }) {
		this.id = props.id;
		this.info = props.info;
		this.content = props.content;
	}

	static fromEntity(post: Post): PostViewModel {
		return new PostViewModel({
			id: post.id,
			info: PostInfoViewModel.fromEntity(post.info),
			content: post.content,
		});
	}

	static rehydrate(props: DehydratedPostProps): PostViewModel {
		return new PostViewModel({
			id: props.id,
			info: PostInfoViewModel.rehydrate(props.info),
			content: props.content,
		});
	}

	dehydrate(): DehydratedPostProps {
		return {
			id: this.id,
			info: this.info.dehydrate(),
			content: this.content,
		};
	}
}

export interface DehydratedPostProps {
	id: number;
	info: DehydratedPostInfoProps;
	content: string;
}
