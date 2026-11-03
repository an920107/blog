import type {
	CreatePostDraftParams,
	PostRepository,
} from '$lib/post/application/port/postRepository';
import type { Post } from '$lib/post/domain/entity/post';

export class CreatePostUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	async execute(params: CreatePostDraftParams): Promise<Post> {
		return this.postRepository.createPostDraft(params);
	}
}
