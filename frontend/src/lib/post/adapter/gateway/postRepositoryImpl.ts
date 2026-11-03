import { CreatePostRequestDto } from '$lib/post/adapter/gateway/dto/createPostRequestDto';
import { PostListQueryDto } from '$lib/post/adapter/gateway/dto/postListQueryDto';
import { UpdatePostRequestDto } from '$lib/post/adapter/gateway/dto/updatePostRequestDto';
import type { PostApiService } from '$lib/post/adapter/port/postApiService';
import type {
	CreatePostDraftParams,
	PostRepository,
	UpdatePostParams,
} from '$lib/post/application/port/postRepository';
import type { Post } from '$lib/post/domain/entity/post';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class PostRepositoryImpl implements PostRepository {
	constructor(private readonly postApiService: PostApiService) {}

	async getAllPosts(
		showUnpublished: boolean,
		labelId?: number,
		keyword?: string
	): Promise<PostInfo[]> {
		const queryDto = new PostListQueryDto({ showUnpublished, labelId, keyword });
		const responseDtos = await this.postApiService.getAllPosts(queryDto);
		return responseDtos.map((dto) => dto.toEntity());
	}

	async getPost(id: string | number): Promise<Post | null> {
		const dto = await this.postApiService.getPost(id);
		return dto?.toEntity() ?? null;
	}

	async createPostDraft(params: CreatePostDraftParams): Promise<Post> {
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
