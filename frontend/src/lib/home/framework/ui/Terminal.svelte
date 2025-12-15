<script lang="ts">
	import { cn } from '$lib/common/framework/components/utils';
	import { selfIntroductionLines } from '$lib/common/framework/ui/StructuredData.svelte';
	import TerminalLastLine from '$lib/home/framework/ui/TerminalLastLine.svelte';
	import TerminalNormalLine from '$lib/home/framework/ui/TerminalNormalLine.svelte';
	import { onDestroy, onMount } from 'svelte';

	const lines = selfIntroductionLines;

	let isReady: boolean = $state(false);
	let currentIndex: number = $state(0);

	let element: HTMLDivElement | null = null;
	let observer: IntersectionObserver | null = null;

	onMount(() => {
		if (!element) {
			return;
		}

		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						isReady = true;
						observer?.disconnect();
					}
				});
			},
			{ threshold: 1 }
		);

		observer.observe(element);
	});

	onDestroy(() => {
		observer?.disconnect();
		observer = null;
	});

	function onLastLineComplete() {
		if (currentIndex < lines.length - 1) {
			currentIndex++;
		}
	}
</script>

<div
	class="content-container flex flex-col items-center justify-center gap-y-2.5 py-32 md:gap-y-8 md:px-24 md:py-32"
>
	<div
		bind:this={element}
		class={cn(
			'flex w-full flex-col gap-y-1.5 p-4 pb-28 md:gap-y-2.5 md:p-8 md:pb-32',
			'border-true-gray-800 bg-true-gray-700',
			'rounded-2xl border-4 shadow-lg md:rounded-3xl md:border-8 md:shadow-xl',
			'font-mono font-medium text-gray-50 md:text-lg',
			'transition-opacity duration-300',
			isReady ? 'opacity-100' : 'opacity-0'
		)}
	>
		{#each lines.slice(0, currentIndex) as line, index (index)}
			<TerminalNormalLine text={line} />
		{/each}

		{#if isReady}
			{#key currentIndex}
				<TerminalLastLine text={lines[currentIndex]} onComplete={onLastLineComplete} />
			{/key}
		{/if}
	</div>
</div>
