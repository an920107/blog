import { StatusType, type AsyncState } from '$lib/common/adapter/presenter/asyncState';
import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
import type { GetAllPostUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { writable } from 'svelte/store';

export class PostListBloc {
	constructor(private readonly getAllPostsUseCase: GetAllPostUseCase) {}

	private readonly state = writable<AsyncState<readonly PostInfoViewModel[]>>({
		status: StatusType.Idle
	});

	get subscribe() {
		return this.state.subscribe;
	}

	dispatch(event: PostListEvent) {
		switch (event.event) {
			case PostListEventType.PostListLoadedEvent:
				this.loadPosts();
				break;
		}
	}

	private async loadPosts() {
		this.state.set({ status: StatusType.Loading });
		const posts = await this.getAllPostsUseCase.execute();
		const postViewModels = posts.map((post) => PostInfoViewModel.fromEntity(post));
		this.state.set({
			status: StatusType.Success,
			data: postViewModels
		});
	}
}

export enum PostListEventType {
	PostListLoadedEvent
}

export interface PostListLoadedEvent {
	event: PostListEventType.PostListLoadedEvent;
}

export type PostListEvent = PostListLoadedEvent;
