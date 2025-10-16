import type { Post } from '$lib/post/domain/entity/post';
import type {
	PostRepository,
	UpdatePostParams,
} from '$lib/post/application/gateway/postRepository';

export class UpdatePostUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	async execute(id: number, params: UpdatePostParams): Promise<Post> {
		return await this.postRepository.updatePost(id, params);
	}
}
