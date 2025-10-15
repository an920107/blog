import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
import type { CreatePostParams } from '$lib/post/application/gateway/postRepository';
import type { CreatePostUseCase } from '$lib/post/application/useCase/createPostUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type PostState = AsyncState<PostViewModel>;

export class PostCreatedStore implements BaseStore<PostState, CreatePostParams> {
	private readonly state = writable<PostState>(AsyncState.idle<PostViewModel>(null));

	constructor(private readonly createPostUseCase: CreatePostUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (params: CreatePostParams) => this.createPost(params);
	}

	private async createPost(params: CreatePostParams): Promise<PostState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: PostState;
		try {
			const post = await this.createPostUseCase.execute(params);
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
