<script lang="ts">
	import { Container } from '$lib/container';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
	import ImageOverallDashboardPage from '$lib/image/framework/ui/ImageOverallDashboardPage.svelte';
	import { getContext, setContext } from 'svelte';

	const container = getContext<Container>(Container.name);
	const store = container.createImageUploadedStore();
	setContext(ImageUploadedStore.name, store);
</script>

<ImageOverallDashboardPage />
