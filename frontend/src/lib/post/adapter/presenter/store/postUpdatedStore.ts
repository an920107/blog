import { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
import type { UpdatePostParams } from '$lib/post/application/port/postRepository';
import type { UpdatePostUseCase } from '$lib/post/application/useCase/updatePostUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

interface PostUpdatedTriggerParams {
	id: number;
	params: UpdatePostParams;
}

export class PostUpdatedStore extends BaseActionStore<PostViewModel, PostUpdatedTriggerParams> {
	static readonly name = 'PostUpdatedStore';

	constructor(private readonly updatePostUseCase: UpdatePostUseCase) {
		super();
	}

	protected async execute(params: PostUpdatedTriggerParams): Promise<PostViewModel> {
		const post = await this.updatePostUseCase.execute(params.id, params.params);

		return PostViewModel.fromEntity(post);
	}
}
