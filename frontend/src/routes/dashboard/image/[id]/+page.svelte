<script lang="ts">
	import { Container } from '$lib/container';
	import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
	import { ImageLoadedStore } from '$lib/image/adapter/presenter/imageLoadedStore';
	import ImageDetailDashboardPage from '$lib/image/framework/ui/ImageDetailDashboardPage.svelte';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const initialData = data.dehydratedData
		? ImageInfoViewModel.rehydrate(data.dehydratedData)
		: undefined;

	const imageLoadedStore = container.createImageLoadedStore(initialData);
	setContext(ImageLoadedStore.name, imageLoadedStore);
</script>

<ImageDetailDashboardPage />
