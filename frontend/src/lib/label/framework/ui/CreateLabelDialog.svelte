<script module lang="ts">
	import z from 'zod';

	const formSchema = z.object({
		name: z.string().trim().nonempty(),
		color: z.string().regex(/^#?[a-f\d]{6}$/i),
	});

	type FormParams = z.infer<typeof formSchema>;
	export type CreateLabelDialogFormParams = FormParams;
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
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import { Label as LabelEntity } from '$lib/label/domain/entity/label';
	import { ColorViewModel } from '$lib/label/adapter/presenter/colorViewModel';

	const {
		disabled,
		onSubmit: createLabel,
	}: {
		disabled: boolean;
		onSubmit: (params: FormParams) => Promise<boolean>;
	} = $props();

	let open = $state(false);

	let formData = $state<FormParams>({
		name: '',
		color: '#dddddd',
	});
	let formErrors = $state<Partial<Record<keyof FormParams, string>>>({});

	const previewLabel = $derived(
		LabelViewModel.fromEntity(
			new LabelEntity({
				id: -1,
				name: formData.name || 'Preview',
				color: ColorViewModel.fromHex(formData.color).toEntity(),
			})
		)
	);

	async function onSubmit(event: SubmitEvent) {
		event.preventDefault();
		formErrors = {};

		const parseResult = formSchema.safeParse(formData);
		if (parseResult.error) {
			const errorResult = z.treeifyError(parseResult.error).properties;
			formErrors.name = errorResult?.name?.errors[0];
			formErrors.color = errorResult?.color?.errors[0];
			return;
		}

		const isSuccess = await createLabel(formData);
		if (!isSuccess) {
			return;
		}

		formData = {
			name: '',
			color: '#dddddd',
		};
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
			<DialogTitle>Create Label</DialogTitle>
		</DialogHeader>

		<form id="create-label-form" onsubmit={onSubmit} class="space-y-3">
			<div>
				<Label for="name-input" class="pb-2">Name</Label>
				<Input
					id="name-input"
					type="text"
					aria-invalid={formErrors.name !== undefined}
					bind:value={formData.name}
				/>
				{#if formErrors.name}
					<p class="text-sm text-red-500">{formErrors.name}</p>
				{/if}
			</div>

			<div>
				<Label for="color-input" class="pb-2">Color</Label>
				<Input
					id="color-input"
					type="color"
					class="w-16"
					aria-invalid={formErrors.color !== undefined}
					bind:value={formData.color}
				/>
				{#if formErrors.color}
					<p class="text-sm text-red-500">{formErrors.color}</p>
				{/if}
			</div>
		</form>

		<DialogFooter class="mt-6 flex flex-row items-center">
			<div class="me-auto">
				<PostLabel label={previewLabel} />
			</div>
			<Button variant="outline" onclick={() => (open = false)} {disabled}>Cancel</Button>
			<Button type="submit" form="create-label-form" {disabled}>Submit</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
