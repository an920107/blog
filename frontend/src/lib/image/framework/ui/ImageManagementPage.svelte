<script lang="ts">
	import { getContext } from 'svelte';
	import UploadImageDialoag from './UploadImageDialoag.svelte';
	import { ImageBloc, ImageEventType } from '$lib/image/adapter/presenter/imageBloc';
	import { StatusType } from '$lib/common/adapter/presenter/asyncState';
	import { toast } from 'svelte-sonner';

	const imageBloc = getContext<ImageBloc>(ImageBloc.name);
	const state = $derived($imageBloc);

	const isLoading = $derived(state.status === StatusType.Loading);

	async function onUploadImageDialogSubmit(file: File) {
		const state = await imageBloc.dispatch({ event: ImageEventType.ImageUploadedEvent, file });

		if (state.status === StatusType.Success) {
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
		} else if (state.status === StatusType.Error) {
			toast.error('Failed to upload image', {
				description: state.error?.message ?? 'Unknown error',
			});
		}
	}
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">Image</h1>
		<UploadImageDialoag disabled={isLoading} onSubmit={onUploadImageDialogSubmit} />
	</div>
	<p>Gallery is currently unavailable.</p>
</div>
