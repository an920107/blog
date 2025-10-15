<script lang="ts">
	import { Container } from '$lib/container';
	import { PostCreatedStore } from '$lib/post/adapter/presenter/postCreatedStore';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import PostOverallDashboardPage from '$lib/post/framework/ui/PostOverallDashboardPage.svelte';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const postCreatedStore = container.createPostCreatedStore();
	setContext(PostCreatedStore.name, postCreatedStore);

	const initialData = data.dehydratedData?.map((post) => PostInfoViewModel.rehydrate(post));
	const postsListedStore = container.createPostsListedStore(initialData);
	setContext(PostsListedStore.name, postsListedStore);
</script>

<PostOverallDashboardPage />
