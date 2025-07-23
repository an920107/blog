<script lang="ts">
	import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
	import { PostListBloc } from '$lib/post/adapter/presenter/postListBloc';
	import { GetAllPostUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
	import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';
	import { setContext } from 'svelte';

	const postApiService = new PostApiServiceImpl();
	const postRepository = new PostRepositoryImpl(postApiService);
	const getAllPostsUseCase = new GetAllPostUseCase(postRepository);
	const postListBloc = new PostListBloc(getAllPostsUseCase);

	setContext(PostListBloc.name, postListBloc);
</script>

<PostOverallPage />
