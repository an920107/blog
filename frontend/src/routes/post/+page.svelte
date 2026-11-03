<script lang="ts">
	import { setContext } from 'svelte';

	import { PostsListedStore } from '$lib/post/adapter/presenter/store/postsListedStore';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContainer();

	const keyword = $derived(data.searchParams.keyword);
	const labelId = $derived(data.searchParams.labelId);

	const getInitialPosts = () =>
		data.dehydratedData.posts?.map((post) => PostInfoViewModel.rehydrate(post));
	const postListedStore = container.createPostsListedStore(getInitialPosts());
	setContext(PostsListedStore.name, postListedStore);

	$effect(() => {
		const posts = data.dehydratedData.posts?.map((post) => PostInfoViewModel.rehydrate(post));
		if (posts) {
			postListedStore.setData(posts);
		}
	});
</script>

<PostOverallPage {keyword} {labelId} />
