<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	const tagsCollection = [
		'APP',
		'C++',
		'Clean Architecture',
		'Design Pattern',
		'Docker',
		'Flutter',
		'Go',
		'Java',
		'LINER',
		'Linux',
		'Python',
		'React',
		'Rust',
		'Squid',
		'Svelte',
		'TypeScript',
		'中央大學',
		'全端',
		'分享',
		'前端',
		'後端',
		'教學',
		'知識',
		'科技',
		'科普',
		'程式設計',
		'資工系',
		'軟體工程',
		'遊戲',
		'魷魚',
	];

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
	class="relative w-full max-w-screen-md transition-opacity duration-500
		{isTagsVisible ? 'opacity-100' : 'opacity-0'}"
>
	<div
		class="absolute inset-0 bg-gradient-to-r from-transparent via-transparent via-60% to-white"
	></div>
	<div class="flex flex-row items-center gap-x-2 overflow-hidden">
		{#each showingTags as tag (tag)}
			<span class="text-nowrap text-gray-400"># {tag}</span>
		{/each}
	</div>
</div>
