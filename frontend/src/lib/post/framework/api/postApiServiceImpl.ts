import { Environment } from '$lib/environment';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostInfoResponseDto } from '$lib/post/adapter/gateway/postInfoResponseDto';

export class PostApiServiceImpl implements PostApiService {
	async getAllPosts(): Promise<PostInfoResponseDto[]> {
		const url = new URL('post/all', Environment.API_BASE_URL);

		const response = await fetch(url.href);

		if (!response.ok) {
			return [];
		}

		const json = await response.json();
		return json.map(PostInfoResponseDto.fromJson);
	}
}
