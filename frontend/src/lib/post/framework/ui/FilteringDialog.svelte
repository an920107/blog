<script module lang="ts">
	import z from 'zod';

	export const formSchema = z.object({
		keyword: z.string().trim().optional(),
		labelId: z.number().optional(),
	});

	type FormParams = z.infer<typeof formSchema>;
	export type FilteringDialogFormParams = FormParams;
</script>

<script lang="ts">
	import { getContext, onMount, tick } from 'svelte';
	import { MediaQuery } from 'svelte/reactivity';

	import { Button, buttonVariants } from '$lib/common/framework/components/ui/button';
	import {
		Command,
		CommandGroup,
		CommandInput,
		CommandItem,
		CommandList,
	} from '$lib/common/framework/components/ui/command';
	import {
		Dialog,
		DialogContent,
		DialogFooter,
		DialogHeader,
		DialogTitle,
	} from '$lib/common/framework/components/ui/dialog';
	import {
		Drawer,
		DrawerContent,
		DrawerFooter,
		DrawerHeader,
		DrawerTitle,
	} from '$lib/common/framework/components/ui/drawer';
	import Input from '$lib/common/framework/components/ui/input/input.svelte';
	import Label from '$lib/common/framework/components/ui/label/label.svelte';
	import {
		Popover,
		PopoverContent,
		PopoverTrigger,
	} from '$lib/common/framework/components/ui/popover';
	import Tooltip from '$lib/common/framework/components/ui/tooltip/tooltip.svelte';
	import TooltipContent from '$lib/common/framework/components/ui/tooltip/tooltip-content.svelte';
	import TooltipTrigger from '$lib/common/framework/components/ui/tooltip/tooltip-trigger.svelte';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { Strings } from '$lib/strings';

	let {
		defaultValues = {},
		onSubmit,
		open = $bindable(false),
	}: {
		defaultValues: FormParams;
		onSubmit: (params: FormParams) => void;
		open?: boolean;
	} = $props();

	const isDesktop = new MediaQuery('(min-width: 768px)');

	let labelPopoverOpen = $state(false);
	let keywordInputElement: HTMLInputElement | null = $state(null);

	let formData: FormParams = $state((() => defaultValues)());
	let formErrors: Partial<Record<keyof FormParams, string>> = $state({});

	$effect(() => {
		if (open) {
			formData = { ...defaultValues };
		}
	});

	const labelsListedStore = getContext<LabelsListedStore>(LabelsListedStore.name);
	const labelsListedState = $derived($labelsListedStore);
	const { trigger: listLabels } = labelsListedStore;

	function handleLabelSelect(id: number) {
		if (formData.labelId === id) {
			formData.labelId = undefined;
		} else {
			formData.labelId = id;
		}
		labelPopoverOpen = false;
	}

	function handleSubmit(event?: SubmitEvent) {
		event?.preventDefault();

		formErrors = {};
		const parseResult = formSchema.safeParse(formData);
		if (!parseResult.success) {
			const errorResult = z.treeifyError(parseResult.error).properties;
			formErrors.keyword = errorResult?.keyword?.errors[0];
			formErrors.labelId = errorResult?.labelId?.errors[0];
			return;
		}

		onSubmit(formData);
		open = false;
	}

	onMount(() => listLabels());

	$effect(() => {
		if (open) {
			// Focus the keyword input once the content has rendered.
			tick().then(() => keywordInputElement?.focus());
		}
	});
</script>

