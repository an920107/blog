import type { CreatePostRequestDto } from '$lib/post/adapter/gateway/dto/createPostRequestDto';
import { PostInfoResponseDto } from '$lib/post/adapter/gateway/dto/postInfoResponseDto';
import type { PostListQueryDto } from '$lib/post/adapter/gateway/dto/postListQueryDto';
import { PostResponseDto } from '$lib/post/adapter/gateway/dto/postResponseDto';
import type { UpdatePostRequestDto } from '$lib/post/adapter/gateway/dto/updatePostRequestDto';
import type { PostApiService } from '$lib/post/adapter/port/postApiService';
import { ApiClient } from '$lib/shared/framework/http/apiClient';
import { HttpStatusCode } from '$lib/shared/framework/http/httpStatusCode';

export class PostApiServiceImpl implements PostApiService {
	constructor(private readonly apiClient: ApiClient) {}

	getAllPosts(searchParams: PostListQueryDto): Promise<PostInfoResponseDto[]> {
		const query = searchParams.toSearchParams().toString();

		return this.apiClient.getJsonArray(
			query.length > 0 ? `post?${query}` : 'post',
			PostInfoResponseDto.fromJson
		);
	}

	getPost(id: string | number): Promise<PostResponseDto | null> {
		return this.apiClient.getJsonOrNull(`post/${id}`, PostResponseDto.fromJson, [
			HttpStatusCode.NOT_FOUND,
		]);
	}

	createPost(payload: CreatePostRequestDto): Promise<PostResponseDto> {
		return this.apiClient.postJson('post', payload.toJson(), PostResponseDto.fromJson);
	}

	updatePost(id: number, payload: UpdatePostRequestDto): Promise<PostResponseDto> {
		return this.apiClient.putJson(`post/${id}`, payload.toJson(), PostResponseDto.fromJson);
	}
}
