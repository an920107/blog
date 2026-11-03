import { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
import type { CreatePostDraftParams } from '$lib/post/application/port/postRepository';
import type { CreatePostUseCase } from '$lib/post/application/useCase/createPostUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class PostCreatedStore extends BaseActionStore<PostViewModel, CreatePostDraftParams> {
	constructor(private readonly createPostUseCase: CreatePostUseCase) {
		super();
	}

	protected async execute(params: CreatePostDraftParams): Promise<PostViewModel> {
		const post = await this.createPostUseCase.execute(params);

		return PostViewModel.fromEntity(post);
	}
}
