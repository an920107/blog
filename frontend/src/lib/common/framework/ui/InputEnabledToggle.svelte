<script lang="ts">
	import Switch from '$lib/common/framework/components/ui/switch/switch.svelte';
	import { onDestroy, onMount } from 'svelte';

	const {
		for: htmlFor,
		defaultValue,
		valueOnEnable,
	}: {
		for: string;
		defaultValue: string;
		valueOnEnable: string;
	} = $props();

	let enabled = $state(defaultValue.length > 0);
	let lastValue = defaultValue;

	$effect(() => {
		const input = document.getElementById(htmlFor) as HTMLInputElement | null;
		if (!input) {
			return;
		}

		if (enabled) {
			input.removeAttribute('hidden');
			input.value ||= lastValue || valueOnEnable;
		} else {
			input.setAttribute('hidden', 'true');
			lastValue = input.value;
			input.value = '';
		}
		input.dispatchEvent(new Event('input'));
		input.dispatchEvent(new Event('change'));
	});

	onMount(() => {
		const input = document.getElementById(htmlFor) as HTMLInputElement | null;
		if (!input) {
			return;
		}

		const onChange = () => (enabled = input.value.length > 0);

		input.addEventListener('change', onChange);

		onDestroy(() => {
			input.removeEventListener('change', onChange);
		});
	});
</script>

<div class="h-9 content-center">
	<Switch bind:checked={enabled} />
</div>
