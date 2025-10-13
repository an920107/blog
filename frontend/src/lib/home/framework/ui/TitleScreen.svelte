<script lang="ts">
	import TitleScreenAnimatedTags from '$lib/home/framework/ui/TitleScreenAnimatedTags.svelte';
</script>

<div
	class="content-container flex min-h-content-height flex-col justify-center gap-y-2.5 md:gap-y-8"
>
	<h2 class="text-3xl font-bold text-gray-800 md:text-6xl">Hello 大家好！</h2>
	<h1 class="flex flex-row items-center gap-x-2 text-4xl font-extrabold text-gray-800 md:text-7xl">
		<span>我是</span>
		<div class="rounded-lg bg-blue-600 px-1.5 py-1">
			<span class="text-white">Squid</span>
		</div>
		<span>魷魚</span>
	</h1>
	<TitleScreenAnimatedTags />
</div>
