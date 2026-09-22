<script lang="ts">
	import type { Snippet } from 'svelte';

	import { cn } from '$lib/common/framework/components/utils';

	const {
		url,
		label,
		onclick,
		class: className,
		children,
	}: {
		url?: URL;
		label: string;
		onclick?: () => void;
		class?: string;
		children: Snippet;
	} = $props();
</script>

<div
	class={cn(
		'size-11 rounded-full border border-gray-200 bg-white',
		'hover:border-gray-300 hover:bg-gray-100',
		className
	)}
>
	{#if url}
		<a
			href={url.href}
			rel="external"
			target="_blank"
			aria-label={label}
			class="flex size-full cursor-pointer items-center justify-center text-gray-600 no-underline"
		>
			{@render children()}
		</a>
	{:else}
		<button
			type="button"
			aria-label={label}
			class="flex size-full cursor-pointer items-center justify-center text-gray-600"
			{onclick}
		>
			{@render children()}
		</button>
	{/if}
</div>
