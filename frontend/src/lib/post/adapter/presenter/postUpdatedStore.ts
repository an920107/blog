import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
import type { UpdatePostUseCase } from '$lib/post/application/useCase/updatePostUseCase';
import type { UpdatePostParams } from '$lib/post/application/gateway/postRepository';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type PostUpdatedState = AsyncState<PostViewModel>;

interface PostUpdatedTriggerParams {
	id: number;
	params: UpdatePostParams;
}

export class PostUpdatedStore implements BaseStore<PostUpdatedState, PostUpdatedTriggerParams> {
	static readonly name = 'PostUpdatedStore';

	private readonly state = writable<PostUpdatedState>(AsyncState.idle<PostViewModel>(null));

	constructor(private readonly updatePostUseCase: UpdatePostUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (request: PostUpdatedTriggerParams) => this.updatePost(request);
	}

	private async updatePost(request: PostUpdatedTriggerParams): Promise<PostUpdatedState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: PostUpdatedState;
		try {
			const post = await this.updatePostUseCase.execute(request.id, request.params);
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
