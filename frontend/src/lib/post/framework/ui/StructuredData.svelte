<script lang="ts">
	/* eslint-disable svelte/no-at-html-tags */

	const {
		headline,
		description,
		datePublished,
		image,
	}: {
		headline: string;
		description: string;
		datePublished: Date;
		image: URL;
	} = $props();

	const structuredData = $derived({
		'@context': 'https://schema.org',
		'@type': 'BlogPosting',
		headline: headline,
		description: description,
		datePublished: datePublished.toISOString(),
		image: image.href,
	});

	const jsonLdScript = $derived(
		`<script type="application/ld+json">${JSON.stringify(structuredData)}${'<'}/script>`
	);
</script>

{@html jsonLdScript}
