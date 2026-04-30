<script lang="ts">
	import generateTitle from '$lib/common/framework/ui/generateTitle';
	import MarkdownRenderer from '$lib/post/framework/ui/MarkdownRenderer.svelte';

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
