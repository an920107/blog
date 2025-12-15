<script lang="ts">
	import { cn } from '$lib/common/framework/components/utils';
	import { onDestroy, onMount } from 'svelte';

	const { showRanbowRing }: { showRanbowRing: boolean } = $props();

	let buttonBottom = $state('1rem');

	onMount(() => {
		const updateButtonPosition = () => {
			const footer = document.querySelector('footer');
			if (!footer) {
				buttonBottom = '1rem';
				return;
			}

			const footerRect = footer.getBoundingClientRect();
			const viewportHeight = window.innerHeight;

			// If the footer is in view, adjust the button position
			if (footerRect.top < viewportHeight) {
				const overlap = viewportHeight - footerRect.top;
				const baseBottom = window.innerWidth >= 768 ? 40 : 16; // md:bottom-10 or bottom-4
				buttonBottom = `${overlap + baseBottom}px`;
			} else {
				buttonBottom = window.innerWidth >= 768 ? '2.5rem' : '1rem';
			}
		};

		updateButtonPosition();
		window.addEventListener('scroll', updateButtonPosition);
		window.addEventListener('resize', updateButtonPosition);

		onDestroy(() => {
			window.removeEventListener('scroll', updateButtonPosition);
			window.removeEventListener('resize', updateButtonPosition);
		});
	});
</script>

<div
	class="group fixed right-4 z-10 transition-all duration-300 md:right-10"
	style="bottom: {buttonBottom};"
>
	{#if showRanbowRing}
		<div class="absolute -inset-1 -z-10 overflow-hidden rounded-full blur-md">
			<div
				class={cn(
					'ranbow-ring',
					'absolute h-[180%] w-[180%]',
					'animate-spin bg-[conic-gradient(from_0deg,transparent_0_340deg,white_360deg)]'
				)}
			></div>
		</div>
	{/if}

	<button
		class={cn(
			'relative size-12 content-center rounded-full text-center',
			'border border-gray-200 bg-white/80 shadow-sm backdrop-blur-md'
		)}
		aria-label="Search & Filter"
	>
		<i class="fa-solid fa-magnifying-glass size-8 content-center text-gray-500"></i>
	</button>
</div>

<style>
	.ranbow-ring {
		animation-duration: 8s;
		top: -50%;
		left: -50%;
		background: conic-gradient(
			from 90deg at 50% 50%,
			#ff0000 0%,
			#ffff00 17%,
			#00ff00 33%,
			#00ffff 50%,
			#0000ff 67%,
			#ff00ff 83%,
			#ff0000 100%
		);
	}
</style>
