import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
import type { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type PostListState = AsyncState<readonly PostInfoViewModel[]>;

export class PostsListedStore
	implements BaseStore<PostListState, { showUnpublished: boolean; labelId?: number }>
{
	private readonly state = writable<PostListState>(AsyncState.idle([]));

	constructor(
		private readonly getAllPostsUseCase: GetAllPostsUseCase,
		initialData?: readonly PostInfoViewModel[]
	) {
		if (initialData) {
			this.state.set(AsyncState.idle(initialData));
		}
	}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (options?: { showUnpublished: boolean; labelId?: number }) =>
			this.loadPosts(options?.showUnpublished, options?.labelId);
	}

	private async loadPosts(showUnpublished?: boolean, labelId?: number): Promise<PostListState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: PostListState;
		try {
			const posts = await this.getAllPostsUseCase.execute(showUnpublished, labelId);
			const postViewModels = posts.map((post) => PostInfoViewModel.fromEntity(post));
			result = AsyncState.success(postViewModels);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
