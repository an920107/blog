<script lang="ts">
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import PostContentPage from '$lib/post/framework/ui/PostContentPage.svelte';
	import { Container } from '$lib/container';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import StructuredData from '$lib/post/framework/ui/StructuredData.svelte';
	import OpenGraph from '$lib/post/framework/ui/OpenGraph.svelte';
	import { Environment } from '$lib/environment';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const initialData = PostViewModel.rehydrate(data.dehydratedData);
	const store = container.createPostLoadedStore(initialData);
	setContext(PostLoadedStore.name, store);

	const postInfo = $derived(initialData.info);
</script>

<svelte:head>
	<title>{generateTitle(postInfo?.title)}</title>
	{#if postInfo}
		<meta name="description" content={postInfo.description} />
	{/if}

	{#if postInfo?.isPublished}
		<StructuredData
			headline={postInfo.title}
			description={postInfo.description}
			datePublished={postInfo.publishedTime!}
			image={postInfo.previewImageUrl}
		/>
		<OpenGraph
			title={postInfo.title}
			description={postInfo.description}
			publishedTime={postInfo.publishedTime!}
			labels={postInfo.labels.map((label) => label.name)}
			url={new URL(`post/${postInfo.semanticId}`, Environment.APP_BASE_URL)}
			image={postInfo.previewImageUrl}
		/>
	{/if}
</svelte:head>

<PostContentPage />
