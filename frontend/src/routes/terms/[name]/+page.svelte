<script lang="ts">
	import MarkdownRenderer from '$lib/post/framework/ui/MarkdownRenderer.svelte';
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
</script>

<section class="content-container py-16">
	<div class="w-full max-w-3xl">
		<MarkdownRenderer content={data.content} />
	</div>
</section>
