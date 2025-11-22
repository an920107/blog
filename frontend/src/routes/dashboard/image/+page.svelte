<script lang="ts">
	import { Container } from '$lib/container';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
	import ImageOverallDashboardPage from '$lib/image/framework/ui/ImageOverallDashboardPage.svelte';
	import { ImagesListedStore } from '$lib/image/adapter/presenter/imagesListedStore';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const store = container.createImageUploadedStore();
	setContext(ImageUploadedStore.name, store);

	const initialData = data.dehydratedData?.map((image) => ImageInfoViewModel.rehydrate(image));
	const imagesListedStore = container.createImagesListedStore(initialData);
	setContext(ImagesListedStore.name, imagesListedStore);
</script>

<ImageOverallDashboardPage />
