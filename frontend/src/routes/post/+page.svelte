<script lang="ts">
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';
	import { Container } from '$lib/container';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import generateTitle from '$lib/common/framework/ui/generateTitle';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const initialData = data.dehydratedData?.map((post) => PostInfoViewModel.rehydrate(post));
	const store = container.createPostsListedStore(initialData);
	setContext(PostsListedStore.name, store);
</script>

<svelte:head>
	<title>{generateTitle('文章')}</title>
	<meta
		name="description"
		content="探索 魚之魷魂 SquidSpirit 的所有文章，這裡是您尋找最新技術洞見與實用教學的園地。"
	/>
</svelte:head>

<PostOverallPage />
