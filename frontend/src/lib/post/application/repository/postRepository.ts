import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export interface PostRepository {
	getAllPosts(): Promise<PostInfo[]>;
}
