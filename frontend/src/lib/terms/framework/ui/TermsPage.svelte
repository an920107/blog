<script lang="ts">
	import MarkdownRenderer from '$lib/post/framework/ui/MarkdownRenderer.svelte';
	import generateTitle from '$lib/shared/framework/ui/generateTitle';

	const { title, content }: { title?: string; content: string } = $props();
</script>

<svelte:head>
	<title>{generateTitle(title)}</title>
</svelte:head>
<section class="content-container py-16">
	<div class="w-full max-w-3xl">
		<MarkdownRenderer {content} />
	</div>
</section>
