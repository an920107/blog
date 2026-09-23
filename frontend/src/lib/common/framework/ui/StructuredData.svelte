<script module lang="ts">
	export interface StructuredDataArticleProps {
		type: 'BlogPosting';
		url: URL;
		headline: string;
		name: string;
		description: string;
		datePublished: Date;
		image: URL | string | null;
		articleSection: string[];
	}

	export interface StructuredDataCollectionPageProps {
		type: 'CollectionPage';
		url: URL;
		name: string;
		description: string;
		mainEntity: {
			numberOfItems: number;
			itemListElement: Array<StructuredDataArticleProps>;
		};
	}

	export interface StructuredDataPersonProps {
		type: 'Person';
	}
</script>

<script lang="ts">
	import { Environment } from '$lib/environment';
	import { Strings } from '$lib/strings';

	/* eslint-disable svelte/no-at-html-tags */

	const {
		props,
	}: {
		props:
			StructuredDataArticleProps | StructuredDataCollectionPageProps | StructuredDataPersonProps;
	} = $props();

	const context = 'https://schema.org';

	function mapBlogPostingPropsToStructuredData(
		props: StructuredDataArticleProps,
		options: { withAuthor: boolean } = {
			withAuthor: true,
		}
	) {
		return {
			'@context': context,
			'@type': props.type,
			inLanguage: 'zh-TW',
			author: options.withAuthor
				? mapPersonPropsToStructuredData({
						type: 'Person',
					})
				: undefined,
			url: props.url.href,
			headline: props.headline,
			name: props.name,
			description: props.description,
			datePublished: props.datePublished.toISOString(),
			image: props.image instanceof URL ? props.image.href : props.image,
			articleSection: props.articleSection.length > 0 ? props.articleSection : 'Blog',
		};
	}

	function mapCollectionPagePropsToStructuredData(props: StructuredDataCollectionPageProps) {
		return {
			'@context': context,
			'@type': props.type,
			inLanguage: 'zh-TW',
			url: props.url.href,
			name: props.name,
			description: props.description,
			mainEntity: {
				'@type': 'ItemList',
				numberOfItems: props.mainEntity.numberOfItems,
				itemListElement: props.mainEntity.itemListElement.map((item, index) => ({
					...mapBlogPostingPropsToStructuredData(item, { withAuthor: false }),
					position: index + 1,
				})),
			},
		};
	}

	function mapPersonPropsToStructuredData(props: StructuredDataPersonProps) {
		return {
			'@context': context,
			'@type': props.type,
			name: 'Tsung-Ying, Yu',
			alternateName: 'Squid',
			url: Environment.APP_BASE_URL,
			image: new URL('favicon.svg', Environment.APP_BASE_URL).href,
			jobTitle: 'Software Engineer',
			description: Strings.SELF_INTRODUCTION_LINES.join('，'),
			alumniOf: {
				'@type': 'EducationalOrganization',
				name: 'National Central University',
			},
			sameAs: [
				'https://github.com/an920107',
				'https://www.youtube.com/@squidspirit16',
				'https://git.squidspirit.com/squid',
			],
		};
	}

	const structuredData = $derived.by(() => {
		switch (props.type) {
			case 'BlogPosting':
				return mapBlogPostingPropsToStructuredData(props);
			case 'CollectionPage':
				return mapCollectionPagePropsToStructuredData(props);
			case 'Person':
				return mapPersonPropsToStructuredData(props);
		}
	});

	const jsonLdScript = $derived(
		`<script type="application/ld+json">${JSON.stringify(structuredData)}${'<'}/script>`
	);
</script>

<svelte:head>
	{@html jsonLdScript}
</svelte:head>
