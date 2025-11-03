<script lang="ts">
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { Container } from '$lib/container';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import { LabelLoadedStore } from '$lib/label/adapter/presenter/labelLoadedStore';
	import { LabelUpdatedStore } from '$lib/label/adapter/presenter/labelUpdatedStore';
	import LabelContentDashboardPage from '$lib/label/framework/ui/LabelContentDashboardPage.svelte';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';

	const { data }: PageProps = $props();

	const container = getContext<Container>(Container.name);

	const initialLabelData = LabelViewModel.rehydrate(data.dehydratedLabel);
	const labelLoadedStore = container.createLabelLoadedStore(initialLabelData);
	setContext(LabelLoadedStore.name, labelLoadedStore);

	const labelUpdatedStore = container.createLabelUpdatedStore();
	setContext(LabelUpdatedStore.name, labelUpdatedStore);

	const initialPostsData = data.dehydratedPosts?.map((post) => PostInfoViewModel.rehydrate(post));
	const postsListedStore = container.createPostsListedStore(initialPostsData);
	setContext(PostsListedStore.name, postsListedStore);
</script>

<LabelContentDashboardPage />
