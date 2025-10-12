import type { PostInfoResponseDto } from '$lib/post/adapter/gateway/postInfoResponseDto';
import type { PostResponseDto } from '$lib/post/adapter/gateway/postResponseDto';

export interface PostApiService {
	getAllPosts(): Promise<PostInfoResponseDto[]>;
	getPost(id: string): Promise<PostResponseDto | null>;
}
