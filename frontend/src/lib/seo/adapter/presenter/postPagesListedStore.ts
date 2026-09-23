import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import type { ListPostPagesUseCase } from '$lib/seo/application/useCase/listPostPagesUseCase';
import type { PostPage } from '$lib/seo/domain/entity/postPage';

type PostPagesState = AsyncState<readonly PostPage[]>;

export class PostPagesListedStore implements BaseStore<PostPagesState> {
	private readonly state = writable<PostPagesState>(AsyncState.idle<readonly PostPage[]>(null));

	constructor(private readonly listPostPagesUseCase: ListPostPagesUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return () => this.loadPostPages();
	}

	private async loadPostPages(): Promise<PostPagesState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: PostPagesState;
		try {
			result = AsyncState.success(await this.listPostPagesUseCase.execute());
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
