<script lang="ts">
	import { getContext } from 'svelte';
	import { ImageLoadedStore } from '$lib/image/adapter/presenter/imageLoadedStore';
	import Table from '$lib/common/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/common/framework/components/ui/table/table-body.svelte';
	import TableRow from '$lib/common/framework/components/ui/table/table-row.svelte';
	import TableCell from '$lib/common/framework/components/ui/table/table-cell.svelte';
	import TableHead from '$lib/common/framework/components/ui/table/table-head.svelte';
	import { copyToClipboard } from '$lib/common/framework/ui/copyToClipboard';

	const imageLoadedStore = getContext<ImageLoadedStore>(ImageLoadedStore.name);
	const imageState = $derived($imageLoadedStore);
	const image = $derived(imageState.data);
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between py-16">
		<h1 class="text-5xl font-bold text-gray-800">Image Details</h1>
		<a
			href="/dashboard/image"
			class="rounded-lg bg-gray-100 px-4 py-2 text-gray-600 transition-colors hover:bg-gray-200"
		>
			Back to Gallery
		</a>
	</div>

	{#if imageState.isSuccess() && image}
		<Table>
			<TableBody>
				<TableRow>
					<TableHead>ID</TableHead>
					<TableCell>{image.id}</TableCell>
				</TableRow>
				<TableRow>
					<TableHead>MIME Type</TableHead>
					<TableCell>
						<span>{image.mimeType}</span>
					</TableCell>
				</TableRow>
				<TableRow>
					<TableHead>URL</TableHead>
					<TableCell>
						<div class="flex items-center gap-2">
							<span class="text-sm break-all">{image.url.href}</span>
							<button
								title="Copy URL"
								onclick={() =>
									copyToClipboard(image.url.href, {
										successMessage: 'The URL is copied to clipboard',
									})}
							>
								<i class="fa-regular fa-clone"></i>
							</button>
						</div>
					</TableCell>
				</TableRow>
			</TableBody>
		</Table>

		<div class="mt-16">
			<div class="overflow-hidden bg-gray-100">
				<img src={image.url.href} alt="Image {image.id}" class="w-full object-contain" />
			</div>
		</div>
	{:else if imageState.isLoading()}
		<div class="flex h-64 items-center justify-center">
			<p class="text-gray-500">Loading image details...</p>
		</div>
	{:else if imageState.isError()}
		<div class="rounded-xl border border-red-200 bg-red-50 p-6 text-red-700">
			<h3 class="font-bold">Error</h3>
			<p>{imageState.error?.message}</p>
		</div>
	{/if}
</div>
