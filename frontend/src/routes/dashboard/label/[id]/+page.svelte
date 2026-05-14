<script lang="ts">
	import { getContext, setContext } from 'svelte';

	import { Container } from '$lib/container';
	import { LabelLoadedStore } from '$lib/label/adapter/presenter/labelLoadedStore';
	import { LabelUpdatedStore } from '$lib/label/adapter/presenter/labelUpdatedStore';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import LabelContentDashboardPage from '$lib/label/framework/ui/LabelContentDashboardPage.svelte';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const id = $derived(data.id);
	const numericId = $derived(Number(id));

	const container = getContext<Container>(Container.name);

	const getInitialLabelData = () => LabelViewModel.rehydrate(data.dehydratedLabel);
	const labelLoadedStore = container.createLabelLoadedStore(getInitialLabelData());
	setContext(LabelLoadedStore.name, labelLoadedStore);

	const labelUpdatedStore = container.createLabelUpdatedStore();
	setContext(LabelUpdatedStore.name, labelUpdatedStore);

	const getInitialPostsData = () =>
		data.dehydratedPosts?.map((post) => PostInfoViewModel.rehydrate(post));
	const postsListedStore = container.createPostsListedStore(getInitialPostsData());
	setContext(PostsListedStore.name, postsListedStore);
</script>

<LabelContentDashboardPage id={numericId} />
