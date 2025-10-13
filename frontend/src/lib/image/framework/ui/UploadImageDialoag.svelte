<script lang="ts">
	import { buttonVariants } from '$lib/common/framework/components/ui/button';
	import Button from '$lib/common/framework/components/ui/button/button.svelte';
	import { Dialog } from '$lib/common/framework/components/ui/dialog';
	import DialogContent from '$lib/common/framework/components/ui/dialog/dialog-content.svelte';
	import DialogFooter from '$lib/common/framework/components/ui/dialog/dialog-footer.svelte';
	import DialogHeader from '$lib/common/framework/components/ui/dialog/dialog-header.svelte';
	import DialogTitle from '$lib/common/framework/components/ui/dialog/dialog-title.svelte';
	import DialogTrigger from '$lib/common/framework/components/ui/dialog/dialog-trigger.svelte';
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
		close();
		files = undefined;
		fileInputErrorMessage = null;
	}

	function close() {
		open = false;
	}
</script>

<Dialog {open} onOpenChange={(val) => (open = val)}>
	<DialogTrigger class={buttonVariants({ variant: 'default' })}>Upload</DialogTrigger>
	<DialogContent
		showCloseButton={false}
		onInteractOutside={(e) => e.preventDefault()}
		onEscapeKeydown={(e) => e.preventDefault()}
	>
		<DialogHeader class="mb-4">
			<DialogTitle>Upload Image</DialogTitle>
		</DialogHeader>

		<form id="upload-form" onsubmit={onSubmit}>
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
			<Button variant="outline" onclick={close} {disabled}>Cancel</Button>
			<Button type="submit" form="upload-form" {disabled}>Submit</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
