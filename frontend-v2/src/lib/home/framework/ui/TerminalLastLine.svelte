<script lang="ts">
	import TerminalCursor from '$lib/home/framework/ui/TerminalCursor.svelte';
	import { onDestroy, onMount } from 'svelte';

	let { text, onComplete: onCompleted }: { text: string; onComplete: () => void } = $props();

	let timeText: string = $state('');
	let showingText: string = $state('');

	let textUpdateInterval: ReturnType<typeof setInterval> | null = null;
	let timeUpdateInterval: ReturnType<typeof setInterval> | null = null;

	onMount(() => {
		setTimeout(() => {
			textUpdateInterval = setInterval(() => {
				if (showingText.length < text.length) {
					showingText += text[showingText.length];
				} else {
					clearInterval(textUpdateInterval!);
					setTimeout(onCompleted, 300);
				}
			}, 50);
		}, 300);

		timeText = dateToLocaleString(new Date());
		timeUpdateInterval = setInterval(() => {
			timeText = dateToLocaleString(new Date());
		}, 1000);
	});

	onDestroy(() => {
		if (textUpdateInterval != null) {
			clearInterval(textUpdateInterval);
			textUpdateInterval = null;
		}
		if (timeUpdateInterval != null) {
			clearInterval(timeUpdateInterval);
			timeUpdateInterval = null;
		}
	});

	function dateToLocaleString(date: Date): string {
		return date.toLocaleString('en-US', {
			hour12: false,
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});
	}
</script>

<div class="flex w-full flex-col pt-1.5 leading-5 md:pt-2.5 md:leading-7">
	<div class="flex w-full flex-row flex-nowrap items-center gap-x-1.5 text-nowrap md:gap-x-2">
		<span>
			╭─  squid{' '}
			<span class="text-blue-500">
				~<span class="max-md:hidden">/Documents/blog</span>
			</span>
		</span>
		<div class="h-0.5 w-full bg-gray-50"></div>
		<span> {timeText}</span>
	</div>
	<div class="flex w-full flex-row gap-x-1.5 md:gap-x-2">
		<span>
			╰─<span class="text-green-400">❯</span>
		</span>
		<span>{showingText}</span>
		<TerminalCursor />
	</div>
</div>
