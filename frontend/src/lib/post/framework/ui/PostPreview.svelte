<script lang="ts">
	import type { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import PostPreviewLabels from '$lib/post/framework/ui/PostPreviewLabels.svelte';

	const { postInfo }: { postInfo: PostInfoViewModel } = $props();

	let isImageLoading = $state(true);
	let isImageError = $state(false);

	function handleImageLoad() {
		isImageLoading = false;
	}

	function handleImageError() {
		isImageLoading = false;
		isImageError = true;
	}
</script>

<a class="flex cursor-pointer flex-col gap-y-6" href="/post/{postInfo.id}">
	<div class="relative aspect-video overflow-hidden rounded-2xl bg-gray-200">
		<img
			class="rounded-2xl object-cover transition-opacity duration-300
                {isImageLoading ? 'opacity-0' : 'opacity-100'}
                {isImageError ? 'hidden' : ''}"
			src={postInfo.previewImageUrl.href}
			alt={postInfo.title}
			onload={handleImageLoad}
			onerror={handleImageError}
		/>
		{#if isImageLoading || isImageError}
			<div class="absolute inset-0 flex items-center justify-center bg-gray-200"></div>
		{/if}
	</div>
	<div class="flex flex-col gap-y-2.5">
		<PostPreviewLabels labels={postInfo.labels} />
		<span class="line-clamp-1 text-lg font-bold">{postInfo.title}</span>
		<span class="line-clamp-3 text-justify text-sm">{postInfo.description}</span>
		<span class="text-sm text-gray-500">{postInfo.formattedPublishedTime}</span>
	</div>
</a>
