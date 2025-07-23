<script lang="ts">
	import { StatusType } from '$lib/common/adapter/presenter/asyncState';
	import { PostListBloc, PostListEventType } from '$lib/post/adapter/presenter/postListBloc';
	import PostPreview from '$lib/post/framework/ui/PostPreview.svelte';
	import { getContext, onMount } from 'svelte';

	const postListBloc = getContext<PostListBloc>(PostListBloc.name);
	const state = $derived($postListBloc);

	onMount(() => postListBloc.dispatch({ event: PostListEventType.PostListLoadedEvent }));
</script>

<div class="container">
	<div class="py-9 text-center text-3xl font-bold text-gray-800 md:py-20 md:text-5xl">文章</div>
	{#if state.status === StatusType.Success}
		<div class="grid grid-cols-1 gap-6 md:grid-cols-2 md:gap-y-8 lg:grid-cols-3">
			{#each state.data as postInfo (postInfo.id)}
				<PostPreview {postInfo} />
			{/each}
		</div>
	{/if}
</div>
