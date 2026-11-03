<script lang="ts">
	import { setContext } from 'svelte';

	import { LabelLoadedStore } from '$lib/label/adapter/presenter/store/labelLoadedStore';
	import { LabelUpdatedStore } from '$lib/label/adapter/presenter/store/labelUpdatedStore';
	import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
	import LabelContentDashboardPage from '$lib/label/framework/ui/LabelContentDashboardPage.svelte';
	import { PostsListedStore } from '$lib/post/adapter/presenter/store/postsListedStore';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const id = $derived(data.id);
	const numericId = $derived(Number(id));

	const container = getContainer();

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
