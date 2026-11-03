import { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
import type { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';
import { Strings } from '$lib/strings';

export class PostLoadedStore extends BaseActionStore<PostViewModel, string | number> {
	protected readonly notFoundMessage = Strings.POST_NOT_FOUND;

	constructor(
		private readonly getPostUseCase: GetPostUseCase,
		initialData?: PostViewModel
	) {
		super(initialData ?? null);
	}

	protected async execute(id: string | number): Promise<PostViewModel | null> {
		const post = await this.getPostUseCase.execute(id);

		return post ? PostViewModel.fromEntity(post) : null;
	}
}
