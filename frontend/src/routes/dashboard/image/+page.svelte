<script lang="ts">
	import { setContext } from 'svelte';

	import { ImageDeletedStore } from '$lib/image/adapter/presenter/store/imageDeletedStore';
	import { ImagesListedStore } from '$lib/image/adapter/presenter/store/imagesListedStore';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/store/imageUploadedStore';
	import { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
	import ImageOverallDashboardPage from '$lib/image/framework/ui/ImageOverallDashboardPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContainer();

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
