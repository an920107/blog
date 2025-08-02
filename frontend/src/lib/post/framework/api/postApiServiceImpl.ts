import { Environment } from '$lib/environment';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostInfoResponseDto } from '$lib/post/adapter/gateway/postInfoResponseDto';
import { PostResponseDto } from '$lib/post/adapter/gateway/postResponseDto';

export class PostApiServiceImpl implements PostApiService {
	constructor(private fetchFn: typeof fetch) {}

	async getAllPosts(): Promise<PostInfoResponseDto[]> {
		const url = new URL('post', Environment.API_BASE_URL);

		const response = await this.fetchFn(url.href);

		if (!response.ok) {
			return [];
		}

		const json = await response.json();
		return json.map(PostInfoResponseDto.fromJson);
	}

	async getPost(id: number): Promise<PostResponseDto | null> {
		const url = new URL(`post/${id}`, Environment.API_BASE_URL);

		const response = await this.fetchFn(url.href);

		if (!response.ok) {
			return null;
		}

		const json = await response.json();
		return PostResponseDto.fromJson(json);
	}
}
