<script module lang="ts">
	import z from 'zod';

	const formSchema = z.object({
		semanticId: z
			.string()
			.max(100)
			.regex(/\D/)
			.regex(/^[a-zA-Z0-9_-]+$/),
		title: z.string().trim().nonempty().max(100),
	});

	type FormParams = z.infer<typeof formSchema>;
	export type CreatePostDialogFormParams = FormParams;
</script>

<script lang="ts">
	import { Button, buttonVariants } from '$lib/common/framework/components/ui/button';
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
		onSubmit: createPost,
	}: {
		disabled: boolean;
		onSubmit: (params: FormParams) => Promise<void>;
	} = $props();

	let open = $state(false);

	let formData = $state<FormParams>({ semanticId: '', title: '' });
	let formErrors = $state<Partial<Record<keyof FormParams, string>>>({});

	async function onSubmit(event: SubmitEvent) {
		event.preventDefault();
		formErrors = {};

		const parseResult = formSchema.safeParse(formData);
		if (parseResult.error) {
			const errorResult = z.treeifyError(parseResult.error).properties;
			if (errorResult?.semanticId?.errors) {
				formErrors.semanticId = errorResult.semanticId.errors[0];
			}
			if (errorResult?.title?.errors) {
				formErrors.title = errorResult.title.errors[0];
			}
			return;
		}

		await createPost(formData);
		formData = { semanticId: '', title: '' };
		open = false;
	}
</script>

<Dialog bind:open>
	<DialogTrigger class={buttonVariants({ variant: 'default' })}>Create</DialogTrigger>
	<DialogContent
		showCloseButton={false}
		onInteractOutside={(e) => e.preventDefault()}
		onEscapeKeydown={(e) => e.preventDefault()}
	>
		<DialogHeader class="mb-4">
			<DialogTitle>Create Post</DialogTitle>
		</DialogHeader>

		<form id="create-post-form" onsubmit={onSubmit} class="space-y-3">
			<div>
				<Label for="semantic-id-input" class="pb-2">Semantic ID</Label>
				<Input
					id="semantic-id-input"
					type="text"
					aria-invalid={formErrors.semanticId !== undefined}
					bind:value={formData.semanticId}
				/>
				{#if formErrors.semanticId}
					<p class="text-sm text-red-500">{formErrors.semanticId}</p>
				{/if}
			</div>

			<div>
				<Label for="title-input" class="pb-2">Title</Label>
				<Input
					id="title-input"
					type="text"
					aria-invalid={formErrors.title !== undefined}
					bind:value={formData.title}
				/>
				{#if formErrors.title}
					<p class="text-sm text-red-500">{formErrors.title}</p>
				{/if}
			</div>
		</form>

		<DialogFooter class="mt-6">
			<Button variant="outline" onclick={() => (open = false)} {disabled}>Cancel</Button>
			<Button type="submit" form="create-post-form" {disabled}>Submit</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
