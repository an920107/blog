import type { PostRepository } from '$lib/post/application/port/postRepository';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class GetAllPostsUseCase {
	constructor(private readonly postRepository: PostRepository) {}

	execute(
		showUnpublished: boolean = false,
		labelId?: number,
		keyword?: string
	): Promise<PostInfo[]> {
		return this.postRepository.getAllPosts(showUnpublished, labelId, keyword);
	}
}
