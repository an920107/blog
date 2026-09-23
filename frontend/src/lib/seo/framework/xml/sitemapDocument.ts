import { SitemapStream, streamToPromise } from 'sitemap';

import type { PostPage } from '$lib/seo/domain/entity/postPage';
import { listStaticSitePageUrls } from '$lib/seo/framework/site/staticSitePages';

/**
 * Builds sitemap.xml from the published posts plus the static pages. Server-only: it pulls
 * in a Node stream implementation, so it must never be imported from client code.
 */
export async function buildSitemapDocument(postPages: readonly PostPage[]): Promise<string> {
	const stream = new SitemapStream();

	for (const url of listStaticSitePageUrls()) {
		stream.write({ url });
	}

	for (const page of postPages) {
		stream.write({ url: page.url.href, lastmod: page.lastModified.toISOString() });
	}

	stream.end();

	const sitemap = await streamToPromise(stream);
	return sitemap.toString();
}
