<script module lang="ts">
	import z from 'zod';

	const formSchema = z.object({
		name: z.string().trim().nonempty(),
		color: z.string().regex(/^#?[a-f\d]{6}$/i),
	});

	type FormParams = z.infer<typeof formSchema>;
	export type EditLabelDialogFormParams = FormParams;
</script>

<script lang="ts">
	import { Button, buttonVariants } from '$lib/common/framework/components/ui/button';
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
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import { Label as LabelEntity } from '$lib/label/domain/entity/label';
	import { ColorViewModel } from '$lib/label/adapter/presenter/colorViewModel';
	import RestoreButton from '$lib/common/framework/ui/RestoreButton.svelte';
	import InputError from '$lib/common/framework/ui/InputError.svelte';

	const {
		title,
		triggerButtonText,
		disabled,
		defaultValues = {
			name: '',
			color: '#dddddd',
		},
		onSubmit,
	}: {
		title: string;
		triggerButtonText: string;
		disabled: boolean;
		defaultValues?: FormParams;
		onSubmit: (params: FormParams) => Promise<boolean>;
	} = $props();

	let open = $state(false);

	let formData: FormParams = $state(defaultValues);
	let formErrors: Partial<Record<keyof FormParams, string>> = $state({});

	const previewLabel = $derived(
		LabelViewModel.fromEntity(
			new LabelEntity({
				id: -1,
				name: formData.name || 'Preview',
				color: ColorViewModel.fromHex(formData.color).toEntity(),
			})
		)
	);

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		formErrors = {};

		const parseResult = formSchema.safeParse(formData);
		if (parseResult.error) {
			const errorResult = z.treeifyError(parseResult.error).properties;
			formErrors.name = errorResult?.name?.errors[0];
			formErrors.color = errorResult?.color?.errors[0];
			return;
		}

		const isSuccess = await onSubmit(formData);
		if (!isSuccess) {
			return;
		}

		formData = defaultValues;
		open = false;
	}
</script>

<Dialog bind:open>
	<DialogTrigger class={buttonVariants({ variant: 'default' })} {disabled}>
		{triggerButtonText}
	</DialogTrigger>
	<DialogContent
		showCloseButton={false}
		onInteractOutside={(e) => e.preventDefault()}
		onEscapeKeydown={(e) => e.preventDefault()}
	>
		<DialogHeader class="mb-4">
			<DialogTitle>{title}</DialogTitle>
		</DialogHeader>

		<form id="create-label-form" onsubmit={handleSubmit} class="space-y-3">
			{@render nameInput()}
			{@render colorInput()}
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

{#snippet nameInput()}
	{@const id = 'name-input'}
	<div>
		<Label for={id} class="pb-2">Name</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Input
				{id}
				type="text"
				aria-invalid={formErrors.name !== undefined}
				bind:value={formData.name}
			/>
			<RestoreButton
				for={id}
				defaultValue={defaultValues.name}
				postAction={() => (formErrors.name = undefined)}
			/>
		</div>
		<InputError message={formErrors.name} />
	</div>
{/snippet}

{#snippet colorInput()}
	{@const id = 'color-input'}
	<div>
		<Label for={id} class="pb-2">Color</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Input
				{id}
				type="color"
				class="w-16"
				aria-invalid={formErrors.color !== undefined}
				bind:value={formData.color}
			/>
			<RestoreButton
				for={id}
				defaultValue={defaultValues.color}
				postAction={() => (formErrors.color = undefined)}
			/>
		</div>
		<InputError message={formErrors.color} />
	</div>
{/snippet}
