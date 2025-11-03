<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import HomePage from '$lib/home/framework/ui/HomePage.svelte';
	import { lines as terminalLines } from '$lib/home/framework/ui/Terminal.svelte';
</script>

<svelte:head>
	<title>{generateTitle()}</title>
	<meta name="description" content={terminalLines.join('，')} />
</svelte:head>

<HomePage />
