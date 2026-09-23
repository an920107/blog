<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import OpenGraph from '$lib/common/framework/ui/OpenGraph.svelte';
	import StructuredData from '$lib/common/framework/ui/StructuredData.svelte';
	import { Environment } from '$lib/environment';
	import Motto from '$lib/home/framework/ui/Motto.svelte';
	import Terminal from '$lib/home/framework/ui/Terminal.svelte';
	import TitleScreen from '$lib/home/framework/ui/TitleScreen.svelte';
	import { Strings } from '$lib/strings';
</script>

<svelte:head>
	<title>{generateTitle()}</title>
	<meta name="description" content={Strings.SELF_INTRODUCTION_LINES.join('，')} />
</svelte:head>

<StructuredData
	props={{
		type: 'Person',
	}}
/>

<OpenGraph
	title="首頁"
	description={Strings.SELF_INTRODUCTION_LINES.join('，')}
	url={new URL('', Environment.APP_BASE_URL)}
	image={new URL('favicon.svg', Environment.APP_BASE_URL)}
	labels={[]}
/>

<div>
	<TitleScreen />
	<Terminal />
	<Motto />
</div>
