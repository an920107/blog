import { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
import type { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

interface PostsListedTriggerParams {
	showUnpublished: boolean;
	labelId?: number;
	keyword?: string;
}

export class PostsListedStore extends BaseActionStore<
	readonly PostInfoViewModel[],
	PostsListedTriggerParams | undefined
> {
	constructor(
		private readonly getAllPostsUseCase: GetAllPostsUseCase,
		initialData?: readonly PostInfoViewModel[]
	) {
		super(initialData ?? []);
	}

	get trigger() {
		return (options?: PostsListedTriggerParams) => this.run(options);
	}

	protected async execute(
		options?: PostsListedTriggerParams
	): Promise<readonly PostInfoViewModel[]> {
		const posts = await this.getAllPostsUseCase.execute(
			options?.showUnpublished,
			options?.labelId,
			options?.keyword
		);

		return posts.map((post) => PostInfoViewModel.fromEntity(post));
	}
}
