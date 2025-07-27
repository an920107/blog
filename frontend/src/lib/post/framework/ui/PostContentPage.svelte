<script lang="ts">
	import { PostBloc, PostEventType } from '$lib/post/adapter/presenter/postBloc';
	import PostContentHeader from '$lib/post/framework/ui/PostContentHeader.svelte';
	import { getContext, onMount } from 'svelte';
	import markdownit from 'markdown-it';
	import SafeHtml from '$lib/common/framework/ui/SafeHtml.svelte';

	const { id }: { id: number } = $props();

	const postBloc = getContext<PostBloc>(PostBloc.name);
	const state = $derived($postBloc);

	const md = markdownit();
	const parsedContent = $derived(state.data?.content ? md.render(state.data.content) : '');

	onMount(() => postBloc.dispatch({ event: PostEventType.PostLoadedEvent, id: id }));
</script>

<article class="container prose pb-10 prose-gray">
	{#if state.data}
		<PostContentHeader postInfo={state.data.info} />
		<div class="max-w-3xl">
			<hr />
			<SafeHtml html={parsedContent} />
		</div>
	{/if}
</article>
