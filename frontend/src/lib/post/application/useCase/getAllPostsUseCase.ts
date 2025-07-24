import type { PostRepository } from '$lib/post/application/repository/postRepository';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class GetAllPostsUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	execute(): Promise<PostInfo[]> {
		return this.postRepository.getAllPosts();
	}
}
