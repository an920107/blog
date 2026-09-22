<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve */

	import { captureException } from '@sentry/sveltekit';
	import { getContext, onDestroy, onMount, tick } from 'svelte';
	import { fade } from 'svelte/transition';

	import { DrawerConfiguredStore } from '$lib/common/adapter/presenter/drawerConfiguredStore';
	import { cn } from '$lib/common/framework/components/utils';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import OpenGraph from '$lib/common/framework/ui/OpenGraph.svelte';
	import StructuredData from '$lib/common/framework/ui/StructuredData.svelte';
	import { Environment } from '$lib/environment';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { Links } from '$lib/links';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import MarkdownRenderer, {
		type HeadingItem,
	} from '$lib/post/framework/ui/MarkdownRenderer.svelte';
	import ShareButton from '$lib/post/framework/ui/ShareButton.svelte';
	import { Strings } from '$lib/strings';

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

	const canonicalUrl = $derived(new URL(`post/${postInfo?.semanticId}`, Environment.APP_BASE_URL));

	let headings: HeadingItem[] = $state([]);
	let activeHeadingId: string | null = $state(null);

	// Defaults to false so the SSR and first client render match (SSR has no navigator)
	let canShareNatively = $state(false);

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

	function getLabelFilteringUrl(labelId: number): URL {
		const url = new URL('post', Environment.APP_BASE_URL);
		url.searchParams.set('label_id', labelId.toString());
		return url;
	}

	async function shareNatively() {
		if (!canShareNatively) {
			return;
		}

		try {
			await navigator.share({
				title: postInfo?.title ?? '',
				text: postInfo?.description ?? '',
				url: canonicalUrl.href,
			});
		} catch (error) {
			if (error instanceof DOMException && error.name === 'AbortError') {
				// The user dismissed the share sheet, which is expected
				return;
			}

			captureException(error);
		}
	}

	$effect(() => {
		if (headings.length > 0) {
			updateActiveHeading();
		}
	});

	onMount(() => {
		canShareNatively = 'share' in navigator;
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
			url: canonicalUrl,
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
		url={canonicalUrl}
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
				<a
					href={getLabelFilteringUrl(label.id).href}
					class="not-prose"
					title={Strings.LOOK_FOR_POSTS_WITH_SAME_LABEL}
				>
					<PostLabel {label} />
				</a>
			{/each}
		</div>
		<h1 class="text-3xl leading-tight font-bold text-gray-800 sm:text-4xl md:text-5xl">
			{postInfo?.title}
		</h1>
		<p>{postInfo?.description}</p>
		<p class="text-gray-500">{postInfo?.publishedTime?.toLocalISODate()}</p>
		{@render mediaShare()}
	</div>
{/snippet}

{#snippet mediaShare()}
	<div class="flex flex-row gap-2.5">
		<ShareButton url={Links.LINKEDIN_SHARE(canonicalUrl)} label={Strings.SHARE_TO_LINKEDIN}>
			<i class="fa-brands fa-linkedin-in text-[1rem]"></i>
		</ShareButton>
		<ShareButton url={Links.FACEBOOK_SHARE(canonicalUrl)} label={Strings.SHARE_TO_FACEBOOK}>
			<i class="fa-brands fa-facebook-f text-[1rem]"></i>
		</ShareButton>
		<ShareButton
			url={Links.X_SHARE(canonicalUrl, postInfo?.title ?? '')}
			label={Strings.SHARE_TO_X}
		>
			<i class="fa-brands fa-x-twitter text-[1rem]"></i>
		</ShareButton>
		<ShareButton
			url={Links.EMAIL_SHARE(
				canonicalUrl,
				postInfo?.title ?? '',
				`${postInfo?.description ?? ''}\n\n${canonicalUrl.href}`
			)}
			label={Strings.SHARE_TO_EMAIL}
		>
			<i class="fa-solid fa-envelope text-[1rem]"></i>
		</ShareButton>
		{#if canShareNatively}
			<ShareButton label={Strings.SHARE} onclick={shareNatively} class="md:hidden">
				<i class="fa-solid fa-share text-[1rem]"></i>
			</ShareButton>
		{/if}
	</div>
{/snippet}

{#snippet toc()}
	<div class="space-y-1">
		<p class="mb-2 truncate font-medium text-gray-800">{Strings.TOC}</p>
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
