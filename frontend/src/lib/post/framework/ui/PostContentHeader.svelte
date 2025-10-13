<script lang="ts">
	import type { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import PostLabel from '$lib/post/framework/ui/PostLabel.svelte';

	const { postInfo }: { postInfo: PostInfoViewModel } = $props();
</script>

<div class="flex flex-col pt-9 md:pt-20">
	<div class="mb-4 flex flex-row gap-2">
		{#each postInfo.labels as label (label.id)}
			<PostLabel {label} />
		{/each}
	</div>
	<h1 class="text-3xl font-bold text-gray-800 sm:text-4xl md:text-5xl">{postInfo.title}</h1>
	<p class="max-w-3xl">{postInfo.description}</p>
	<span class="text-gray-500">{postInfo.formattedPublishedTime}</span>
</div>
