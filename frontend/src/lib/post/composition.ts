import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
import { PostCreatedStore } from '$lib/post/adapter/presenter/store/postCreatedStore';
import { PostLoadedStore } from '$lib/post/adapter/presenter/store/postLoadedStore';
import { PostsListedStore } from '$lib/post/adapter/presenter/store/postsListedStore';
import { PostUpdatedStore } from '$lib/post/adapter/presenter/store/postUpdatedStore';
import type { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
import type { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
import type { PostRepository } from '$lib/post/application/port/postRepository';
import { CreatePostUseCase } from '$lib/post/application/useCase/createPostUseCase';
import { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { UpdatePostUseCase } from '$lib/post/application/useCase/updatePostUseCase';
import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
import type { ApiClient } from '$lib/shared/framework/http/apiClient';

export function createPostModule(apiClient: ApiClient) {
	const apiService = new PostApiServiceImpl(apiClient);
	const repository: PostRepository = new PostRepositoryImpl(apiService);
	const getAllPostsUseCase = new GetAllPostsUseCase(repository);
	const getPostUseCase = new GetPostUseCase(repository);
	const createPostUseCase = new CreatePostUseCase(repository);
	const updatePostUseCase = new UpdatePostUseCase(repository);

	return {
		getAllPostsUseCase,
		createPostsListedStore: (initialData?: readonly PostInfoViewModel[]) =>
			new PostsListedStore(getAllPostsUseCase, initialData),
		createPostLoadedStore: (initialData?: PostViewModel) =>
			new PostLoadedStore(getPostUseCase, initialData),
		createPostCreatedStore: () => new PostCreatedStore(createPostUseCase),
		createPostUpdatedStore: () => new PostUpdatedStore(updatePostUseCase),
	};
}

export type PostModule = ReturnType<typeof createPostModule>;
