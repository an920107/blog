import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import type { PostRepository } from '$lib/post/application/repository/postRepository';
import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class PostRepositoryImpl implements PostRepository {
	constructor(private readonly postApiService: PostApiService) {}

	async getAllPosts(): Promise<PostInfo[]> {
		const dtos = await this.postApiService.getAllPosts();
		return dtos.map((dto) => dto.toEntity());
	}

	async getPost(id: number): Promise<Post | null> {
		const dto = await this.postApiService.getPost(id);
		return dto?.toEntity() ?? null;
	}
}
