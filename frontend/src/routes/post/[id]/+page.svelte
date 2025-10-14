<script lang="ts">
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import PostContentPage from '$lib/post/framework/ui/PostContentPage.svelte';
	import { Container } from '$lib/container';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';

	const { data, params }: PageProps = $props();
	const { id } = params;
	const container = getContext<Container>(Container.name);

	const initialData = PostViewModel.rehydrate(data.dehydratedData!);
	const store = container.createPostLoadedStore(initialData);
	setContext(PostLoadedStore.name, store);
</script>

<PostContentPage {id} />
