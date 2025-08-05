<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import Motto from '$lib/home/framework/ui/Motto.svelte';
	import Terminal from '$lib/home/framework/ui/Terminal.svelte';
	import TitleScreen from '$lib/home/framework/ui/TitleScreen.svelte';
</script>

<svelte:head>
	<title>{generateTitle('程式、科技、教學、分享')}</title>
	<!-- The meta description is set in `Terminal` -->
</svelte:head>
<div>
	<TitleScreen />
	<Terminal />
	<Motto />
</div>
