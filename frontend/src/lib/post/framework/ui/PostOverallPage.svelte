<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import PostPreview from '$lib/post/framework/ui/PostPreview.svelte';
	import { getContext } from 'svelte';

	const store = getContext<PostsListedStore>(PostsListedStore.name);
	const state = $derived($store);
</script>

<svelte:head>
	<title>{generateTitle('文章')}</title>
	<meta
		name="description"
		content="探索 魚之魷魂 SquidSpirit 的所有文章，這裡是您尋找最新技術洞見與實用教學的園地。"
	/>
</svelte:head>
<div class="content-container pb-10">
	<h1 class="py-9 text-center text-3xl font-bold text-gray-800 md:py-20 md:text-5xl">文章</h1>
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 md:gap-y-8 lg:grid-cols-3">
		{#each state.data ?? [] as postInfo (postInfo.id)}
			<PostPreview {postInfo} />
		{/each}
	</div>
</div>
