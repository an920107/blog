<script lang="ts">
	/* eslint-disable svelte/no-navigation-without-resolve */

	import { captureException } from '@sentry/sveltekit';
	import { toast } from 'svelte-sonner';

	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { cn } from '$lib/common/framework/components/utils';
	import type { FilteringDialogFormParams } from '$lib/post/framework/ui/FilteringDialog.svelte';
	import { parsePostListSearchParams } from '$lib/post/framework/ui/postListSearchParams';
	import { Strings } from '$lib/strings';

	// The filtering dialog is lazily imported so its code (dialog/drawer/command,
	// zod, …) stays out of the initial bundle of every page. It is prefetched on
	// pointer/focus intent so the first open still feels instant.
	let FilteringDialog:
		typeof import('$lib/post/framework/ui/FilteringDialog.svelte').default | null = $state(null);

	let open = $state(false);

	const defaultValues = $derived(parsePostListSearchParams(page.url.searchParams));

	const hasLabelFilter = $derived(defaultValues.labelId !== undefined);
	const hasActiveQuery = $derived(Boolean(defaultValues.keyword) || hasLabelFilter);

	async function loadDialog(): Promise<boolean> {
		if (FilteringDialog) {
			return true;
		}

		try {
			const module = await import('$lib/post/framework/ui/FilteringDialog.svelte');
			FilteringDialog = module.default;
			return true;
		} catch (error) {
			captureException(error);
			return false;
		}
	}

	async function openDialog() {
		// Only open once the dialog is actually available, so `open` never reports a
		// state the UI cannot render.
		if (await loadDialog()) {
			open = true;
		} else {
			toast.error(Strings.SEARCH_UNAVAILABLE);
		}
	}

	function handleSubmit(params: FilteringDialogFormParams) {
		const url = new URL(resolve('/post'), page.url);

		if (params.keyword) {
			url.searchParams.set('keyword', params.keyword);
		} else {
			url.searchParams.delete('keyword');
		}

		if (params.labelId !== undefined) {
			url.searchParams.set('label_id', params.labelId.toString());
		} else {
			url.searchParams.delete('label_id');
		}

		goto(url, { keepFocus: true });
	}
</script>

<button
	type="button"
	class={cn(
		'flex h-9 w-56 flex-row items-center gap-x-2 rounded-md px-3',
		'border border-input bg-background text-left text-sm text-muted-foreground shadow-xs',
		'max-md:hidden lg:w-64',
		'hover:bg-accent focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50'
	)}
	aria-label={Strings.SEARCH}
	aria-haspopup="dialog"
	aria-expanded={open}
	onpointerenter={loadDialog}
	onpointerdown={loadDialog}
	onfocus={loadDialog}
	onclick={openDialog}
>
	<i class="fa-solid fa-magnifying-glass shrink-0"></i>
	<span class={cn('min-w-0 flex-1 truncate', defaultValues.keyword ? 'text-foreground' : '')}>
		{defaultValues.keyword || Strings.SEARCH_PLACEHOLDER}
	</span>
	{#if hasLabelFilter}
		<i class="fa-solid fa-filter shrink-0 text-blue-600" aria-hidden="true"></i>
	{/if}
</button>

<button
	type="button"
	class="relative md:hidden"
	aria-label={Strings.SEARCH}
	aria-haspopup="dialog"
	aria-expanded={open}
	onpointerenter={loadDialog}
	onpointerdown={loadDialog}
	onfocus={loadDialog}
	onclick={openDialog}
>
	<i class="fa-solid fa-magnifying-glass size-4"></i>
	{#if hasActiveQuery}
		<span class="absolute -top-1 -right-1 size-2 rounded-full bg-blue-600"></span>
	{/if}
</button>

{#if FilteringDialog}
	<FilteringDialog bind:open {defaultValues} onSubmit={handleSubmit} />
{/if}
