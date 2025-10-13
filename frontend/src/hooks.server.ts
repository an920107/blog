import { sequence } from '@sveltejs/kit/hooks';
import * as Sentry from '@sentry/sveltekit';
import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
import { PostBloc } from '$lib/post/adapter/presenter/postBloc';
import { PostListBloc } from '$lib/post/adapter/presenter/postListBloc';
import { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
import type { Handle } from '@sveltejs/kit';
import { Environment } from '$lib/environment';
import { AuthApiServiceImpl } from '$lib/auth/framework/api/authApiServiceImpl';
import { AuthRepositoryImpl } from '$lib/auth/adapter/gateway/authRepositoryImpl';
import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
import type { AuthRepository } from '$lib/auth/application/gateway/authRepository';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import type { PostRepository } from '$lib/post/application/gateway/postRepository';
import { AuthBloc } from '$lib/auth/adapter/presenter/authBloc';
import { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';

Sentry.init({
	dsn: Environment.SENTRY_DSN,
	tracesSampleRate: 1,
	enableLogs: true,
});

export const handle: Handle = sequence(Sentry.sentryHandle(), ({ event, resolve }) => {
	const authApiService: AuthApiService = new AuthApiServiceImpl(event.fetch);
	const authRepository: AuthRepository = new AuthRepositoryImpl(authApiService);
	const getCurrentUserUseCase = new GetCurrentUserUseCase(authRepository);

	const postApiService: PostApiService = new PostApiServiceImpl(event.fetch);
	const postRepository: PostRepository = new PostRepositoryImpl(postApiService);
	const getAllPostsUseCase = new GetAllPostsUseCase(postRepository);
	const getPostUseCase = new GetPostUseCase(postRepository);

	event.locals.authBloc = new AuthBloc(getCurrentUserUseCase);
	event.locals.postListBloc = new PostListBloc(getAllPostsUseCase);
	event.locals.postBloc = new PostBloc(getPostUseCase);

	return resolve(event);
});

export const handleError = Sentry.handleErrorWithSentry();
