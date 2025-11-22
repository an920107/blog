<script lang="ts">
	import { getContext } from 'svelte';
	import UploadImageDialoag from '$lib/image/framework/ui/UploadImageDialoag.svelte';
	import { toast } from 'svelte-sonner';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
	import { ImagesListedStore } from '$lib/image/adapter/presenter/imagesListedStore';
	import { Button } from '$lib/common/framework/components/ui/button';
	import { cn } from '$lib/common/framework/components/utils';
	import { copyToClipboard } from '$lib/common/framework/ui/copyToClipboard';

	const imageUploadedStore = getContext<ImageUploadedStore>(ImageUploadedStore.name);
	const imageUploadedState = $derived($imageUploadedStore);
	const { trigger: uploadImage } = imageUploadedStore;

	const imagesListedStore = getContext<ImagesListedStore>(ImagesListedStore.name);
	const imagesState = $derived($imagesListedStore);
	const { trigger: listImages } = imagesListedStore;

	async function onUploadImageDialogSubmit(file: File) {
		const state = await uploadImage(file);

		if (state.isSuccess()) {
			const imageInfo = state.data;
			await copyToClipboard(imageInfo.url.href, {
				successMessage: `Image uploaded successfully with ID: ${imageInfo.id}`,
				successDescription: 'The URL is copied to clipboard',
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

	<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-5">
		{#each imagesState.data ?? [] as image (image.id)}
			<div class="group relative aspect-square overflow-hidden rounded-lg bg-gray-100">
				<img
					src={image.url.href}
					alt="Image {image.id}"
					class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-110"
					loading="lazy"
				/>
				<div
					class={cn(
						'absolute inset-0 flex flex-col items-center justify-center bg-black/50',
						'opacity-0 transition-opacity duration-300 group-hover:opacity-100'
					)}
				>
					<div class="mb-2 text-white">
						<span class="font-bold">ID:</span>
						{image.id}
					</div>
					<div class="mb-4 text-sm text-gray-200">
						{image.mimeType}
					</div>
					<div class="flex gap-2">
						<Button
							title="Copy URL"
							type="button"
							variant="secondary"
							size="icon"
							onclick={() =>
								copyToClipboard(image.url.href, {
									successMessage: 'The URL is copied to clipboard',
								})}
						>
							<i class="fa-regular fa-clone"></i>
						</Button>
						<Button
							title="View Details"
							type="button"
							href="/dashboard/image/{image.id}"
							variant="secondary"
							size="icon"
						>
							<i class="fa-regular fa-eye"></i>
						</Button>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
