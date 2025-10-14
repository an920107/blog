import { CreatePostRequestDto } from '$lib/post/adapter/gateway/creatPostRequestDto';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostListQueryDto } from '$lib/post/adapter/gateway/postListQueryDto';
import type {
	CreatePostParams,
	PostRepository,
} from '$lib/post/application/gateway/postRepository';
import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class PostRepositoryImpl implements PostRepository {
	constructor(private readonly postApiService: PostApiService) {}

	async getAllPosts(showUnpublished: boolean): Promise<PostInfo[]> {
		const queryDto = new PostListQueryDto({ showUnpublished });
		const responseDtos = await this.postApiService.getAllPosts(queryDto);
		return responseDtos.map((dto) => dto.toEntity());
	}

	async getPost(id: string): Promise<Post | null> {
		const dto = await this.postApiService.getPost(id);
		return dto?.toEntity() ?? null;
	}

	async createPost(params: CreatePostParams): Promise<Post> {
		const requestDto = CreatePostRequestDto.fromParams(params);
		const responseDto = await this.postApiService.createPost(requestDto);
		return responseDto.toEntity();
	}
}
