<script lang="ts">
	import { PostBloc, PostEventType } from '$lib/post/adapter/presenter/postBloc';
	import PostContentHeader from '$lib/post/framework/ui/PostContentHeader.svelte';
	import { getContext, onMount } from 'svelte';
	import markdownit from 'markdown-it';
	import SafeHtml from '$lib/common/framework/ui/SafeHtml.svelte';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import StructuredData from '$lib/post/framework/ui/StructuredData.svelte';

	const { id }: { id: string } = $props();

	const postBloc = getContext<PostBloc>(PostBloc.name);
	const state = $derived($postBloc);

	const md = markdownit();
	const parsedContent = $derived(state.data?.content ? md.render(state.data.content) : '');

	onMount(() => postBloc.dispatch({ event: PostEventType.PostLoadedEvent, id: id }));
</script>

<svelte:head>
	<title>{generateTitle(state.data?.info.title)}</title>
	{#if state.data}
		<meta name="description" content={state.data.info.description} />
		{#if state.data.info.isPublished}
			<StructuredData
				headline={state.data.info.title}
				description={state.data.info.description}
				datePublished={state.data.info.publishedTime!}
				image={state.data.info.previewImageUrl}
			/>
		{/if}
	{/if}
</svelte:head>
<article class="container prose pb-10 prose-gray">
	{#if state.data}
		<PostContentHeader postInfo={state.data.info} />
		<div class="max-w-3xl">
			<hr />
			<SafeHtml html={parsedContent} />
		</div>
	{/if}
</article>
