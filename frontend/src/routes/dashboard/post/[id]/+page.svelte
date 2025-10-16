<script lang="ts">
	import { Container } from '$lib/container';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import { PostUpdatedStore } from '$lib/post/adapter/presenter/postUpdatedStore';
	import PostContentDashboardPage from '$lib/post/framework/ui/PostContentDashboardPage.svelte';

	const { data }: PageProps = $props();
	const { id } = data;

	const container = getContext<Container>(Container.name);
	const postLoadedStore = container.createPostLoadedStore(
		PostViewModel.rehydrate(data.dehydratedData)
	);
	const postUpdatedStore = container.createPostUpdatedStore();

	setContext(PostLoadedStore.name, postLoadedStore);
	setContext(PostUpdatedStore.name, postUpdatedStore);
</script>

<PostContentDashboardPage {id} />
