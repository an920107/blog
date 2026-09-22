<script lang="ts">
	import { getContext, setContext } from 'svelte';

	import { Container } from '$lib/container';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

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
