<script lang="ts">
	/* eslint-disable svelte/no-at-html-tags */

	import sanitizeHtml from 'sanitize-html';

	const { html }: { html: string } = $props();

	const sanitizedHtml = $derived(
		sanitizeHtml(html, { allowedTags: sanitizeHtml.defaults.allowedTags.concat(['img']) })
	);
</script>

{@html sanitizedHtml}
