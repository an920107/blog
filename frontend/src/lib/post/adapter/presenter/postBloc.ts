import { StatusType, type AsyncState } from '$lib/common/adapter/presenter/asyncState';
import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
import type { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { get, writable } from 'svelte/store';

export type PostState = AsyncState<PostViewModel>;
export type PostEvent = PostLoadedEvent;

export class PostBloc {
	private readonly state = writable<PostState>({
		status: StatusType.Idle,
	});

	constructor(
		private readonly getPostUseCase: GetPostUseCase,
		initialData?: PostViewModel
	) {
		this.state.set({
			status: StatusType.Idle,
			data: initialData,
		});
	}

	get subscribe() {
		return this.state.subscribe;
	}

	async dispatch(event: PostEvent): Promise<PostState> {
		switch (event.event) {
			case PostEventType.PostLoadedEvent:
				return this.loadPost(event.id);
		}
	}

	private async loadPost(id: string): Promise<PostState> {
		this.state.set({ status: StatusType.Loading, data: get(this.state).data });

		const post = await this.getPostUseCase.execute(id);
		if (!post) {
			this.state.set({ status: StatusType.Error, error: new Error('Post not found') });
			return get(this.state);
		}

		const postViewModel = PostViewModel.fromEntity(post);
		const result: PostState = {
			status: StatusType.Success,
			data: postViewModel,
		};

		this.state.set(result);
		return result;
	}
}

export enum PostEventType {
	PostLoadedEvent,
}

interface PostLoadedEvent {
	event: PostEventType.PostLoadedEvent;
	id: string;
}
