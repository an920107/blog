<script lang="ts">
	import SafeHtml from '$lib/common/framework/ui/SafeHtml.svelte';
	import markdownit from 'markdown-it';

	const { content }: { content: string } = $props();

	const md = markdownit();
	const parsedContent = $derived(md.render(content));
</script>

<div class="prose max-w-none prose-gray">
	<SafeHtml html={parsedContent} />
</div>
