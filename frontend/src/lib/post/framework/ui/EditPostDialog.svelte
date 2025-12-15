<script module lang="ts">
	import z from 'zod';

	const formSchema = z.object({
		semanticId: z
			.string()
			.max(100)
			.regex(/\D/)
			.regex(/^[\da-z_-]+$/i),
		title: z.string().trim().nonempty(),
		description: z
			.string()
			.trim()
			.transform((s) => s.replaceAll('\n', '')),
		content: z.string(),
		labelIds: z.array(z.number()),
		previewImageUrl: z
			.url()
			.nullable()
			.transform((s) => (s ? new URL(s) : null)),
		publishedTime: z
			.date()
			.nullable()
			.transform((d) => (d ? new EnhancedDate(d) : null)),
	});

	type FormParams = z.infer<typeof formSchema>;
	export type EditPostDialogFormParams = FormParams;
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
	import RestoreButton from '$lib/common/framework/ui/RestoreButton.svelte';
	import InputError from '$lib/common/framework/ui/InputError.svelte';
	import Textarea from '$lib/common/framework/components/ui/textarea/textarea.svelte';
	import InputEnabledToggle from '$lib/common/framework/ui/InputEnabledToggle.svelte';
	import { EnhancedDate } from '$lib/common/adapter/presenter/enhancedDate';
	import { getContext, onMount, tick } from 'svelte';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import {
		Popover,
		PopoverContent,
		PopoverTrigger,
	} from '$lib/common/framework/components/ui/popover';
	import {
		Command,
		CommandGroup,
		CommandInput,
		CommandItem,
		CommandList,
	} from '$lib/common/framework/components/ui/command';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';

	const {
		mode,
		title = mode === 'create' ? 'Create Post' : 'Update Post',
		triggerButtonText,
		disabled,
		defaultValues = {
			semanticId: '',
			title: '',
			description: '',
			content: '',
			labelIds: [],
			previewImageUrl: null,
			publishedTime: null,
		},
		onSubmit,
	}: {
		mode: 'create' | 'update';
		title?: string;
		triggerButtonText: string;
		disabled: boolean;
		defaultValues?: FormParams;
		onSubmit: (params: FormParams) => Promise<boolean>;
	} = $props();

	let open = $state(false);

	let formData: FormParams = $state(defaultValues);
	let formErrors: Partial<Record<keyof FormParams, string>> = $state({});

	let previewImageUrl = $state(defaultValues.previewImageUrl?.href ?? '');
	let publishedTime = $state(defaultValues.publishedTime?.toDateTimeInputValue() ?? '');

	const labelsListedStore =
		getContext<LabelsListedStore | undefined>(LabelsListedStore.name) ?? null;
	const labelsListedState = $derived(labelsListedStore ? $labelsListedStore : null);
	const listLabels = labelsListedStore?.trigger ?? null;

	let labelIdsInputElement: HTMLInputElement | null = null;
	async function labelOperation(method: 'remove' | 'add' | 'previous' | 'next', labelId: number) {
		switch (method) {
			case 'remove':
				formData.labelIds = formData.labelIds.filter((id) => id !== labelId);
				break;
			case 'add':
				if (!formData.labelIds.includes(labelId)) {
					formData.labelIds = [...formData.labelIds, labelId];
				}
				break;
			case 'previous': {
				const index = formData.labelIds.indexOf(labelId);
				if (index > 0) {
					const newLabelIds = [...formData.labelIds];
					[newLabelIds[index - 1], newLabelIds[index]] = [
						newLabelIds[index],
						newLabelIds[index - 1],
					];
					formData.labelIds = newLabelIds;
				}
				break;
			}
			case 'next': {
				const index = formData.labelIds.indexOf(labelId);
				if (index >= 0 && index < formData.labelIds.length - 1) {
					const newLabelIds = [...formData.labelIds];
					[newLabelIds[index + 1], newLabelIds[index]] = [
						newLabelIds[index],
						newLabelIds[index + 1],
					];
					formData.labelIds = newLabelIds;
				}
				break;
			}
		}
		await tick();
		labelIdsInputElement?.dispatchEvent(new Event('input'));
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		formErrors = {};

		const parseResult = formSchema.safeParse({
			...formData,
			previewImageUrl: previewImageUrl || null,
			publishedTime: publishedTime ? new Date(publishedTime) : null,
		});

		if (parseResult.error) {
			const errorResult = z.treeifyError(parseResult.error).properties;
			formErrors.semanticId = errorResult?.semanticId?.errors[0];
			formErrors.title = errorResult?.title?.errors[0];
			formErrors.description = errorResult?.description?.errors[0];
			formErrors.content = errorResult?.content?.errors[0];
			formErrors.labelIds = errorResult?.labelIds?.errors[0];
			formErrors.previewImageUrl = errorResult?.previewImageUrl?.errors[0];
			formErrors.publishedTime = errorResult?.publishedTime?.errors[0];
			return;
		}

		const isSuccess = await onSubmit(parseResult.data);
		if (!isSuccess) {
			return;
		}

		formData = defaultValues;
		open = false;
	}

	onMount(() => listLabels?.());
