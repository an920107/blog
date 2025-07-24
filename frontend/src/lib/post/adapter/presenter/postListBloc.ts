import { StatusType, type AsyncState } from '$lib/common/adapter/presenter/asyncState';
import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
import type { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { get, writable } from 'svelte/store';

export type PostListState = AsyncState<readonly PostInfoViewModel[]>;
export type PostListEvent = PostListLoadedEvent;

export class PostListBloc {
	private readonly state = writable<PostListState>({
		status: StatusType.Idle
	});

	constructor(
		private readonly getAllPostsUseCase: GetAllPostsUseCase,
		initialData?: readonly PostInfoViewModel[]
	) {
		this.state.set({
			status: StatusType.Idle,
			data: initialData
		});
	}

	get subscribe() {
		return this.state.subscribe;
	}

	async dispatch(event: PostListEvent): Promise<PostListState> {
		switch (event.event) {
			case PostListEventType.PostListLoadedEvent:
				return this.loadPosts();
		}
	}

	private async loadPosts(): Promise<PostListState> {
		this.state.set({ status: StatusType.Loading, data: get(this.state).data });
		const posts = await this.getAllPostsUseCase.execute();
		const postViewModels = posts.map((post) => PostInfoViewModel.fromEntity(post));
		const result: PostListState = {
			status: StatusType.Success,
			data: postViewModels
		};

		this.state.set(result);
		return result;
	}
}

export enum PostListEventType {
	PostListLoadedEvent
}

export interface PostListLoadedEvent {
	event: PostListEventType.PostListLoadedEvent;
}
