import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export interface PostRepository {
	getAllPosts(): Promise<PostInfo[]>;
	getPost(id: string): Promise<Post | null>;
}
