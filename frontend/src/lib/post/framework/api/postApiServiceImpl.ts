import { HttpError } from '$lib/common/framework/web/httpError';
import { HttpStatusCode } from '$lib/common/framework/web/httpStatusCode';
import { Environment } from '$lib/environment';
import type { CreatePostRequestDto } from '$lib/post/adapter/gateway/creatPostRequestDto';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostInfoResponseDto } from '$lib/post/adapter/gateway/postInfoResponseDto';
import type { PostListQueryDto } from '$lib/post/adapter/gateway/postListQueryDto';
import { PostResponseDto } from '$lib/post/adapter/gateway/postResponseDto';

export class PostApiServiceImpl implements PostApiService {
	constructor(private readonly fetchFn: typeof fetch) {}

	async getAllPosts(searchParams: PostListQueryDto): Promise<PostInfoResponseDto[]> {
		const url = new URL('post', Environment.API_BASE_URL);
		url.search = searchParams.toSearchParams().toString();

		const response = await this.fetchFn(url);

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return data.map(PostInfoResponseDto.fromJson);
	}

	async getPost(id: string): Promise<PostResponseDto | null> {
		const url = new URL(`post/${id}`, Environment.API_BASE_URL);

		const response = await this.fetchFn(url);

		if (response.status === HttpStatusCode.NOT_FOUND) {
			return null;
		}

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return PostResponseDto.fromJson(data);
	}

	async createPost(payload: CreatePostRequestDto): Promise<PostResponseDto> {
		const url = new URL('post', Environment.API_BASE_URL);

		const response = await this.fetchFn(url, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload.toJson()),
		});

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return PostResponseDto.fromJson(data);
	}
}
