<script lang="ts">
	import PostContentHeader from '$lib/post/framework/ui/PostContentHeader.svelte';
	import { getContext, onMount } from 'svelte';
	import markdownit from 'markdown-it';
	import SafeHtml from '$lib/common/framework/ui/SafeHtml.svelte';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import StructuredData from '$lib/post/framework/ui/StructuredData.svelte';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';

	const { id }: { id: string } = $props();

	const store = getContext<PostLoadedStore>(PostLoadedStore.name);
	const state = $derived($store);
	const { trigger: loadPost } = store;

	const md = markdownit();
	const parsedContent = $derived(state.data?.content ? md.render(state.data.content) : '');
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
	{#if postInfo}
		<PostContentHeader {postInfo} />
		<div class="max-w-3xl">
			<hr />
			<SafeHtml html={parsedContent} />
		</div>
	{/if}
</article>
