<script lang="ts">
	import { setContext } from 'svelte';

	import { PostCreatedStore } from '$lib/post/adapter/presenter/store/postCreatedStore';
	import { PostsListedStore } from '$lib/post/adapter/presenter/store/postsListedStore';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
	import PostOverallDashboardPage from '$lib/post/framework/ui/PostOverallDashboardPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContainer();

	const postCreatedStore = container.createPostCreatedStore();
	setContext(PostCreatedStore.name, postCreatedStore);

	const getInitialData = () =>
		data.dehydratedData?.map((post) => PostInfoViewModel.rehydrate(post));
	const postsListedStore = container.createPostsListedStore(getInitialData());
	setContext(PostsListedStore.name, postsListedStore);
</script>

<PostOverallDashboardPage />
