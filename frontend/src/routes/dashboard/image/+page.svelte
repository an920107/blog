<script lang="ts">
	import { Container } from '$lib/container';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
	import ImageOverallDashboardPage from '$lib/image/framework/ui/ImageOverallDashboardPage.svelte';
	import { ImagesListedStore } from '$lib/image/adapter/presenter/imagesListedStore';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
	import { ImageDeletedStore } from '$lib/image/adapter/presenter/imageDeletedStore';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const imageUploadedStore = container.createImageUploadedStore();
	setContext(ImageUploadedStore.name, imageUploadedStore);

	const imageDeletedStore = container.createImageDeletedStore();
	setContext(ImageDeletedStore.name, imageDeletedStore);

	const getInitialData = () =>
		data.dehydratedData?.map((image) => ImageInfoViewModel.rehydrate(image));
	const imagesListedStore = container.createImagesListedStore(getInitialData());
	setContext(ImagesListedStore.name, imagesListedStore);
</script>

<ImageOverallDashboardPage />
