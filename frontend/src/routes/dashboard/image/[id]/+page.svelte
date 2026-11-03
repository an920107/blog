<script lang="ts">
	import { setContext } from 'svelte';

	import { ImageLoadedStore } from '$lib/image/adapter/presenter/store/imageLoadedStore';
	import { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
	import ImageDetailDashboardPage from '$lib/image/framework/ui/ImageDetailDashboardPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContainer();

	const getInitialData = () =>
		data.dehydratedData ? ImageInfoViewModel.rehydrate(data.dehydratedData) : undefined;

	const imageLoadedStore = container.createImageLoadedStore(getInitialData());
	setContext(ImageLoadedStore.name, imageLoadedStore);
</script>

<ImageDetailDashboardPage />
