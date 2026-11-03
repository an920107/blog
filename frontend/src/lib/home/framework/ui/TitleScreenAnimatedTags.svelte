<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	import { Strings } from '$lib/strings';

	const tagsCollection = Strings.HOME_TAGS;

	// Initialize with placeholder to prevent flickering
	let showingTags: string[] = $state(['']);
	let isTagsVisible: boolean = $state(false);

	let interval: ReturnType<typeof setInterval> | null = null;

	onMount(() => {
		shuffleTags();
		isTagsVisible = true;

		interval = setInterval(() => {
			isTagsVisible = false;
			setTimeout(() => {
				shuffleTags();
				isTagsVisible = true;
			}, 500);
		}, 4000);
	});

	onDestroy(() => {
		if (interval != null) {
			clearInterval(interval);
			interval = null;
		}
	});

	function shuffleTags() {
		showingTags = [...tagsCollection].sort(() => Math.random() - 0.5);
	}
</script>

<div
	class="relative w-full max-w-3xl transition-opacity duration-500
		{isTagsVisible ? 'opacity-100' : 'opacity-0'}"
>
	<div
		class="absolute inset-0 bg-linear-to-r from-transparent via-transparent via-60% to-white"
	></div>
	<div class="flex flex-row items-center gap-x-2 overflow-hidden">
		{#each showingTags as tag (tag)}
			<span class="text-nowrap text-gray-400"># {tag}</span>
		{/each}
	</div>
</div>
