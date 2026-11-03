import type { CreatePostRequestDto } from '$lib/post/adapter/gateway/dto/createPostRequestDto';
import type { PostInfoResponseDto } from '$lib/post/adapter/gateway/dto/postInfoResponseDto';
import type { PostListQueryDto } from '$lib/post/adapter/gateway/dto/postListQueryDto';
import type { PostResponseDto } from '$lib/post/adapter/gateway/dto/postResponseDto';
import type { UpdatePostRequestDto } from '$lib/post/adapter/gateway/dto/updatePostRequestDto';

export interface PostApiService {
	getAllPosts(searchParams: PostListQueryDto): Promise<PostInfoResponseDto[]>;
	getPost(id: string | number): Promise<PostResponseDto | null>;
	createPost(payload: CreatePostRequestDto): Promise<PostResponseDto>;
	updatePost(id: number, payload: UpdatePostRequestDto): Promise<PostResponseDto>;
}
