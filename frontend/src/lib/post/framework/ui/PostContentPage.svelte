<script lang="ts">
	import { getContext, onDestroy, onMount, tick } from 'svelte';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import StructuredData from '$lib/common/framework/ui/StructuredData.svelte';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import MarkdownRenderer, {
		type HeadingItem,
	} from '$lib/post/framework/ui/MarkdownRenderer.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { fade } from 'svelte/transition';
	import { cn } from '$lib/common/framework/components/utils';
	import OpenGraph from '../../../common/framework/ui/OpenGraph.svelte';
	import { Environment } from '$lib/environment';
	import { DrawerConfiguredStore } from '$lib/common/adapter/presenter/drawerConfiguredStore';

	const { id }: { id: string } = $props();

	const pageLoadedstore = getContext<PostLoadedStore>(PostLoadedStore.name);
	const pageLoadedstate = $derived($pageLoadedstore);
	const { trigger: loadPost } = pageLoadedstore;

	const drawerConfiguredStore = getContext<DrawerConfiguredStore>(DrawerConfiguredStore.name);
	const drawerConfiguredState = $derived($drawerConfiguredStore);
	const drawerViewModel = $derived(drawerConfiguredState?.data);
	const { trigger: configureDrawer } = drawerConfiguredStore;

	const post = $derived(pageLoadedstate.data);
	const postInfo = $derived(post?.info);
	const content = $derived(post?.content ?? '');

	let headings: HeadingItem[] = $state([]);
	let activeHeadingId: string | null = $state(null);

	async function smoothScrollToHeading(headingId: string) {
		if (drawerViewModel?.isOpen) {
			configureDrawer(drawerViewModel.copyWith({ isOpen: false }));
			await tick();
		}

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
		if (drawerViewModel) {
			configureDrawer(drawerViewModel.copyWith({ content: tocWithPadding }));
		}

		const handleScroll = () => updateActiveHeading();
		window.addEventListener('scroll', handleScroll);
		onDestroy(() => {
			window.removeEventListener('scroll', handleScroll);
		});
	});

	onDestroy(() => {
		if (drawerViewModel) {
			configureDrawer(drawerViewModel.copyWith({ isOpen: false, content: null }));
		}
	});
</script>

<svelte:head>
	<title>{generateTitle(postInfo?.title)}</title>
	{#if postInfo}
		<meta name="description" content={postInfo.description} />
	{/if}
</svelte:head>

{#if postInfo?.isPublished}
	<StructuredData
		props={{
			type: 'BlogPosting',
			url: new URL(`post/${postInfo.semanticId}`, Environment.APP_BASE_URL),
			headline: postInfo.title,
			name: postInfo.title,
			description: postInfo.description,
			datePublished: postInfo.publishedTime!.nativeDate,
			image: postInfo.previewImageUrl,
			articleSection: postInfo.labels.map((label) => label.name),
		}}
	/>
	<OpenGraph
		title={postInfo.title}
		description={postInfo.description}
		publishedTime={postInfo.publishedTime!.nativeDate}
		labels={postInfo.labels.map((label) => label.name)}
		url={new URL(`post/${postInfo.semanticId}`, Environment.APP_BASE_URL)}
		image={postInfo.previewImageUrl}
	/>
{/if}

<div class="content-container pb-16 md:flex md:flex-row md:gap-6">
	<article
		class={cn(
			'prose max-w-screen prose-gray',
			'md:max-w-lg min-[50rem]:max-w-xl min-[56rem]:max-w-2xl lg:max-w-3xl'
		)}
	>
		{@render header()}
		<hr />
		<MarkdownRenderer {content} onHeadingUpdate={(val) => (headings = val)} />
	</article>
	{#if headings.length > 0}
		<div transition:fade class="ms-auto max-w-xs min-w-0 pt-32 max-md:hidden">
			<div class="sticky top-24 max-h-[calc(100vh-6rem)] overflow-y-auto">
				{@render toc()}
			</div>
		</div>
	{/if}
</div>

{#snippet header()}
	<div class="flex flex-col pt-9 md:pt-20">
		<div class="mb-4 flex flex-row flex-wrap gap-2">
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
	<div class="space-y-1">
		<p class="mb-2 truncate font-medium text-gray-800">章節目錄</p>
		{#each headings as heading (heading.id)}
			{@const padding = (heading.level - 2) * 1.5}
			{@const isActive = activeHeadingId === heading.id}
			<div style="padding-left: {padding}rem;">
				<button
					class={cn(
						'w-fit max-w-full overflow-hidden text-left',
						'font-light text-nowrap text-ellipsis decoration-gray-400 hover:underline',
						isActive ? 'text-gray-900' : 'text-gray-400'
					)}
					onclick={() => smoothScrollToHeading(heading.id)}
				>
					{heading.text}
				</button>
			</div>
		{/each}
	</div>
{/snippet}

{#snippet tocWithPadding()}
	<div class="px-2.5">
		{@render toc()}
	</div>
{/snippet}
