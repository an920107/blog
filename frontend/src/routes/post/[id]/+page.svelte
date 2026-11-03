<script lang="ts">
	import { setContext } from 'svelte';

	import { PostLoadedStore } from '$lib/post/adapter/presenter/store/postLoadedStore';
	import { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
	import PostContentPage from '$lib/post/framework/ui/PostContentPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const id = $derived(data.id);
	const container = getContainer();

	const getInitialData = () => PostViewModel.rehydrate(data.dehydratedData);
	const store = container.createPostLoadedStore(getInitialData());
	setContext(PostLoadedStore.name, store);
</script>

<PostContentPage {id} />
