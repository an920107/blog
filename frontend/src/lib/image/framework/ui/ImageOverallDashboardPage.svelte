<script lang="ts">
	import { getContext } from 'svelte';
	import { toast } from 'svelte-sonner';

	import { ImageDeletedStore } from '$lib/image/adapter/presenter/store/imageDeletedStore';
	import { ImagesListedStore } from '$lib/image/adapter/presenter/store/imagesListedStore';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/store/imageUploadedStore';
	import UploadImageDialog from '$lib/image/framework/ui/UploadImageDialog.svelte';
	import { Button } from '$lib/shared/framework/components/ui/button';
	import { cn } from '$lib/shared/framework/components/utils';
	import { copyToClipboard } from '$lib/shared/framework/ui/copyToClipboard';
	import { Strings } from '$lib/strings';

	const imageUploadedStore = getContext<ImageUploadedStore>(ImageUploadedStore.name);
	const imageUploadedState = $derived($imageUploadedStore);
	const { trigger: uploadImage } = imageUploadedStore;

	const imagesListedStore = getContext<ImagesListedStore>(ImagesListedStore.name);
	const imagesState = $derived($imagesListedStore);
	const { trigger: listImages } = imagesListedStore;

	const imageDeletedStore = getContext<ImageDeletedStore>(ImageDeletedStore.name);
	const imageDeletedState = $derived($imageDeletedStore);
	const { trigger: deleteImage } = imageDeletedStore;

	async function onUploadImageDialogSubmit(file: File) {
		const state = await uploadImage(file);

		if (state.isSuccess()) {
			const imageInfo = state.data;
			await copyToClipboard(imageInfo.url.href, {
				successMessage: `${Strings.IMAGE_UPLOADED_PREFIX} ${imageInfo.id}`,
				successDescription: Strings.URL_COPIED,
			});
			listImages();
		} else if (state.isError()) {
			toast.error(Strings.FAILED_TO_UPLOAD_IMAGE, {
				description: state.error?.message ?? Strings.UNKNOWN_ERROR,
			});
		}
	}

	async function onDeleteImageButtonClick(imageId: number) {
		const state = await deleteImage(imageId);

		if (state.isSuccess()) {
			toast.success(Strings.IMAGE_DELETED);
			await listImages();
		} else if (state.isError()) {
			toast.error(Strings.FAILED_TO_DELETE_IMAGE, {
				description: state.error?.message ?? Strings.UNKNOWN_ERROR,
			});
		}
	}
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">{Strings.DASHBOARD_IMAGE}</h1>
		<UploadImageDialog
			disabled={imageUploadedState.isLoading()}
			onSubmit={onUploadImageDialogSubmit}
		/>
	</div>

	<div class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-5">
		{#each imagesState.data ?? [] as image (image.id)}
			<div class="group relative aspect-square overflow-hidden rounded-lg bg-gray-100">
				<img
					src={image.url.href}
					alt="{Strings.IMAGE} {image.id}"
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
							title={Strings.COPY_URL}
							type="button"
							variant="secondary"
							size="icon"
							onclick={() =>
								copyToClipboard(image.url.href, {
									successMessage: Strings.URL_COPIED,
								})}
						>
							<i class="fa-regular fa-clone"></i>
						</Button>
						<Button
							title={Strings.VIEW_DETAILS}
							type="button"
							href="/dashboard/image/{image.id}"
							variant="secondary"
							size="icon"
						>
							<i class="fa-regular fa-eye"></i>
						</Button>
						<Button
							title={image.isReferred ? Strings.IMAGE_IN_USE : Strings.DELETE_IMAGE}
							type="button"
							variant="destructive"
							size="icon"
							disabled={image.isReferred || imageDeletedState.isLoading()}
							onclick={() => onDeleteImageButtonClick(image.id)}
						>
							<i class="fa-solid fa-trash-can"></i>
						</Button>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
