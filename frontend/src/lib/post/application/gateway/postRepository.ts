import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export interface PostRepository {
	getAllPosts(showUnpublished: boolean, labelId?: number): Promise<PostInfo[]>;
	getPost(id: string | number): Promise<Post | null>;
	createPost(params: CreatePostParams): Promise<Post>;
	updatePost(id: number, params: UpdatePostParams): Promise<Post>;
}

export interface CreatePostParams {
	semanticId: string;
	title: string;
}

export interface UpdatePostParams {
	title: string;
	description: string;
	content: string;
	labelIds: number[];
	previewImageUrl: URL | null;
	publishedTime: Date | null;
}
