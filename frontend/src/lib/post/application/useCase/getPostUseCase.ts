import type { PostRepository } from '$lib/post/application/gateway/postRepository';
import type { Post } from '$lib/post/domain/entity/post';

export class GetPostUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	execute(id: string | number): Promise<Post | null> {
		return this.postRepository.getPost(id);
	}
}