</script>

<Dialog bind:open>
	<DialogTrigger class={buttonVariants({ variant: 'default' })}>{triggerButtonText}</DialogTrigger>
	<DialogContent
		showCloseButton={false}
		onInteractOutside={(e) => e.preventDefault()}
		onEscapeKeydown={(e) => e.preventDefault()}
	>
		<DialogHeader class="mb-4">
			<DialogTitle>{title}</DialogTitle>
		</DialogHeader>

		<form id="create-post-form" onsubmit={handleSubmit} class="space-y-3">
			{#if mode === 'create'}
				{@render semanticIdInput()}
			{/if}
			{@render titleInput()}
			{#if mode === 'update'}
				{@render labelIdsInput()}
				{@render descriptionInput()}
				{@render contentInput()}
				{@render previewImageUrlInput()}
				{@render publishedTimeInput()}
			{/if}
		</form>

		<DialogFooter class="mt-6">
			<Button variant="outline" onclick={() => (open = false)} {disabled}>Cancel</Button>
			<Button type="submit" form="create-post-form" {disabled}>Submit</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>

{#snippet semanticIdInput()}
	{@const id = 'semantic-id-input'}
	<div>
		<Label for={id} class="pb-2">Semantic ID</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Input
				{id}
				type="text"
				aria-invalid={formErrors.semanticId !== undefined}
				bind:value={formData.semanticId}
			/>
			<RestoreButton
				for={id}
				defaultValue={defaultValues.semanticId}
				postAction={() => (formErrors.semanticId = undefined)}
			/>
		</div>
		<InputError message={formErrors.semanticId} />
	</div>
{/snippet}

{#snippet titleInput()}
	{@const id = 'title-input'}
	<div>
		<Label for={id} class="pb-2">Title</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Input
				{id}
				type="text"
				aria-invalid={formErrors.title !== undefined}
				bind:value={formData.title}
			/>
			<RestoreButton
				for={id}
				defaultValue={defaultValues.title}
				postAction={() => (formErrors.title = undefined)}
			/>
		</div>
		<InputError message={formErrors.title} />
	</div>
{/snippet}

{#snippet labelIdsInput()}
	{@const id = 'label-ids-input'}
	<div>
		<Label for={id} class="pb-2">Label IDs</Label>
		<div class="flex flex-row items-center gap-x-2">
			<input
				{id}
				bind:this={labelIdsInputElement}
				hidden
				readonly
				type="text"
				aria-invalid={formErrors.labelIds !== undefined}
				value={JSON.stringify(formData.labelIds)}
				oninput={(e) => (formData.labelIds = JSON.parse((e.target as HTMLInputElement).value))}
			/>
			{@render labelSearchPopover()}
			<div class="flex w-full flex-row flex-wrap items-center gap-2">
				{#each formData.labelIds as labelId (labelId)}
					{@const label = labelsListedState?.data?.find((l) => l.id === labelId)}
					{#if label}
						{@render labelItem(label)}
					{/if}
				{/each}
			</div>
			<RestoreButton
				for={id}
				defaultValue={JSON.stringify(defaultValues.labelIds)}
				postAction={() => (formErrors.labelIds = undefined)}
			/>
		</div>
		<InputError message={formErrors.labelIds} />
	</div>
{/snippet}

{#snippet descriptionInput()}
	{@const id = 'description-input'}
	<div>
		<Label for={id} class="pb-2">Description</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Textarea
				{id}
				class="h-24 resize-none text-wrap wrap-anywhere"
				aria-invalid={formErrors.description !== undefined}
				bind:value={formData.description}
			/>
			<RestoreButton
				for={id}
				defaultValue={defaultValues.description}
				postAction={() => (formErrors.description = undefined)}
			/>
		</div>
		<InputError message={formErrors.description} />
	</div>
{/snippet}

{#snippet contentInput()}
	{@const id = 'content-input'}
	<div>
		<Label for={id} class="pb-2">Content</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Textarea
				{id}
				class="h-24 resize-none text-wrap wrap-anywhere"
				aria-invalid={formErrors.content !== undefined}
				bind:value={formData.content}
			/>
			<RestoreButton
				for={id}
				defaultValue={defaultValues.content}
				postAction={() => (formErrors.content = undefined)}
			/>
		</div>
		<InputError message={formErrors.content} />
	</div>
{/snippet}

{#snippet previewImageUrlInput()}
	{@const id = 'preview-image-url-input'}
	{@const defaultValue = defaultValues.previewImageUrl?.href ?? ''}
	<div>
		<Label for={id} class="pb-2">Preview Image URL (optional)</Label>
		<div class="flex flex-row items-center gap-x-2">
			<InputEnabledToggle for={id} {defaultValue} valueOnEnable="https://example.com" />
			<Input
				{id}
				type="text"
				aria-invalid={formErrors.previewImageUrl !== undefined}
				bind:value={previewImageUrl}
			/>
			<RestoreButton
				for={id}
				{defaultValue}
				postAction={() => (formErrors.previewImageUrl = undefined)}
			/>
		</div>
		<InputError message={formErrors.previewImageUrl} />
	</div>
{/snippet}

{#snippet publishedTimeInput()}
	{@const id = 'published-time-input'}
	{@const defaultValue = defaultValues.publishedTime?.toDateTimeInputValue() ?? ''}
	<div>
		<Label for={id} class="pb-2">Published Time (optional)</Label>
		<div class="flex flex-row items-center gap-x-2">
			<InputEnabledToggle
				for={id}
				{defaultValue}
				valueOnEnable={new EnhancedDate().toDateTimeInputValue()}
			/>
			<Input
				{id}
				type="datetime-local"
				class="w-fit"
				aria-invalid={formErrors.publishedTime !== undefined}
				bind:value={publishedTime}
			/>
			<RestoreButton
				for={id}
				{defaultValue}
				postAction={() => (formErrors.publishedTime = undefined)}
			/>
		</div>
		<InputError message={formErrors.publishedTime} />
	</div>
{/snippet}

{#snippet labelSearchPopover()}
	<Popover>
		<PopoverTrigger class={buttonVariants({ variant: 'secondary', size: 'icon-sm' })}>
			<i class="fa-solid fa-plus"></i>
		</PopoverTrigger>
		<PopoverContent class="max-w-52 p-0">
			<Command>
				<CommandInput />
				<CommandList>
					<CommandGroup>
						{#each labelsListedState?.data ?? [] as label (label.id)}
							{#if !formData.labelIds.includes(label.id)}
								<CommandItem
									class="cursor-pointer"
									value={label.name}
									onclick={() => labelOperation('add', label.id)}
								>
									<PostLabel {label} />
								</CommandItem>
							{/if}
						{/each}
					</CommandGroup>
				</CommandList>
			</Command>
		</PopoverContent>
	</Popover>
{/snippet}

{#snippet labelItem(label: LabelViewModel)}
	{@const style = `color: ${label.color.darken(0.6).hex};`}
	<div class="group flex flex-row items-center">
		<button
			type="button"
			title="Move previous"
			class="hidden group-hover:block"
			onclick={() => labelOperation('previous', label.id)}
		>
			<i class="fa-solid fa-chevron-left" {style}></i>
		</button>
		<PostLabel {label} />
		<button
			type="button"
			title="Move next"
			class="hidden group-hover:block"
			onclick={() => labelOperation('next', label.id)}
		>
			<i class="fa-solid fa-chevron-right" {style}></i>
		</button>
		<button
			type="button"
			title="Remove"
			class="hidden group-hover:block"
			onclick={() => labelOperation('remove', label.id)}
		>
			<i class="fa-solid fa-trash-can" {style}></i>
		</button>
	</div>
{/snippet}
