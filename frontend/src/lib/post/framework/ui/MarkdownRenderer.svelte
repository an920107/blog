<script module lang="ts">
	export interface HeadingItem {
		id: string;
		text: string;
		level: 2 | 3 | 4 | 5 | 6;
	}
</script>

<script lang="ts">
	import hljs from 'highlight.js';
	import markdownit from 'markdown-it';
	import markdownitAttrs from 'markdown-it-attrs';
	import type { Attachment } from 'svelte/attachments';

	import SafeHtml from '$lib/common/framework/ui/SafeHtml.svelte';

	const {
		content,
		onHeadingUpdate,
	}: {
		content: string;
		onHeadingUpdate?: (headings: HeadingItem[]) => void;
	} = $props();

	const md = markdownit({
		highlight: (str, lang) => {
			if (!lang || !hljs.getLanguage(lang)) {
				const escapedCode: string = md.utils.escapeHtml(str);
				return `<pre class="hljs"><code>${escapedCode}</code></pre>`;
			}

			const highlightedCode = hljs.highlight(str, {
				language: lang,
				ignoreIllegals: true,
			}).value;
			return `<pre class="hljs"><code>${highlightedCode}</code></pre>`;
		},
	});
	md.use(markdownitAttrs);
	const parsedContent = $derived(md.render(content));

	/** Generates a URL-safe, human-readable anchor slug (falls back to `section`). */
	function slugify(text: string): string {
		return (
			text
				.trim()
				.toLowerCase()
				.replace(/[^\p{L}\p{N}]+/gu, '-')
				.replace(/^-+|-+$/g, '') || 'section'
		);
	}

	const attachment: Attachment = (element) => {
		const headings: HeadingItem[] = [];
		const slugCounts: Record<string, number> = {};

		element.querySelectorAll('h2, h3, h4, h5, h6').forEach((h) => {
			const level = parseInt(h.tagName.charAt(1)) as 2 | 3 | 4 | 5 | 6;
			const slug = slugify(h.textContent || '');
			const count = slugCounts[slug] ?? 0;
			slugCounts[slug] = count + 1;
			const id = count === 0 ? slug : `${slug}-${count}`;

			h.id = id;
			headings.push({ id, text: h.textContent || '', level });
		});

		onHeadingUpdate?.(headings);
	};
</script>

<div {@attach attachment} class="prose max-w-none prose-gray">
	<SafeHtml html={parsedContent} />
</div>
