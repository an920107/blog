<script lang="ts">
	import { setContext } from 'svelte';

	import { LabelsListedStore } from '$lib/label/adapter/presenter/store/labelsListedStore';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/store/postLoadedStore';
	import { PostUpdatedStore } from '$lib/post/adapter/presenter/store/postUpdatedStore';
	import { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
	import PostContentDashboardPage from '$lib/post/framework/ui/PostContentDashboardPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const id = $derived(data.id);

	const container = getContainer();
	const getInitialData = () => PostViewModel.rehydrate(data.dehydratedData);
	const postLoadedStore = container.createPostLoadedStore(getInitialData());
	const postUpdatedStore = container.createPostUpdatedStore();
	const labelsListedStore = container.createLabelsListedStore();

	setContext(PostLoadedStore.name, postLoadedStore);
	setContext(PostUpdatedStore.name, postUpdatedStore);
	setContext(LabelsListedStore.name, labelsListedStore);
</script>

<PostContentDashboardPage {id} />
