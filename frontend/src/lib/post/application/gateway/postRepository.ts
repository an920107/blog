import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export interface PostRepository {
	getAllPosts(showUnpublished: boolean, labelId?: number): Promise<PostInfo[]>;
	getPost(id: string): Promise<Post | null>;
	createPost(params: CreatePostParams): Promise<Post>;
}

export interface CreatePostParams {
	semanticId: string;
	title: string;
}
