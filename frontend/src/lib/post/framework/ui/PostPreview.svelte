<script lang="ts">
	import { cn } from '$lib/common/framework/components/utils';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import type { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';

	const { postInfo }: { postInfo: PostInfoViewModel } = $props();

	let isImageLoading = $state(true);
	let isImageError = $state(false);

	function onImageLoad() {
		isImageLoading = false;
	}

	function onImageError() {
		isImageLoading = false;
		isImageError = true;
	}
</script>

<a
	class={cn(
		'flex cursor-pointer flex-col gap-y-6',
		'group transition-transform duration-300 hover:scale-[1.02] hover:transform'
	)}
	href="/post/{postInfo.semanticId}"
	title={postInfo.title}
>
	<div
		class={cn(
			'relative aspect-video overflow-hidden rounded-2xl bg-gray-200',
			'transition-shadow duration-300 group-hover:shadow-xl group-hover:shadow-black/5'
		)}
	>
		<img
			class={cn(
				'h-full w-full rounded-2xl object-cover object-center transition-opacity duration-300',
				isImageLoading ? 'opacity-0' : 'opacity-100',
				isImageError ? 'hidden' : ''
			)}
			src={postInfo.previewImageUrl?.href}
			alt={postInfo.title}
			onload={onImageLoad}
			onerror={onImageError}
		/>
		{#if isImageLoading || isImageError}
			<div class="absolute inset-0 flex items-center justify-center bg-gray-200"></div>
		{/if}
	</div>
	<div class="flex flex-col gap-y-2.5">
		{@render labelsView(postInfo.labels)}
		<h2 class="line-clamp-1 text-lg font-bold text-gray-800">{postInfo.title}</h2>
		<p class="line-clamp-3 text-justify text-sm">{postInfo.description}</p>
		<span class="text-sm text-gray-500">{postInfo.publishedTime?.toLocalISODate()}</span>
	</div>
</a>

{#snippet labelsView(labels: readonly LabelViewModel[])}
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
{/snippet}
