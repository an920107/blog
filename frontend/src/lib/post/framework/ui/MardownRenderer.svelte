<script module lang="ts">
	export interface HeadingItem {
		id: string;
		text: string;
		level: 2 | 3 | 4 | 5 | 6;
	}
</script>

<script lang="ts">
	import SafeHtml from '$lib/common/framework/ui/SafeHtml.svelte';
	import markdownit from 'markdown-it';
	import CryptoJS from 'crypto-js';
	import type { Attachment } from 'svelte/attachments';

	const {
		content,
		onHeadingUpdate,
	}: {
		content: string;
		onHeadingUpdate?: (headings: HeadingItem[]) => void;
	} = $props();

	const md = markdownit();
	const parsedContent = $derived(md.render(content));

	const attachment: Attachment = (element) => {
		const headings: HeadingItem[] = [];
		const idSet = new Set<string>();

		element.querySelectorAll('h2, h3, h4, h5, h6').forEach((h) => {
			const level = parseInt(h.tagName.charAt(1)) as 2 | 3 | 4 | 5 | 6;
			let id = CryptoJS.MD5(h.textContent || '').toString();

			while (idSet.has(id)) {
				id = CryptoJS.MD5(id).toString();
			}

			h.id = id;
			headings.push({ id, text: h.textContent || '', level });
		});

		onHeadingUpdate?.(headings);
	};
</script>

<div {@attach attachment} class="prose max-w-none prose-gray">
	<SafeHtml html={parsedContent} />
</div>
