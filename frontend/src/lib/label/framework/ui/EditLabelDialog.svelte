<script module lang="ts">
	import type { LabelFormValues } from '$lib/label/application/validation/labelFormSchema';

	export type EditLabelDialogFormParams = LabelFormValues;
</script>

<script lang="ts">
	import z from 'zod';

	import { ColorViewModel } from '$lib/label/adapter/presenter/viewModel/colorViewModel';
	import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
	import { labelFormSchema } from '$lib/label/application/validation/labelFormSchema';
	import { Label as LabelEntity } from '$lib/label/domain/entity/label';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { Button, buttonVariants } from '$lib/shared/framework/components/ui/button';
	import {
		Dialog,
		DialogContent,
		DialogFooter,
		DialogHeader,
		DialogTitle,
		DialogTrigger,
	} from '$lib/shared/framework/components/ui/dialog';
	import Input from '$lib/shared/framework/components/ui/input/input.svelte';
	import Label from '$lib/shared/framework/components/ui/label/label.svelte';
	import InputError from '$lib/shared/framework/ui/InputError.svelte';
	import RestoreButton from '$lib/shared/framework/ui/RestoreButton.svelte';
	import { Strings } from '$lib/strings';

	type FormParams = EditLabelDialogFormParams;

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

	let formData: FormParams = $state((() => defaultValues)());
	let formErrors: Partial<Record<keyof FormParams, string>> = $state({});

	const previewLabel = $derived(
		LabelViewModel.fromEntity(
			new LabelEntity({
				id: -1,
				name: formData.name || Strings.PREVIEW,
				color: ColorViewModel.fromHex(formData.color).toEntity(),
			})
		)
	);

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		formErrors = {};

		const parseResult = labelFormSchema.safeParse(formData);
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
			<Button variant="outline" onclick={() => (open = false)} {disabled}
				>{Strings.CANCEL_DIALOG}</Button
			>
			<Button type="submit" form="create-label-form" {disabled}>{Strings.SUBMIT}</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>

{#snippet nameInput()}
	{@const id = 'name-input'}
	<div>
		<Label for={id} class="pb-2">{Strings.NAME}</Label>
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
		<Label for={id} class="pb-2">{Strings.COLOR}</Label>
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
