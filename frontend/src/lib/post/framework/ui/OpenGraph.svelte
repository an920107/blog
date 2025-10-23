<script lang="ts">
	import { Strings } from '$lib/strings';

	const {
		title,
		description,
		publishedTime,
		labels,
		url,
		image,
	}: {
		title: string;
		description: string;
		publishedTime: Date;
		labels: string[];
		url: URL;
		image: URL | null;
	} = $props();

	const section = $derived(labels.length > 0 ? labels[0] : 'Blog');
</script>

<svelte:head>
	<meta property="og:site_name" content={Strings.APP_NAME} />
	<meta property="og:type" content="article" />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={url.href} />
	<meta property="og:article:published_time" content={publishedTime.toISOString()} />
	{#if section}
		<meta property="og:article:section" content={section} />
	{/if}
	{#each labels as label (label)}
		<meta property="og:article:tag" content={label} />
	{/each}
	{#if image}
		<meta property="og:image" content={image.href} />
	{/if}
</svelte:head>
