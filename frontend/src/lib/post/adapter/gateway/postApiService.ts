import type { PostInfoResponseDto } from '$lib/post/adapter/gateway/postInfoResponseDto';

export interface PostApiService {
	getAllPosts(): Promise<PostInfoResponseDto[]>;
}
