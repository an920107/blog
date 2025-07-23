<script lang="ts">
	import type { LabelViewModel } from '$lib/post/adapter/presenter/labelViewModel';

	const { labels }: { labels: readonly LabelViewModel[] } = $props();
</script>

<div class="flex flex-row gap-x-2 text-xs">
	{#each labels.slice(0, 2) as label (label.id)}
		<div
			class="flex flex-row items-center gap-x-1 rounded-full px-2 py-0.5"
			style="background-color: {label.color.hex};"
		>
			<div
				class="size-2 rounded-full"
				style="background-color: {label.color.darken(0.2).hex};"
			></div>
			<span>{label.name}</span>
		</div>
	{/each}
	{#if labels.length > 2}
		<div class="rounded-full bg-gray-200 px-2 py-0.5">
			<span>+{labels.length - 2}</span>
		</div>
	{/if}
</div>
