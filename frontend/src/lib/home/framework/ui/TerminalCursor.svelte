<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	let isVisible: boolean = $state(true);

	let interval: ReturnType<typeof setInterval> | null = null;

	onMount(() => {
		interval = setInterval(() => {
			toggleVisibility();
			setTimeout(toggleVisibility, 500);
		}, 1000);
	});

	onDestroy(() => {
		if (interval != null) {
			clearInterval(interval);
			interval = null;
		}
	});

	function toggleVisibility() {
		isVisible = !isVisible;
	}
</script>

<span class="transition-opacity duration-200 {isVisible ? 'opacity-100' : 'opacity-0'}">█</span>
