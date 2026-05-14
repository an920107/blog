<script lang="ts">
	import { getContext, setContext } from 'svelte';

	import { Container } from '$lib/container';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
	import PostContentPage from '$lib/post/framework/ui/PostContentPage.svelte';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const id = $derived(data.id);
	const container = getContext<Container>(Container.name);

	const getInitialData = () => PostViewModel.rehydrate(data.dehydratedData);
	const store = container.createPostLoadedStore(getInitialData());
	setContext(PostLoadedStore.name, store);
</script>

<PostContentPage {id} />
