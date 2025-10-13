<script lang="ts">
	import type { LabelViewModel } from '$lib/post/adapter/presenter/labelViewModel';
	import PostLabel from '$lib/post/framework/ui/PostLabel.svelte';

	const { labels }: { labels: readonly LabelViewModel[] } = $props();
</script>

<div class="flex flex-row gap-x-2">
	{#each labels.slice(0, 2) as label (label.id)}
		<PostLabel {label} />
	{/each}
	{#if labels.length > 2}
		<div class="rounded-full bg-gray-200 px-2 py-0.5 text-xs">
			<span>+{labels.length - 2}</span>
		</div>
	{/if}
</div>
