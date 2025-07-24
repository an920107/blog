<script lang="ts">
	import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
	import { PostBloc } from '$lib/post/adapter/presenter/postBloc';
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
	import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
	import { setContext } from 'svelte';
	import type { PageProps } from './$types';
	import PostContentPage from '$lib/post/framework/ui/PostContentPage.svelte';

	const { data, params }: PageProps = $props();

	const id = parseInt(params.id, 10);

	const initialData = PostViewModel.rehydrate(data.dehydratedData!);

	const postApiService = new PostApiServiceImpl(fetch);
	const postRepository = new PostRepositoryImpl(postApiService);
	const getPostUseCase = new GetPostUseCase(postRepository);
	const postBloc = new PostBloc(getPostUseCase, initialData);

	setContext(PostBloc.name, postBloc);
</script>

<PostContentPage {id} />
