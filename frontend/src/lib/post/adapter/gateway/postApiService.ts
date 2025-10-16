import type { CreatePostRequestDto } from '$lib/post/adapter/gateway/creatPostRequestDto';
import type { UpdatePostRequestDto } from '$lib/post/adapter/gateway/updatePostRequestDto';
import type { PostInfoResponseDto } from '$lib/post/adapter/gateway/postInfoResponseDto';
import type { PostListQueryDto } from '$lib/post/adapter/gateway/postListQueryDto';
import type { PostResponseDto } from '$lib/post/adapter/gateway/postResponseDto';

export interface PostApiService {
	getAllPosts(searchParams: PostListQueryDto): Promise<PostInfoResponseDto[]>;
	getPost(id: string | number): Promise<PostResponseDto | null>;
	createPost(payload: CreatePostRequestDto): Promise<PostResponseDto>;
	updatePost(id: number, payload: UpdatePostRequestDto): Promise<PostResponseDto>;
}
