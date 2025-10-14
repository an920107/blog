import type { PostRepository } from '$lib/post/application/gateway/postRepository';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class GetAllPostsUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	execute(showUnpublished: boolean = false): Promise<PostInfo[]> {
		return this.postRepository.getAllPosts(showUnpublished);
	}
}
