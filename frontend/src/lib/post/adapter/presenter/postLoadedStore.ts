import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
import type { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type PostState = AsyncState<PostViewModel>;

export class PostLoadedStore implements BaseStore<PostState, string> {
	private readonly state = writable<PostState>(AsyncState.idle<PostViewModel>(null));

	constructor(
		private readonly getPostUseCase: GetPostUseCase,
		initialData?: PostViewModel
	) {
		if (initialData) {
			this.state.set(AsyncState.idle(initialData));
		}
	}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (id: string) => this.loadPost(id);
	}

	private async loadPost(id: string): Promise<PostState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: PostState;
		try {
			const post = await this.getPostUseCase.execute(id);
			if (!post) {
				result = AsyncState.error(new Error('Post not found'), get(this.state).data);
				this.state.set(result);
				return result;
			}
			const postViewModel = PostViewModel.fromEntity(post);
			result = AsyncState.success(postViewModel);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
