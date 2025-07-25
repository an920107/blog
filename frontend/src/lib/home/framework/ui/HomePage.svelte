<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import Motto from '$lib/home/framework/ui/Motto.svelte';
	import Terminal from '$lib/home/framework/ui/Terminal.svelte';
	import TitleScreen from '$lib/home/framework/ui/TitleScreen.svelte';
</script>

<svelte:head>
	<title>{generateTitle()}</title>
</svelte:head>
<div>
	<TitleScreen />
	<Terminal />
	<Motto />
</div>
