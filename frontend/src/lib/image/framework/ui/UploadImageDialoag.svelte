<script lang="ts">
	import { buttonVariants } from '$lib/common/framework/components/ui/button';
	import Button from '$lib/common/framework/components/ui/button/button.svelte';
	import {
		Dialog,
		DialogContent,
		DialogFooter,
		DialogHeader,
		DialogTitle,
		DialogTrigger,
	} from '$lib/common/framework/components/ui/dialog';
	import Input from '$lib/common/framework/components/ui/input/input.svelte';
	import Label from '$lib/common/framework/components/ui/label/label.svelte';

	const {
		disabled,
		onSubmit: uploadImage,
	}: {
		disabled: boolean;
		onSubmit: (file: File) => Promise<void>;
	} = $props();

	const imageMimeTypes = ['image/jpeg', 'image/png', 'image/gif', 'image/webp'];

	let open = $state(false);
	let files: FileList | undefined = $state(undefined);
	let fileInputErrorMessage: string | null = $state(null);
	const isFileInputError = $derived(fileInputErrorMessage !== null);

	async function onSubmit(event: SubmitEvent) {
		event.preventDefault();
		fileInputErrorMessage = null;

		const file = files?.[0];
		if (!file || !imageMimeTypes.includes(file.type)) {
			fileInputErrorMessage = 'Please select an valid image file.';
			return;
		}

		await uploadImage(file);
		files = undefined;
		fileInputErrorMessage = null;
		open = false;
	}
</script>

<Dialog bind:open>
	<DialogTrigger class={buttonVariants({ variant: 'default' })}>Upload</DialogTrigger>
	<DialogContent
		showCloseButton={false}
		onInteractOutside={(e) => e.preventDefault()}
		onEscapeKeydown={(e) => e.preventDefault()}
	>
		<DialogHeader class="mb-4">
			<DialogTitle>Upload Image</DialogTitle>
		</DialogHeader>

		<form id="upload-image-form" onsubmit={onSubmit}>
			<Label for="file-input" class="pb-2">
				{`Image File (${imageMimeTypes.join(', ')})`}
			</Label>
			<Input
				id="file-input"
				type="file"
				accept={imageMimeTypes.join(',')}
				aria-invalid={isFileInputError}
				class="cursor-pointer"
				bind:files
				{disabled}
			/>
			{#if isFileInputError}
				<p class="text-sm text-red-500">{fileInputErrorMessage}</p>
			{/if}
		</form>

		<DialogFooter class="mt-6">
			<Button variant="outline" onclick={() => (open = false)} {disabled}>Cancel</Button>
			<Button type="submit" form="upload-image-form" {disabled}>Submit</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
