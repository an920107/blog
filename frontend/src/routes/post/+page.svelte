<script lang="ts">
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';
	import { Container } from '$lib/container';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const initialData = data.dehydratedData?.map((post) => PostInfoViewModel.rehydrate(post));
	const store = container.createPostsListedStore(initialData);
	setContext(PostsListedStore.name, store);
</script>

<PostOverallPage />
