<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import OpenGraph from '$lib/common/framework/ui/OpenGraph.svelte';
	import StructuredData from '$lib/common/framework/ui/StructuredData.svelte';
	import { Environment } from '$lib/environment';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import PostPreview from '$lib/post/framework/ui/PostPreview.svelte';
	import { getContext, onMount } from 'svelte';

	const store = getContext<PostsListedStore>(PostsListedStore.name);
	const state = $derived($store);
	const { trigger: loadPosts } = store;

	const description =
		'探索 魚之魷魂 SquidSpirit 的所有文章，這裡是您尋找最新技術洞見與實用教學的園地。';

	onMount(() => loadPosts());
</script>

<svelte:head>
	<title>{generateTitle('文章')}</title>
	<meta name="description" content={description} />
</svelte:head>

{#if state.data}
	<StructuredData
		props={{
			type: 'CollectionPage',
			url: new URL('post', Environment.APP_BASE_URL),
			name: '文章',
			description: description,
			mainEntity: {
				numberOfItems: state.data.length,
				itemListElement: state.data.map((postInfo) => ({
					type: 'BlogPosting',
					url: new URL(`post/${postInfo.semanticId}`, Environment.APP_BASE_URL),
					headline: postInfo.title,
					name: postInfo.title,
					description: postInfo.description,
					datePublished: postInfo.publishedTime!.nativeDate,
					image: postInfo.previewImageUrl,
					articleSection: postInfo.labels.map((label) => label.name),
				})),
			},
		}}
	/>
{/if}

<OpenGraph
	title="文章"
	{description}
	url={new URL('post', Environment.APP_BASE_URL)}
	image={new URL('favicon.svg', Environment.APP_BASE_URL)}
	labels={[]}
/>

<div class="content-container pb-10">
	<h1 class="py-9 text-center text-3xl font-bold text-gray-800 md:py-20 md:text-5xl">文章</h1>
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 md:gap-y-8 lg:grid-cols-3">
		{#each state.data ?? [] as postInfo (postInfo.id)}
			<PostPreview {postInfo} />
		{/each}
	</div>
</div>
