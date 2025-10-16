<script lang="ts">
	import { Button } from '$lib/common/framework/components/ui/button';
	import { onDestroy, onMount } from 'svelte';

	const {
		for: htmlFor,
		defaultValue,
		postAction,
	}: {
		for: string;
		defaultValue: string;
		postAction?: () => void;
	} = $props();

	let hidden = $state(true);

	function onClick() {
		const input = document.getElementById(htmlFor) as HTMLInputElement | null;
		if (input) {
			input.value = defaultValue;
			input.dispatchEvent(new Event('input'));
			input.dispatchEvent(new Event('change'));
		}
		postAction?.();
	}

	onMount(() => {
		const input = document.getElementById(htmlFor) as HTMLInputElement | null;

		function handleInput() {
			hidden = input?.value === defaultValue;
		}

		handleInput();
		input?.addEventListener('input', handleInput);

		onDestroy(() => {
			input?.removeEventListener('input', handleInput);
		});
	});
</script>

<Button variant="ghost" size="icon-sm" {hidden} onclick={onClick}>
	<i class="fa-solid fa-rotate-left"></i>
</Button>
