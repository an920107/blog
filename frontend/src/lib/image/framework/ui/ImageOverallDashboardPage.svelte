<script lang="ts">
	import { getContext } from 'svelte';
	import UploadImageDialoag from '$lib/image/framework/ui/UploadImageDialoag.svelte';
	import { toast } from 'svelte-sonner';
	import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';

	const store = getContext<ImageUploadedStore>(ImageUploadedStore.name);
	const state = $derived($store);
	const { trigger: uploadImage } = store;

	async function onUploadImageDialogSubmit(file: File) {
		const state = await uploadImage(file);

		if (state.isSuccess()) {
			const imageInfo = state.data;
			console.log('Image URL', imageInfo.url.href);

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
		<UploadImageDialoag disabled={state.isLoading()} onSubmit={onUploadImageDialogSubmit} />
	</div>
	<p>Gallery is currently unavailable.</p>
</div>
