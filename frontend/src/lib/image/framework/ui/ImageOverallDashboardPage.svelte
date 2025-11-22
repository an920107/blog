<script lang="ts">
	import { getContext } from 'svelte';
	import UploadImageDialoag from '$lib/image/framework/ui/UploadImageDialoag.svelte';
	import { toast } from 'svelte-sonner';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
	import { ImagesListedStore } from '$lib/image/adapter/presenter/imagesListedStore';

	const imageUploadedStore = getContext<ImageUploadedStore>(ImageUploadedStore.name);
	const imageUploadedState = $derived($imageUploadedStore);
	const { trigger: uploadImage } = imageUploadedStore;

	const imagesListedStore = getContext<ImagesListedStore>(ImagesListedStore.name);
	const imagesListedState = $derived($imagesListedStore);
	const { trigger: listImages } = imagesListedStore;

	async function onUploadImageDialogSubmit(file: File) {
		const state = await uploadImage(file);

		if (state.isSuccess()) {
			const imageInfo = state.data;

			let copiedToClipboard: boolean;
			try {
				await navigator.clipboard.writeText(imageInfo.url.href);
				copiedToClipboard = true;
			} catch {
				copiedToClipboard = false;
			}

			toast.success(`Image uploaded successfully with ID: ${imageInfo.id}`, {
				description: copiedToClipboard
					? 'The URL is copied to clipboard'
					: 'The URL is printed in console',
			});

			listImages();
		} else if (state.isError()) {
			toast.error('Failed to upload image', {
				description: state.error?.message ?? 'Unknown error',
			});
		}
	}
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">Image</h1>
		<UploadImageDialoag
			disabled={imageUploadedState.isLoading()}
			onSubmit={onUploadImageDialogSubmit}
		/>
	</div>

	<ul class="space-y-2">
		{#each imagesListedState.data ?? [] as image (image.id)}
			<li class="flex items-center gap-4 rounded-lg border p-4">
				<span class="font-mono text-sm">ID: {image.id}</span>
				<a href={image.url.href} target="_blank" class="truncate text-blue-500 hover:underline">
					{image.url.href}
				</a>
				<span class="text-sm text-gray-500">{image.mimeType}</span>
			</li>
		{/each}
	</ul>
</div>
