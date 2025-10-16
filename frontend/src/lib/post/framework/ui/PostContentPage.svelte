<script lang="ts">
	import { getContext, onDestroy, onMount } from 'svelte';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import StructuredData from '$lib/post/framework/ui/StructuredData.svelte';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import MardownRenderer, { type HeadingItem } from '$lib/post/framework/ui/MardownRenderer.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { fade } from 'svelte/transition';

	const { id }: { id: string } = $props();

	const pageLoadedstore = getContext<PostLoadedStore>(PostLoadedStore.name);
	const pageLoadedstate = $derived($pageLoadedstore);
	const { trigger: loadPost } = pageLoadedstore;

	const post = $derived(pageLoadedstate.data);
	const postInfo = $derived(post?.info);
	const content = $derived(post?.content ?? '');

	let headings: HeadingItem[] = $state([]);
	let activeHeadingId: string | null = $state(null);

	function smoothScrollToHeading(headingId: string) {
		const element = document.getElementById(headingId);
		if (element) {
			element.scrollIntoView({
				behavior: 'smooth',
				block: 'start',
			});
		}
	}

	function updateActiveHeading() {
		if (headings.length === 0) {
			return;
		}

		let currentHeadingId: string | null = null;

		for (const heading of headings) {
			const element = document.getElementById(heading.id);
			if (element) {
				const rect = element.getBoundingClientRect();

				if (rect.top <= window.innerHeight / 4) {
					currentHeadingId = heading.id;
				} else {
					break;
				}
			}
		}

		activeHeadingId = currentHeadingId;
	}

	$effect(() => {
		if (headings.length > 0) {
			updateActiveHeading();
		}
	});

	onMount(() => {
		loadPost(id);

		const handleScroll = () => updateActiveHeading();
		window.addEventListener('scroll', handleScroll);
		onDestroy(() => {
			window.removeEventListener('scroll', handleScroll);
		});
	});
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
<div class="content-container pb-16 lg:flex lg:flex-row lg:gap-12">
	<article class="prose max-w-3xl prose-gray lg:flex-1">
		{@render header()}
		<hr />
		<MardownRenderer {content} onHeadingUpdate={(val) => (headings = val)} />
	</article>
	{@render toc()}
</div>

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

{#snippet toc()}
	{#if headings.length > 0}
		<div transition:fade class="ms-auto min-w-0 pt-32 max-lg:hidden">
			<div class="sticky top-toolbar-height max-h-content-height space-y-1">
				<p class="mb-2 truncate font-medium text-gray-600">章節目錄</p>
				{#each headings as heading (heading.id)}
					{@const padding = (heading.level - 2) * 1.5}
					{@const isActive = activeHeadingId === heading.id}
					<p class="truncate font-light" style="padding-left: {padding}rem;">
						<button
							class="decoration-gray-400 hover:underline {isActive
								? 'text-gray-900'
								: 'text-gray-400'}"
							onclick={() => smoothScrollToHeading(heading.id)}
						>
							{heading.text}
						</button>
					</p>
				{/each}
			</div>
		</div>
	{/if}
{/snippet}
