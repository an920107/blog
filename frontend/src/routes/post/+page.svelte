<script lang="ts">
	import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
	import { PostListBloc } from '$lib/post/adapter/presenter/postListBloc';
	import { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
	import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
	import { setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';
	import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
	import type { PostRepository } from '$lib/post/application/gateway/postRepository';

	let { data }: PageProps = $props();

	const initialData = data.dehydratedData?.map((post) => PostInfoViewModel.rehydrate(post));

	const postApiService: PostApiService = new PostApiServiceImpl(fetch);
	const postRepository: PostRepository = new PostRepositoryImpl(postApiService);
	const getAllPostsUseCase = new GetAllPostsUseCase(postRepository);
	const postListBloc = new PostListBloc(getAllPostsUseCase, initialData);

	setContext(PostListBloc.name, postListBloc);
</script>

<PostOverallPage />
