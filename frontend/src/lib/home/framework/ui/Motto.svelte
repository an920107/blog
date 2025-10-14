<script>
	import MottoAnimatedMark from '$lib/home/framework/ui/MottoAnimatedMark.svelte';
</script>

<div
	class="content-container flex h-screen flex-col items-center justify-center gap-y-2.5 md:gap-y-8"
>
	<div
		class="flex w-[19rem] flex-col gap-y-3 text-3xl font-bold text-gray-800 md:w-[38rem] md:gap-y-4 md:text-6xl"
	>
		<div class="flex w-full flex-row items-center justify-start gap-x-2.5">
			<span>Keep</span>
			<MottoAnimatedMark text="Learning" direction="left" />
		</div>
		<div class="flex w-full flex-row items-center justify-end gap-x-2.5 md:gap-x-4">
			<MottoAnimatedMark text="Keep" direction="right" />
			<span>Progressing</span>
		</div>
	</div>
</div>
