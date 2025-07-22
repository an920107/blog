<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	const { text, direction }: { text: string; direction: 'left' | 'right' } = $props();

	let isReady: boolean = $state(false);
	let origin = $derived(direction === 'left' ? 'origin-left' : 'origin-right');

	let element: HTMLSpanElement | null = null;
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

	onDestroy(() => observer?.disconnect());
</script>

<span
	bind:this={element}
	class="rounded-md bg-blue-600 px-1 py-0.5 text-white transition-transform delay-500 duration-1000 md:rounded-lg md:px-2.5 md:py-2 {origin} {isReady
		? 'scale-x-100'
		: 'scale-x-0'}"
>
	<span class="scale-x-100">{text}</span>
</span>
