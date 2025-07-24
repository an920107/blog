import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
import { PostBloc } from '$lib/post/adapter/presenter/postBloc';
import { PostListBloc } from '$lib/post/adapter/presenter/postListBloc';
import { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = ({ event, resolve }) => {
	const postApiService = new PostApiServiceImpl(event.fetch);
	const postRepository = new PostRepositoryImpl(postApiService);
	const getAllPostsUseCase = new GetAllPostsUseCase(postRepository);
	const getPostUseCase = new GetPostUseCase(postRepository);

	event.locals.postListBloc = new PostListBloc(getAllPostsUseCase);
	event.locals.postBloc = new PostBloc(getPostUseCase);

	return resolve(event);
};
