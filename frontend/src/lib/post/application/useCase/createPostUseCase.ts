import type {
	CreatePostParams,
	PostRepository,
} from '$lib/post/application/gateway/postRepository';
import type { Post } from '$lib/post/domain/entity/post';

export class CreatePostUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	async execute(params: CreatePostParams): Promise<Post> {
		return this.postRepository.createPost(params);
	}
}
