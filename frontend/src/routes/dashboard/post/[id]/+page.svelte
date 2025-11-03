<script lang="ts">
	import { Container } from '$lib/container';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import { PostUpdatedStore } from '$lib/post/adapter/presenter/postUpdatedStore';
	import PostContentDashboardPage from '$lib/post/framework/ui/PostContentDashboardPage.svelte';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';

	const { data }: PageProps = $props();
	const { id } = data;

	const container = getContext<Container>(Container.name);
	const postLoadedStore = container.createPostLoadedStore(
		PostViewModel.rehydrate(data.dehydratedData)
	);
	const postUpdatedStore = container.createPostUpdatedStore();
	const labelsLiestedStore = container.createLabelsListedStore();

	setContext(PostLoadedStore.name, postLoadedStore);
	setContext(PostUpdatedStore.name, postUpdatedStore);
	setContext(LabelsListedStore.name, labelsLiestedStore);
</script>

<PostContentDashboardPage {id} />
