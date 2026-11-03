<script>
	import MottoAnimatedMark from '$lib/home/framework/ui/MottoAnimatedMark.svelte';
	import { Strings } from '$lib/strings';
</script>

<div
	class="content-container flex h-screen flex-col items-center justify-center gap-y-2.5 md:gap-y-8"
>
	<div
		class="flex w-[19rem] flex-col gap-y-3 text-3xl font-bold text-gray-800 md:w-[38rem] md:gap-y-4 md:text-6xl"
	>
		<div class="flex w-full flex-row items-center justify-start gap-x-2.5">
			<span>{Strings.MOTTO_KEEP}</span>
			<MottoAnimatedMark text={Strings.MOTTO_LEARNING} direction="left" />
		</div>
		<div class="flex w-full flex-row items-center justify-end gap-x-2.5 md:gap-x-4">
			<MottoAnimatedMark text={Strings.MOTTO_KEEP} direction="right" />
			<span>{Strings.MOTTO_PROGRESSING}</span>
		</div>
	</div>
</div>
