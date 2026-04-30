<script lang="ts">
	import { getContext, setContext } from 'svelte';

	import { Container } from '$lib/container';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import { PostUpdatedStore } from '$lib/post/adapter/presenter/postUpdatedStore';
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import PostContentDashboardPage from '$lib/post/framework/ui/PostContentDashboardPage.svelte';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const id = $derived(data.id);

	const container = getContext<Container>(Container.name);
	const getInitialData = () => PostViewModel.rehydrate(data.dehydratedData);
	const postLoadedStore = container.createPostLoadedStore(getInitialData());
	const postUpdatedStore = container.createPostUpdatedStore();
	const labelsLiestedStore = container.createLabelsListedStore();

	setContext(PostLoadedStore.name, postLoadedStore);
	setContext(PostUpdatedStore.name, postUpdatedStore);
	setContext(LabelsListedStore.name, labelsLiestedStore);
</script>

<PostContentDashboardPage {id} />
