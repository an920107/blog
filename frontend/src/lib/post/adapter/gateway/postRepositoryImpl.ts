import { CreatePostRequestDto } from '$lib/post/adapter/gateway/creatPostRequestDto';
import { UpdatePostRequestDto } from '$lib/post/adapter/gateway/updatePostRequestDto';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostListQueryDto } from '$lib/post/adapter/gateway/postListQueryDto';
import type {
	CreatePostParams,
	UpdatePostParams,
	PostRepository,
} from '$lib/post/application/gateway/postRepository';
import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class PostRepositoryImpl implements PostRepository {
	constructor(private readonly postApiService: PostApiService) {}

	async getAllPosts(showUnpublished: boolean, labelId?: number): Promise<PostInfo[]> {
		const queryDto = new PostListQueryDto({ showUnpublished, labelId });
		const responseDtos = await this.postApiService.getAllPosts(queryDto);
		return responseDtos.map((dto) => dto.toEntity());
	}

	async getPost(id: string | number): Promise<Post | null> {
		const dto = await this.postApiService.getPost(id);
		return dto?.toEntity() ?? null;
	}

	async createPost(params: CreatePostParams): Promise<Post> {
		const requestDto = CreatePostRequestDto.fromParams(params);
		const responseDto = await this.postApiService.createPost(requestDto);
		return responseDto.toEntity();
	}

	async updatePost(id: number, params: UpdatePostParams): Promise<Post> {
		const requestDto = UpdatePostRequestDto.fromParams(params);
		const responseDto = await this.postApiService.updatePost(id, requestDto);
		return responseDto.toEntity();
	}
}
