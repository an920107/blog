import { resolve } from '$app/paths';
import { Environment } from '$lib/environment';
import { TERMS_DOCUMENTS } from '$lib/terms/framework/termsDocuments';

/**
 * The public, non-post pages of the site. They are only relevant to the sitemap (never to
 * the feed), and only the frontend knows these routes, so the list lives here rather than
 * in the domain.
 */
export function listStaticSitePageUrls(): readonly string[] {
	const paths = [
		resolve('/'),
		resolve('/post'),
		...TERMS_DOCUMENTS.map((document) => resolve('/terms/[name]', { name: document.pathname })),
	];

	return paths.map((path) => new URL(path, Environment.APP_BASE_URL).href);
}