{#if isDesktop.current}
	<Dialog bind:open>
		<DialogContent
			showCloseButton={false}
			onOpenAutoFocus={(e) => e.preventDefault()}
			onCloseAutoFocus={(e) => e.preventDefault()}
		>
			<DialogHeader class="mb-4">
				<DialogTitle>{Strings.SEARCH_AND_FILTER_TITLE}</DialogTitle>
			</DialogHeader>

			<form id="filtering-form" onsubmit={handleSubmit} class="space-y-3">
				{@render keywordInput()}
				{@render labelIdInput()}
			</form>

			<DialogFooter class="mt-6">
				{@render formButtons()}
			</DialogFooter>
		</DialogContent>
	</Dialog>
{:else}
	<Drawer bind:open>
		<DrawerContent>
			<DrawerHeader>
				<DrawerTitle>{Strings.SEARCH_AND_FILTER_TITLE}</DrawerTitle>
			</DrawerHeader>

			<form id="filtering-form" onsubmit={handleSubmit} class="space-y-3 px-4">
				{@render keywordInput()}
				{@render labelIdInput()}
			</form>

			<DrawerFooter class="flex-row gap-2">
				{@render formButtons()}
			</DrawerFooter>
		</DrawerContent>
	</Drawer>
{/if}

{#snippet formButtons()}
	<Button
		type="button"
		variant="secondary"
		class="me-auto"
		onclick={() => {
			formData = { keyword: undefined, labelId: undefined };
			handleSubmit();
		}}
	>
		{Strings.CLEAR}
	</Button>
	<Button variant="outline" onclick={() => (open = false)}>{Strings.CANCEL}</Button>
	<Button type="submit" form="filtering-form">{Strings.CONFIRM}</Button>
{/snippet}

{#snippet keywordInput()}
	{@const id = 'keyword-input'}
	<div>
		<div class="flex flex-row items-center gap-1 pb-2">
			<Label for={id}>{Strings.KEYWORD}</Label>
			<Tooltip delayDuration={300} disableCloseOnTriggerClick={true}>
				<TooltipTrigger class="inline-flex cursor-help! items-center">
					<i class="fa-solid fa-circle-info pt-0.5 text-[0.75rem] text-gray-500"></i>
				</TooltipTrigger>
				<TooltipContent>{Strings.SEARCH_POST_HINT}</TooltipContent>
			</Tooltip>
		</div>
		<div class="flex flex-row items-center gap-x-2">
			<Input
				{id}
				type="text"
				class="px-4 py-2 text-base! placeholder:text-gray-500"
				placeholder={Strings.SEARCH_POST_PLACEHOLDER}
				aria-invalid={formErrors.keyword !== undefined}
				bind:value={formData.keyword}
				bind:ref={keywordInputElement}
			/>
			{#if formData.keyword}
				<Button variant="ghost" size="icon" onclick={() => (formData.keyword = undefined)}>
					<i class="fa-solid fa-xmark"></i>
				</Button>
			{/if}
		</div>
	</div>
{/snippet}

{#snippet labelIdInput()}
	{@const id = 'label-id-input'}
	<div>
		<Label for={id} class="pb-2">{Strings.LABEL}</Label>
		<div class="flex flex-row items-center gap-x-2">
			<Popover bind:open={labelPopoverOpen}>
				<PopoverTrigger
					class={buttonVariants({
						variant: 'outline',
						class: 'flex-1 justify-start text-left font-normal',
					})}
					aria-invalid={formErrors.labelId !== undefined}
				>
					{#if formData.labelId}
						{@const label = labelsListedState?.data?.find((l) => l.id === formData.labelId)}
						{#if label}
							<PostLabel {label} />
						{:else}
							<span class="text-base text-gray-500">{Strings.LABEL_SELECT_PLACEHOLDER}</span>
						{/if}
					{:else}
						<span class="text-base text-gray-500">{Strings.LABEL_SELECT_PLACEHOLDER}</span>
					{/if}
				</PopoverTrigger>
				<PopoverContent class="p-0" align="start">
					<Command>
						<CommandInput class="text-base" placeholder={Strings.SEARCH_LABEL_PLACEHOLDER} />
						<CommandList>
							<CommandGroup>
								{#each labelsListedState?.data ?? [] as label (label.id)}
									<CommandItem
										class="cursor-pointer"
										value={label.name}
										onclick={() => handleLabelSelect(label.id)}
									>
										<div class="flex items-center gap-2">
											<PostLabel {label} />
											{#if formData.labelId === label.id}
												<i class="fa-solid fa-check ml-auto"></i>
											{/if}
										</div>
									</CommandItem>
								{/each}
							</CommandGroup>
						</CommandList>
					</Command>
				</PopoverContent>
			</Popover>
			{#if formData.labelId}
				<Button variant="ghost" size="icon" onclick={() => (formData.labelId = undefined)}>
					<i class="fa-solid fa-xmark"></i>
				</Button>
			{/if}
		</div>
	</div>
{/snippet}
