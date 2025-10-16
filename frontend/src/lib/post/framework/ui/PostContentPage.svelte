<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import StructuredData from '$lib/post/framework/ui/StructuredData.svelte';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import MardownRenderer from '$lib/post/framework/ui/MardownRenderer.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';

	const { id }: { id: string } = $props();

	const store = getContext<PostLoadedStore>(PostLoadedStore.name);
	const state = $derived($store);
	const { trigger: loadPost } = store;

	const postInfo = $derived(state.data?.info);

	onMount(() => loadPost(id));
</script>

<svelte:head>
	<title>{generateTitle(postInfo?.title)}</title>
	{#if postInfo}
		<meta name="description" content={postInfo.description} />
		{#if postInfo.isPublished}
			<StructuredData
				headline={postInfo.title}
				description={postInfo.description}
				datePublished={postInfo.publishedTime!}
				image={postInfo.previewImageUrl}
			/>
		{/if}
	{/if}
</svelte:head>
<article class="content-container prose pb-10 prose-gray">
	<div class="max-w-3xl">
		{@render header()}
		<hr />
		<MardownRenderer content={state.data?.content ?? ''} />
	</div>
</article>

{#snippet header()}
	<div class="flex flex-col pt-9 md:pt-20">
		<div class="mb-4 flex flex-row gap-2">
			{#each postInfo?.labels ?? [] as label (label.id)}
				<PostLabel {label} />
			{/each}
		</div>
		<h1 class="text-3xl leading-tight font-bold text-gray-800 sm:text-4xl md:text-5xl">
			{postInfo?.title}
		</h1>
		<p>{postInfo?.description}</p>
		<span class="text-gray-500">{postInfo?.publishedTime?.toLocalISODate()}</span>
	</div>
{/snippet}
